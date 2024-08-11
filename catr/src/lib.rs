use std::error::Error;
use std::fs;

use clap::{Arg, App};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Meisam Jafari Mosleh <meisamjafarimosleh@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input File(s)")
            .multiple(true)
            .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank")
        )
        .arg(
            Arg::with_name("number_nonblank")
            .short("b")
            .long("number_nonblank")
            .help("Number non-blank lines")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for (index , file) in config.files.iter().enumerate() {
        let text: String = fs::read_to_string(file)?;
        println!("{}", text);

        let mut text_index = 0;
        let mut line_break_index = 0;
        while text_index <= text.len() {
            let substring: &str = &text[text_index + line_break_index..];

            line_break_index = match substring.find('\n') {
                Some(value) => value,
                None => text.len()
            };

            let text_slice: &str = &text[(text_index + 1)..line_break_index];
        } 
    }


    Ok(())
}