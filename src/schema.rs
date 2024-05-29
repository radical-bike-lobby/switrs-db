//! Schema operations for the SWITRS sqlite DB creation

use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    io::Write,
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
        self.load_data_with_options(name, table_data, false, false)
    }

    /// Load data into the named table from the CSV file at the given table_data path
    fn load_data_with_options(
        &self,
        name: &str,
        table_data: &Path,
        allow_duplicates: bool,
        report_new_entries: bool,
    ) -> Result<usize, Box<dyn std::error::Error>> {
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

        let (fields, values) = {
            // construct "field = "
            headers_record = csv.headers()?.clone();
            let mut fields = String::new();
            let mut values = String::new();
            let mut first = true;
            for f in headers_record.into_iter() {
                if !first {
                    fields.push_str(", ");
                    values.push_str(", ");
                } else {
                    first = false;
                }

                fields.push_str(f);
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
                .inspect(|count| {
                    if report_new_entries && *count > 0 {
                        print!("    INSERTED ");
                        for (field, value) in headers_record.iter().zip(record.iter()) {
                            print!("{field}={value},");
                        }
                        println!();
                    }
                })
                .or_else(|result| {
                    // if we're allowing dups, ignore the error
                    //  TODO: this should probably check for the correct error
                    if allow_duplicates {
                        Ok(0)
                    } else {
                        Err(result)
                    }
                })
                .inspect_err(|e| {
                    print!("error on insert into {name}: {e}, row {count}:");
                    for (field, value) in headers_record.iter().zip(record.iter()) {
                        print!("{field}={value},");
                    }
                    println!();
                })?;

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
        // initialize lookup tables
        self.connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)?;

        // Build all the standard tables
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

        // build fixup tables
        self.fixup_tables()?;

        Ok(())
    }

    /// Run tasks to fill fixup tables, or produce csv's which add lookup tables to cleanup data
    fn fixup_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.fixup_roads()?;

        Ok(())
    }

    /// This uses the Berkeley Road Typos and the Corrected Roads to construct a lookup table with correct road names
    ///   for each Case ID
    fn fixup_roads(&self) -> Result<(), Box<dyn std::error::Error>> {
        // when processing collision data, we will cleanup some data,
        //   for that we have some custom insert and one off tables
        let mut insert_road_stmt = self.connection().prepare(
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
        )?;

        let mut select_roads = self
            .connection()
            .prepare("SELECT case_id, primary_rd, secondary_rd FROM collisions")?;

        let mut roads = select_roads.query([])?;
        while let Some(road) = roads.next()? {
            // add normalized roads from the collisions table
            let case_id = road.get_ref("case_id")?.as_str()?;
            let primary_rd = road.get_ref("primary_rd")?.as_str()?;
            let secondary_rd = road.get_ref("secondary_rd")?.as_str()?;

            let primary_rd = normalize_road(primary_rd);
            let secondary_rd = normalize_road(secondary_rd);

            insert_road_stmt.insert([
                Some(case_id),
                Some(&primary_rd.road),
                primary_rd.address,
                primary_rd.block,
                primary_rd.direction,
                Some(&secondary_rd.road),
                secondary_rd.address,
                secondary_rd.block,
                secondary_rd.direction,
            ])
            .inspect_err(|e| {
                println!("error on insert into normalized_roadcase_id={case_id},primary={primary_rd:?},secondary={secondary_rd:?}: {e}");
            })?;
        }

        //
        // find all roads not in our known roads list
        //   we will prefer names that match the "known list", the move to the normalized name matching, then typos
        let mut select_roads = self
            .connection()
            .prepare("
                SELECT 
                n.case_id as case_id,
                n.primary_rd as normal_primary_rd,
                n.primary_rd_address,
                n.primary_rd_block,
                n.primary_rd_direction,
                n.secondary_rd as normal_secondary_rd,
                n.secondary_rd_address,
                n.secondary_rd_block,
                n.seconardy_rd_direction,
                c.primary_rd as original_primary_rd,
                c.secondary_rd as original_secondary_rd,
                cr.primary_rd as correct_primary_rd,
                cr.secondary_rd as correct_secondary_rd,
                cp.primary_rd as verified_primary_rd,
                cs.secondary_rd as verified_secondary_rd,
                tp.correct_rd as suggest_primary_rd,
                ts.correct_rd as suggest_secondary_rd
                FROM 
                normalized_roads as n
                LEFT JOIN collisions_view as c ON c.case_id = n.case_id
                LEFT JOIN corrected_roads as cr ON cr.case_id = n.case_id
                LEFT JOIN collisions as cp ON cp.case_id = n.case_id AND cp.primary_rd in (SELECT DISTINCT correct_rd FROM berkeley_road_typos)
                LEFT JOIN collisions as cs ON cs.case_id = n.case_id AND cs.secondary_rd in (SELECT DISTINCT correct_rd FROM berkeley_road_typos)
                LEFT JOIN berkeley_road_typos as tp ON tp.normalized_rd = n.primary_rd
                LEFT JOIN berkeley_road_typos as ts ON ts.normalized_rd = n.secondary_rd
                ORDER BY case_id
            ")?;
        let mut corrections = select_roads.query([])?;

        // we will always rebuild the corrections file.
        let mut corrected_roads = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open("berkeley-tables/CORRECTED_ROADS.csv")?;
        writeln!(corrected_roads, "case_id,primary_rd,secondary_rd")?;
        while let Some(correction) = corrections.next()? {
            let case_id = correction.get_ref("case_id")?.as_str()?;
            let normal_primary_rd = correction.get_ref("normal_primary_rd")?.as_str()?;
            let normal_secondary_rd = correction.get_ref("normal_secondary_rd")?.as_str()?;
            let original_primary_rd = correction.get_ref("original_primary_rd")?.as_str()?;
            let original_secondary_rd = correction.get_ref("original_secondary_rd")?.as_str()?;

            let correct_primary_rd = correction.get_ref("correct_primary_rd")?.as_str_or_null()?;
            let correct_secondary_rd = correction
                .get_ref("correct_secondary_rd")?
                .as_str_or_null()?;

            let verified_primary_rd = correction
                .get_ref("verified_primary_rd")?
                .as_str_or_null()?;
            let verified_secondary_rd = correction
                .get_ref("verified_secondary_rd")?
                .as_str_or_null()?;
            let suggest_primary_rd = correction.get_ref("suggest_primary_rd")?.as_str_or_null()?;
            let suggest_secondary_rd = correction
                .get_ref("suggest_secondary_rd")?
                .as_str_or_null()?;

            let primary_rd = correct_primary_rd
                .or(verified_primary_rd.or(suggest_primary_rd))
                .unwrap_or("");
            let secondary_rd = correct_secondary_rd
                .or(verified_secondary_rd.or(suggest_secondary_rd))
                .unwrap_or("");

            writeln!(
                corrected_roads,
                "{case_id},\"{primary_rd}\",\"{secondary_rd}\""
            )?;

            if primary_rd.is_empty() {
                println!("WARNING {case_id} has unknown primary_rd: {original_primary_rd}");
                println!("  to get of this warning add '{normal_primary_rd}' as 'normalized_rd' to berkeley-tables/BERKELEY_ROAD_TYPOS.csv and the 'correct_rd' entry");
                println!("  or add the original name '{original_primary_rd}' as 'normalized_rd' to berkeley-tables/BERKELEY_ROAD_TYPOS.csv and the 'correct_rd' entry");
            }

            if secondary_rd.is_empty() {
                println!("WARNING {case_id} has unknown secondary_rd: {original_secondary_rd}");
                println!("  to get of this warning add '{normal_secondary_rd}' as 'normalized_rd' to berkeley-tables/BERKELEY_ROAD_TYPOS.csv and the 'correct_rd' entry");
                println!("  or add the original name '{original_secondary_rd}' as 'normalized_rd' to berkeley-tables/BERKELEY_ROAD_TYPOS.csv and the 'correct_rd' entry");
            }
        }

        // reload data from the CORRECTED_ROADS
        println!("RELOADING corrected_roads with any new roads");
        self.load_data_with_options(
            "corrected_roads",
            Path::new("berkeley-tables/CORRECTED_ROADS.csv"),
            true,
            true,
        )?;

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
            r"(^(?<address_pre>\d+) +)?((?<direction_pre>NORTHBOUND|EASTBOUND|WESTBOUND|SOUTHBOUND|NORTH|EAST|WEST|SOUTH|N/B|E/B|W/B|S/B|NB|EB|WB|SB|N|E|W|S) +)?(?<street>(I-\d+)|(RT +\d+)|(\w+[ \w]+[[:alpha:]]+))([\.,])*(/B)?( +(?<direction_post>NORTHBOUND|EASTBOUND|WESTBOUND|SOUTHBOUND|NORTH|EAST|WEST|SOUTH|N/B|E/B|W/B|S/B|NB|EB|WB|SB|N|E|W|S)[\.,/]*)?( +((?<address_post>\d+)|(\(?(?<block>\d+) BLOCK\)?))$)?",
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
            direction: caps
                .name("direction_post")
                .or_else(|| caps.name("direction_pre"))
                .map(|m| m.as_str()),
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
        test("W COLUSA AV", "COLUSA AV", None, None, Some("W"));
        test("EAST ASHBY AVE", "ASHBY AVE", None, None, Some("EAST"));
    }
}
