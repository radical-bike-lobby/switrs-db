//! Schema operations for the SWITRS sqlite DB creation

use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use new_string_template::template::Template;
use regex::Regex;
use rusqlite::{params_from_iter, Connection};
use serde::Deserialize;

/// Specifies which schema and data should be used for creating a table
#[derive(Debug, Deserialize)]
pub struct LookupTable {
    pk_type: String,
    data: PathBuf,
    schema: Option<PathBuf>,
}

/// Path to the data to load into the table
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "path", rename_all = "snake_case")]
pub enum DataPath {
    /// The file name (relative to where the raw data was extracted) of the csv data
    RawData(PathBuf),
    /// Path relative to the application
    Path(PathBuf),
    /// Create the table as empty
    Empty,
}

/// Primary Table definition as defined in the Toml
#[derive(Debug, Deserialize)]
pub struct PrimaryTable {
    /// Path to the schema file for the table, like collisions.sql
    schema: PathBuf,

    /// Path to the data to load into the table
    #[serde(flatten)]
    data: DataPath,
}

/// Schema defenition as loaded from the Toml
#[derive(Debug, Deserialize)]
pub struct Schema {
    #[serde(alias = "table-order")]
    table_order: Vec<String>,
    tables: HashMap<String, PrimaryTable>,
    #[serde(alias = "lookup-schema")]
    lookup_schema: PathBuf,
    #[serde(alias = "lookup-tables")]
    lookup_tables: HashMap<String, LookupTable>,
}

impl Schema {
    /// Loads the Schema definition from the Toml at the given path
    pub fn from_toml_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let schema = basic_toml::from_slice(&fs::read(path)?)?;

        Ok(schema)
    }
}

/// Extensions to the DB Connection to initialize the DB
pub trait NewDB {
    /// Get access to the DB connection (generally will be Self)
    fn connection(&self) -> &Connection;

    /// Create a table where the name and pk_type are passed into the sql as template parameters
    fn create_table(
        &self,
        name: &str,
        pk_type: &str,
        table_schema: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // build the DDL expression
        let ddl = fs::read_to_string(table_schema).map_err(|e| {
            format!(
                "failed to read {table_schema}: {e}",
                table_schema = table_schema.display()
            )
        })?;
        let ddl = Template::new(ddl);
        let data = {
            let mut map = HashMap::new();
            map.insert("table", name);
            map.insert("pk_type", pk_type);
            map
        };

        let ddl = ddl.render(&data)?;
        self.connection().execute_batch(&ddl)?;
        Ok(())
    }

    /// Load data into the named table from the CSV file at the given table_data path
    fn load_data(
        &self,
        name: &str,
        table_data: &Path,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let fixup_roads = name == "collisions";

        // open the csv file
        let mut csv = csv::ReaderBuilder::new()
            .quoting(true)
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(table_data)
            .map_err(|e| {
                format!(
                    "failed to read csv {table_data}: {e}",
                    table_data = table_data.display()
                )
            })?;

        // build up the insert statement
        let mut field_count = 0;
        let headers_record;
        let mut case_id_idx = 0usize;
        let mut primary_rd_idx = 0usize;
        let mut secondary_rd_idx = 0usize;

        let (fields, values) = {
            // construct "field = "
            headers_record = csv.headers()?.clone();
            let mut fields = String::new();
            let mut values = String::new();
            let mut first = true;
            for (idx, f) in headers_record.into_iter().enumerate() {
                if !first {
                    fields.push_str(", ");
                    values.push_str(", ");
                } else {
                    first = false;
                }

                let f = f.to_lowercase();
                if f.to_lowercase() == "case_id" {
                    case_id_idx = idx;
                } else if f == "primary_rd" {
                    primary_rd_idx = idx;
                } else if f == "secondary_rd" {
                    secondary_rd_idx = idx;
                }

                fields.push_str(&f);
                values.push('?');
                field_count += 1;
            }

            (fields, values)
        };

        if field_count == 0 {
            return Ok(0);
        }

        let mut insert_stmt = self
            .connection()
            .prepare(&format!("INSERT INTO {name} ({fields}) VALUES({values})"))?;

        // when processing collision data, we will cleanup some data,
        //   for that we have some custom insert and one off tables
        let mut insert_road_stmt = if fixup_roads {
            Some(self.connection().prepare(
                "INSERT INTO normalized_roads (
                case_id,
                primary_rd,
                primary_rd_address,
                primary_rd_block,
                primary_rd_direction,
                secondary_rd,
                secondary_rd_address,
                secondary_rd_block,
                seconardy_rd_direction
            ) VALUES(
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            )",
            )?)
        } else {
            None
        };

        // collect all the data
        let mut count = 0;
        for record in csv.into_records() {
            let record = record?;

            // convert empty strings to NULL, should we change '-' to NULL as well?
            let record_iter = record
                .iter()
                .map(|s| if s.is_empty() { None } else { Some(s) });

            insert_stmt
                .insert(params_from_iter(record_iter))
                .inspect_err(|e| {
                    print!("error on insert into {name}: {e}, row {count}:");
                    for (field, value) in headers_record.iter().zip(record.iter()) {
                        print!("{field}={value},");
                    }
                    println!();
                })?;

            // add normalized roads from the collisions table
            if fixup_roads {
                let case_id = record.get(case_id_idx);
                let primary_rd = record.get(primary_rd_idx);
                let secondary_rd = record.get(secondary_rd_idx);

                let primary_rd = primary_rd.map(normalize_road);
                let secondary_rd = secondary_rd.map(normalize_road);

                if let Some(case_id) = case_id {
                    insert_road_stmt.as_mut().unwrap().insert([
                        Some(case_id), 
                        primary_rd.as_ref().map(|r| r.road.deref()),
                        primary_rd.as_ref().and_then(|r| r.address),
                        primary_rd.as_ref().and_then(|r| r.block),
                        primary_rd.as_ref().and_then(|r| r.direction),
                        secondary_rd.as_ref().map(|r| r.road.deref()),
                        secondary_rd.as_ref().and_then(|r| r.address),
                        secondary_rd.as_ref().and_then(|r| r.block),
                        secondary_rd.as_ref().and_then(|r| r.direction),
                    ])
                    .inspect_err(|e| {
                        println!("error on insert into normalized_road: {e}, row {count}: case_id={case_id},primary={primary_rd:?},secondary={secondary_rd:?}");
                    })?;
                }
            }

            count += 1;
        }

        Ok(count)
    }

    /// Initialize all the lookup tables in lookup_tables
    fn init_lookup_tables(
        &self,
        lookup_tables: &HashMap<String, LookupTable>,
        table_schema: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (name, table) in lookup_tables {
            println!("LOADING {name}");
            let schema = table.schema.as_deref().unwrap_or(table_schema);
            self.create_table(name, &table.pk_type, schema)?;
            self.load_data(name, &table.data)?;
        }

        Ok(())
    }

    /// Create and load all the tables defined in the Schema
    fn load_from_schema(
        &self,
        schemas: &Schema,
        data: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)?;

        for table_name in &schemas.table_order {
            let table: &PrimaryTable = schemas
                .tables
                .get(table_name)
                .ok_or_else(|| format!("table missing from [tables]: {table_name}"))?;

            let data = match &table.data {
                DataPath::RawData(path) => Some(data.join(path)),
                DataPath::Path(path) => Some(path.clone()),
                DataPath::Empty => None,
            };

            println!("LOADING {table_name}");
            self.connection()
                .create_table(table_name, "", &table.schema)?;

            if let Some(data) = data {
                self.connection().load_data(table_name, &data)?;
            }
        }

        Ok(())
    }
}

impl NewDB for Connection {
    fn connection(&self) -> &Self {
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
struct NormalizedRoad<'a> {
    road: Cow<'a, str>,
    address: Option<&'a str>,
    block: Option<&'a str>,
    direction: Option<&'a str>,
}

/// Takes Road names and removes address information, or block information
fn normalize_road(road: &str) -> NormalizedRoad<'_> {
    static ADDRESS_MATCHER: OnceLock<Regex> = OnceLock::new();
    static REPLACE_SPACES: OnceLock<Regex> = OnceLock::new();

    let address_matcher: &Regex = ADDRESS_MATCHER.get_or_init(|| {
        Regex::new(
            r"(^(?<address_pre>\d+) +)?(?<street>(I-\d+)|(RT +\d+)|(\w+[ \w]+[[:alpha:]]+))([\.,])*(/B)?( +(?<direction>NORTHBOUND|EASTBOUND|WESTBOUND|SOUTHBOUND|N/B|E/B|W/B|S/B|NB|EB|WB|SB|N|E|W|S)[\.,/]*)?( +((?<address_post>\d+)|(\(?(?<block>\d+) BLOCK\)?))$)?",
        )
        .expect("bad regular expression")
    });

    let replace_spaces: &Regex =
        REPLACE_SPACES.get_or_init(|| Regex::new(r"( +)").expect("bad regular expression"));

    let caps = address_matcher.captures(road);

    if let Some(caps) = caps {
        let road = caps.name("street").map(|m| m.as_str()).unwrap_or(road);
        let road = replace_spaces.replace_all(road, " ");

        NormalizedRoad {
            road,
            address: caps
                .name("address_pre")
                .or_else(|| caps.name("address_post"))
                .map(|s| s.as_str()),
            block: caps.name("block").map(|m| m.as_str()),
            direction: caps.name("direction").map(|m| m.as_str()),
        }
    } else {
        let road = replace_spaces.replace_all(road, " ");

        NormalizedRoad {
            road,
            address: None,
            block: None,
            direction: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml() {
        let schemas = Schema::from_toml_file(Path::new("Schemas.toml")).expect("toml is bad");

        assert_eq!(schemas.table_order[1], "collisions");
        assert_eq!(
            schemas.tables["collisions"].schema,
            Path::new("schema/collisions.sql")
        );
    }

    #[test]
    fn test_create_table_char_1() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");
        let table = LookupTable {
            pk_type: String::from("CHAR(1)"),
            data: PathBuf::from("lookup-tables/DAY_OF_WEEK.csv"),
            schema: None,
        };

        connection
            .connection()
            .create_table(
                "day_of_week",
                &table.pk_type,
                Path::new("schema/pk_table.sql"),
            )
            .expect("failed to create table");

        connection
            .execute("SELECT * from day_of_week", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("day_of_week", &table.data)
            .expect("failed to create table");

        assert_eq!(7, count);
    }

    #[test]
    fn test_create_table_char_2() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");
        let table = LookupTable {
            pk_type: String::from("CHAR(2)"),
            data: PathBuf::from("lookup-tables/PCF_VIOL_CATEGORY.csv"),
            schema: None,
        };

        connection
            .connection()
            .create_table(
                "pcf_violation_category",
                &table.pk_type,
                Path::new("schema/pk_table.sql"),
            )
            .expect("failed to create table");

        connection
            .execute("SELECT * from pcf_violation_category", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("pcf_violation_category", &table.data)
            .expect("failed to create table");

        assert_eq!(26, count);
    }

    #[test]
    fn test_create_table_varchar_2() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");
        let table = LookupTable {
            pk_type: String::from("VARCHAR2(2)"),
            data: PathBuf::from("lookup-tables/PRIMARY_RAMP.csv"),
            schema: None,
        };

        connection
            .connection()
            .create_table(
                "primary_ramp",
                &table.pk_type,
                Path::new("schema/pk_table.sql"),
            )
            .expect("failed to create table");

        connection
            .execute("SELECT * from primary_ramp", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("primary_ramp", &table.data)
            .expect("failed to create table");

        assert_eq!(12, count);
    }

    #[test]
    fn test_create_collisions() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");

        // initialize all the lookup tables
        let schemas = Schema::from_toml_file(Path::new("Schemas.toml")).expect("toml is bad");
        connection
            .connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)
            .expect("failed to init lookup tables");

        // create the normalized_roads schema
        connection
            .connection()
            .create_table(
                "normalized_roads",
                "",
                Path::new("schema/normalized_roads.sql"),
            )
            .expect("failed to create table");

        connection
            .connection()
            .create_table("collisions", "", Path::new("schema/collisions.sql"))
            .expect("failed to create table");

        connection
            .execute("SELECT * from collisions", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("collisions", Path::new("tests/data/collisions.csv"))
            .expect("failed to create table");

        assert_eq!(40, count);
    }

    #[test]
    fn test_create_parties() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");

        // initialize all the lookup tables
        let schemas = Schema::from_toml_file(Path::new("Schemas.toml")).expect("toml is bad");
        connection
            .connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)
            .expect("failed to init lookup tables");

        // create the normalized_roads schema
        connection
            .connection()
            .create_table(
                "normalized_roads",
                "",
                Path::new("schema/normalized_roads.sql"),
            )
            .expect("failed to create table");

        // load test data into the collisions table
        connection
            .connection()
            .create_table("collisions", "", Path::new("schema/collisions.sql"))
            .expect("failed to create table");
        connection
            .connection()
            .load_data("collisions", Path::new("tests/data/collisions.csv"))
            .expect("failed to create table");

        // parties
        connection
            .connection()
            .create_table("parties", "", Path::new("schema/parties.sql"))
            .expect("failed to create table");

        connection
            .execute("SELECT * from parties", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("parties", Path::new("tests/data/parties.csv"))
            .expect("failed to create table");

        assert_eq!(80, count);
    }

    #[test]
    fn test_create_victims() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");

        // initialize all the lookup tables
        let schemas = Schema::from_toml_file(Path::new("Schemas.toml")).expect("toml is bad");
        connection
            .connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)
            .expect("failed to init lookup tables");

        // create the normalized_roads schema
        connection
            .connection()
            .create_table(
                "normalized_roads",
                "",
                Path::new("schema/normalized_roads.sql"),
            )
            .expect("failed to create table");

        // load test data into the collisions table
        connection
            .connection()
            .create_table("collisions", "", Path::new("schema/collisions.sql"))
            .expect("failed to create table");
        connection
            .connection()
            .load_data("collisions", Path::new("tests/data/collisions.csv"))
            .expect("failed to create table");

        // load parties
        connection
            .connection()
            .create_table("parties", "", Path::new("schema/parties.sql"))
            .expect("failed to create table");
        connection
            .connection()
            .load_data("parties", Path::new("tests/data/parties.csv"))
            .expect("failed to create table");

        // test victims
        connection
            .connection()
            .create_table("victims", "", Path::new("schema/victims.sql"))
            .expect("failed to create table");

        connection
            .execute("SELECT * from victims", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("victims", Path::new("tests/data/victims.csv"))
            .expect("failed to create table");

        assert_eq!(39, count);
    }

    #[test]
    fn test_normalize_road() {
        let test = |raw, road, address, block, direction| {
            assert_eq!(
                NormalizedRoad {
                    road: Cow::Borrowed(road),
                    address,
                    block,
                    direction,
                },
                normalize_road(raw)
            )
        };

        test("GRANT", "GRANT", None, None, None);
        test("1201 2ND ST", "2ND ST", Some("1201"), None, None);
        test("WARD 1403", "WARD", Some("1403"), None, None);
        test("RT 123", "RT 123", None, None, None);
        test("RT 13", "RT 13", None, None, None);
        test("RT 80 E", "RT 80", None, None, Some("E"));
        test("RT1805", "RT1805", None, None, None);
        test("SAN PABLO 1229", "SAN PABLO", Some("1229"), None, None);
        test("6TH  ST", "6TH ST", None, None, None);
        test("7 GAUSS WAY", "GAUSS WAY", Some("7"), None, None);
        test("ASHBY AVE.", "ASHBY AVE", None, None, None);
        test(
            "EUCLID AVE (600 BLOCK)",
            "EUCLID AVE",
            None,
            Some("600"),
            None,
        );
        test("RT 80 E/B", "RT 80", None, None, Some("E/B"));
        test("I-80 WB TO UNIVERSITY AVE", "I-80", None, None, Some("WB"));
        test("I-80 E/B TO I-580 W/B", "I-80", None, None, Some("E/B"));
        test(
            "1313 NINTH STREET (PARKING LOT)",
            "NINTH STREET",
            Some("1313"),
            None,
            None,
        );
        test(
            "85 EL CAMINO REAL RD",
            "EL CAMINO REAL RD",
            Some("85"),
            None,
            None,
        );
        test(
            "8TH STREET, 1400 BLOCK",
            "8TH STREET",
            None,
            Some("1400"),
            None,
        );
        test(
            "ADDISON ST. WESTBOUND, 1500 BLOCK",
            "ADDISON ST",
            None,
            Some("1500"),
            Some("WESTBOUND"),
        );
        test(
            "CEDAR ST. (2200 BLOCK)",
            "CEDAR ST",
            None,
            Some("2200"),
            None,
        );
        test(
            "CEDAR STREET, 1800 BLOCK",
            "CEDAR STREET",
            None,
            Some("1800"),
            None,
        );
        test(
            "CHANNING WAY E/B  800 BLOCK",
            "CHANNING WAY E",
            None,
            Some("800"),
            None,
        );
        test(
            "CAMPUS DR, 1400 BLOCK",
            "CAMPUS DR",
            None,
            Some("1400"),
            None,
        );
        test(
            "CAMPUS DR., 1400 BLOCK",
            "CAMPUS DR",
            None,
            Some("1400"),
            None,
        );
    }
}
