//! Schema operations for the SWITRS sqlite DB creation

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use new_string_template::template::Template;
use rusqlite::{params_from_iter, Connection};
use serde::Deserialize;

/// Specifies which schema and data should be used for creating a table
#[derive(Debug, Deserialize)]
pub struct LookupTable {
    pk_type: String,
    data: PathBuf,
    schema: Option<PathBuf>,
}

/// Primary Table definition as defined in the Toml
#[derive(Debug, Deserialize)]
pub struct PrimaryTable {
    /// Path to the schema file for the table, like collisions.sql
    schema: PathBuf,
    /// The file name (relative to where the raw data was extracted) of the csv data
    raw_data: PathBuf,
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
            for f in &headers_record {
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

        let insert_stmt = format!("INSERT INTO {name} ({fields}) VALUES({values})");

        let mut stmt = self.connection().prepare(&insert_stmt)?;

        // collect all the data
        let mut count = 0;
        for record in csv.into_records() {
            let record = record?;

            // convert empty strings to NULL, should we change '-' to NULL as well?
            let record_iter = record
                .into_iter()
                .map(|s| if s.is_empty() { None } else { Some(s) });
            stmt.insert(params_from_iter(record_iter))
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
        self.connection()
            .init_lookup_tables(&schemas.lookup_tables, &schemas.lookup_schema)?;

        for table_name in &schemas.table_order {
            let table: &PrimaryTable = schemas
                .tables
                .get(table_name)
                .ok_or_else(|| format!("table missing from [tables]: {table_name}"))?;

            let data = data.join(&table.raw_data);

            println!("LOADING {table_name}");
            self.connection()
                .create_table(table_name, "", &table.schema)?;
            self.connection().load_data(table_name, &data)?;
        }

        Ok(())
    }
}

impl NewDB for Connection {
    fn connection(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml() {
        let schemas = Schema::from_toml_file(Path::new("Schemas.toml")).expect("toml is bad");

        assert_eq!(schemas.table_order[0], "collisions");
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
            data: PathBuf::from("lookup-tables/PCF_VIOLATION_CATEGORY.csv"),
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

        assert_eq!(10, count);
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
}
