//! Schema operations for the SWITRS sqlite DB creation

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use new_string_template::template::Template;
use rusqlite::{params_from_iter, Connection};
use serde::Deserialize;

/// Specifies which schema and data should be used for creating a table
#[derive(Debug, Deserialize)]
pub struct TableDefinition {
    schema: PathBuf,
    data: PathBuf,
}

pub trait NewDB {
    fn connection(&self) -> &Connection;

    fn create_table(
        &self,
        name: &str,
        table_schema: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // build the DDL expression
        let ddl = fs::read_to_string(table_schema)?;
        let ddl = Template::new(ddl);
        let data = {
            let mut map = HashMap::new();
            map.insert("table", name);
            map
        };

        let ddl = ddl.render(&data)?;

        self.connection().execute_batch(&ddl)?;
        Ok(())
    }

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
            .from_path(&table_data)?;

        // build up the insert statement
        let (fields, values) = {
            // construct "field = "
            let headers = csv.headers()?;
            let mut fields = String::new();
            let mut values = String::new();
            let mut first = true;
            for f in headers.iter() {
                if !first {
                    fields.push_str(", ");
                    values.push_str(", ");
                } else {
                    first = false;
                }

                fields.push_str(f);
                values.push('?');
            }

            (fields, values)
        };

        let insert_stmt = format!("INSERT INTO {name} ({fields}) VALUES({values})");

        let mut stmt = self.connection().prepare(&insert_stmt)?;

        // collect all the data
        let mut count = 0;
        for record in csv.into_records() {
            let record = record?;
            stmt.insert(params_from_iter(record.into_iter()))?;
            count += 1;
        }

        Ok(count)
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
    fn test_create_table_char_1() {
        let connection = Connection::open_in_memory().expect("failed to open in memory DB");
        let table = TableDefinition {
            schema: PathBuf::from("schema/char-1-id.sql"),
            data: PathBuf::from("lookup-tables/DAY_OF_WEEK.csv"),
        };

        connection
            .connection()
            .create_table("day_of_week", &table.schema)
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
        let table = TableDefinition {
            schema: PathBuf::from("schema/char-2-id.sql"),
            data: PathBuf::from("lookup-tables/PCF_VIOLATION_CATEGORY.csv"),
        };

        connection
            .connection()
            .create_table("pcf_violation_category", &table.schema)
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
        let table = TableDefinition {
            schema: PathBuf::from("schema/varchar-2-id.sql"),
            data: PathBuf::from("lookup-tables/PRIMARY_RAMP.csv"),
        };

        connection
            .connection()
            .create_table("primary_ramp", &table.schema)
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

        connection
            .connection()
            .create_table("collisions", Path::new("schema/collisions.sql"))
            .expect("failed to create table");

        connection
            .execute("SELECT * from collisions", [])
            .expect("failed to execute query");

        let count = connection
            .connection()
            .load_data("collisions", Path::new("tests/data/collisions.csv"))
            .expect("failed to create table");

        assert_eq!(6, count);
    }
}
