//! CLI for generating the Sqlite DB from the SWITRS database

use std::path::{Path, PathBuf};

use clap::Parser;
use log::info;
use rusqlite::{Connection, DatabaseName};

use switrs_db::schema::{NewDB, Schema};

const OLD_SWITRS_PATH: &str = "old-switrs";

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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("switrs_db=info"))
        .init();

    let data_path = args.data_path;
    let sqlite_file = args.sqlite_file;
    let schema = args.schema;

    info!(
        "Loading data from {data_path} and writing to {sqlite_file}",
        data_path = data_path.display(),
        sqlite_file = sqlite_file.display()
    );

    // we'll build the DB in memory, and then store in a file
    let connection = Connection::open_in_memory()?;

    let schemas = Schema::from_toml_file(&schema)?;
    connection.load_from_schema(&schemas, &Path::new(OLD_SWITRS_PATH))?;

    info!(
        "Successfully imported data, writing DB to {sqlite_file}",
        sqlite_file = sqlite_file.display()
    );
    connection.backup(DatabaseName::Main, sqlite_file, None)?;

    Ok(())
}
