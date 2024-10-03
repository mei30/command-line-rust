use crate::Column::*;
use anyhow::{anyhow, bail, Result};
use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering::*,
    fs::File,
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

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
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

    let case = |line: String| {
        if args.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().map_while(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().map_while(Result::ok).map(case);

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if args.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if args.show_col2 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if args.show_col3 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    if args.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&args.delimiter));
        }
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}
