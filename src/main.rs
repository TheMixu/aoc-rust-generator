use chrono::{Datelike, Local};
use clap::Parser;

use dotenv::dotenv;
use reqwest::blocking::Client;
use std::{
    env,
    error::Error,
    fmt,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};
/// Simple program to generate Advent of Code day binary
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Advent of Code day
    #[arg(short, long)]
    day: u8,
    /// Advent of Code year
    #[arg(short, long, default_value_t = Local::now().date().year())]
    year: i32,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();
    if args.day > 25 || args.day < 1 {
        println!("Day must be between 1 and 25");
    } else {
        match generate_day(&args) {
            Err(e) => println!("Something went wrong with generating day. {}", e),
            Ok(_) => println!("Generated day {}", args.day),
        }
    }
}
#[derive(Debug, Clone)]
struct AocError;
impl Error for AocError {}
impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid response from Advent of Code server.")
    }
}
fn generate_day(args: &Args) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        args.year, args.day
    );
    let key = "AOC_COOKIE";
    let cookie = env::var(key)?;
    let client = Client::new();
    let mut res = client.get(&url).header("cookie", cookie).send()?;
    if res.status() != 200 {
        println!("Advent of Code returned text: {:?}", res.text()?);
        return Err(Box::new(AocError));
    }
    let bin = "./src/bin";
    if !Path::new(bin).exists() {
        create_dir(bin)?;
    }
    let path = format!("./src/bin/{}-{}", args.year, args.day);
    create_dir(&path)?;
    let mut file = File::create(format!("{}/input.txt", &path))?;

    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf)?;
    file.write_all(&buf)?;
    let mut main_file = File::create(format!("{}/main.rs", &path))?;
    main_file.write_all(
        format!(
            "use std::fs;\n\
        fn main() {{\n\
            let input = fs::read(\"./src/bin/{}-{}/input.txt\").unwrap();\n\
        }}\n",
            args.year, args.day
        )
        .as_bytes(),
    )?;
    Ok(())
}
