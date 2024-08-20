use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Rust version of `wc`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run() -> Result<()> {
    let mut args = Args::parse();
    if [args.lines, args.words, args.words, args.chars]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.bytes = true;
        args.words = true;
    }

    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(_) => println!("Opened: {}", filename),
        }
    }

    Ok(())
}
