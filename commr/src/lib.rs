use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
// Rust version of comm

struct Args {
    /// Input file 1
    #[arg(value_name("FILE1"), default_value("-"))]
    file1: String,

    /// Input file 2
    #[arg(value_name("FILE2"), default_value("-"))]
    file2: String,

    /// Suppress printing of column 1
    #[arg(short('1'))]
    show_col1: bool,

    /// Suppress printing of column 2
    #[arg(short('2'))]
    show_col2: bool,

    /// Suppress printing of column 3
    #[arg(short('3'))]
    show_col3: bool,

    /// Case-insensitive comparison of lines
    #[arg(short('i'))]
    insensitive: bool,

    /// Output delimiter
    #[arg(
        short('d'),
        long("output-delimiter"),
        value_name("DELIM"),
        default_value("\t")
    )]
    delimiter: String,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{}: {}", filename, e))?,
        ))),
    }
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let file1 = &args.file1;
    let file2 = &args.file2;

    if file1 == "_" && file2 == "_" {
        return Err(anyhow!("Both input files cannot be STDIN (\"-\")".to_string()));
    }

    let _file1 = open(file1)?;
    let _file2 = open(file2)?;

    println!("{:#?}", args);
    Ok(())
}
