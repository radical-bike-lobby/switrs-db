//! CLI for generating the Sqlite DB from the SWITRS database

use std::path::PathBuf;

use clap::Parser;
use rusqlite::{Connection, DatabaseName};

use switrs_db::schema::{NewDB, Schema};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the raw data dump from iswitrs
    #[arg(short = 'd')]
    data_path: PathBuf,

    /// SQLITE db file to create from the raw data
    #[arg(short = 'f')]
    sqlite_file: PathBuf,

    /// Path to the Schemas TOML configuration file
    #[arg(short = 's', default_value = "Schemas.toml")]
    schema: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data_path = args.data_path;
    let sqlite_file = args.sqlite_file;
    let schema = args.schema;

    println!(
        "Loading data from {data_path} and writing to {sqlite_file}",
        data_path = data_path.display(),
        sqlite_file = sqlite_file.display()
    );

    // we'll build the DB in memory, and then store in a file
    let connection = Connection::open_in_memory()?;

    let schemas = Schema::from_toml_file(&schema)?;
    connection.load_from_schema(&schemas, &data_path)?;

    println!(
        "Successfully imported data, writing DB to {sqlite_file}",
        sqlite_file = sqlite_file.display()
    );
    connection.backup(DatabaseName::Main, sqlite_file, None)?;

    Ok(())
}
