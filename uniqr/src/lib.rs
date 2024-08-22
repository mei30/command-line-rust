use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    #[arg(short, long)]
    count: bool
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> Result<()> {
    let mut args = Args::parse();

    println!("{:#?}", args);

    Ok(())
}