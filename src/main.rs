#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();

    println!("Hello, world!");
}
