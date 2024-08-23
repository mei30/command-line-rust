use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Rust version of `find`
struct Args {
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    Ok(())
}
