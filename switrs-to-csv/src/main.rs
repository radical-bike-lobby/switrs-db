use std::path::PathBuf;

use clap::{command, Parser};
use rusqlite::{Connection, OpenFlags};
use switrs_to_csv::Collision;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f')]
    records_db: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let db = Connection::open_with_flags(args.records_db, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let mut stmt = db.prepare(
        r#"
SELECT case_id as case_id FROM collisions LIMIT 1
"#,
    )?;
    let collisions = stmt.query_map([], |row| Collision::try_from(row))?;

    for collision in collisions {
        println!("Found collision {:?}", collision);
    }

    Ok(())
}
