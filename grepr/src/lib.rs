use anyhow::Result;
use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Rust version of grep
struct Args {
    /// Pattern search for...
    #[arg(value_name = "PATTERN", value_parser(Regex::new))]
    pattern: Regex,

    /// A FILE of “-” stands for standard input.  If no FILE is given, recursive searches examine the working directory, and nonrecursive searches read standard input.
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Suppress  normal  output;  instead  print  a  count  of matching lines for each input file.
    #[arg(short)]
    count: bool,

    /// Ignore  case  distinctions  in patterns and input data, so that characters that differ only in case match each other
    #[arg(short('i'))]
    case_insensitive: bool,

    /// Invert the sense of matching, to select non-matching lines.
    #[arg(short('v'))]
    invert_match: bool,

    /// Read all files under each directory, recursively.
    #[arg(short('r'), long)]
    recursive: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> Result<()> {
    let config = Args::parse();

    let is_file = |x: &DirEntry| x.file_type().is_file();

    let re = RegexBuilder::new(config.pattern.as_str())
        .case_insensitive(config.case_insensitive)
        .build()
        .unwrap();

    let mut buffer: String = String::new();
    let mut count = 0;
    let mut invert_match_count = 0;
    for filename in config.files {
        let metadata = fs::metadata(filename.as_str())?;

        count = 0;
        invert_match_count = 0;

        if metadata.is_file() {
            match open(&filename) {
                Err(err) => eprintln!("{}: {}", filename, err),
                Ok(mut file_handle) => loop {
                    let result = file_handle.read_line(&mut buffer)?;

                    if result == 0 {
                        break;
                    }
                    if re.is_match(&buffer) {
                        count += 1;

                        if !config.count {
                            print!("{}", buffer);
                        }
                    } else if config.invert_match {
                        invert_match_count += 1;
                        if !config.count {
                            print!("{}", buffer);
                        }
                    }

                    buffer.clear();
                },
            }

            if config.count {
                if config.invert_match {
                    println!("{}:{}", filename, invert_match_count);
                } else {
                    println!("{}:{}",filename, count);
                }
            }
        } else if metadata.is_dir() && config.recursive {
            for entry in WalkDir::new(filename.as_str())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(is_file)
            {
                count = 0;
                invert_match_count = 0;

                match open(&entry.path().to_str().unwrap()) {
                    Err(err) => eprintln!("{}: {}", filename, err),
                    Ok(mut file_handle) => loop {
                        let result = file_handle.read_line(&mut buffer)?;

                        if result == 0 {
                            break;
                        }

                        if re.is_match(&buffer) {
                            count += 1;

                            if !config.count {
                                print!("{}", buffer);
                            }
                        } else if config.invert_match {
                            invert_match_count += 1;
                            if !config.count {
                                print!("{}", buffer);
                            }
                        }

                        buffer.clear();
                    },
                }

                if config.count {
                    if config.invert_match {
                        println!("{}:{}", entry.path().display(), invert_match_count);
                    } else {
                        println!("{}:{}", entry.path().display(), count);
                    }
                }
            }
        } else {
            eprintln!("{} is a directory", filename);
            continue;
        }
    }

    Ok(())
}
