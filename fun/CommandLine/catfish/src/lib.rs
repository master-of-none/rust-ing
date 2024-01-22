use clap::{Arg, Command};
use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Vec<String>,
    number_lines: bool,
    number_non_blank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catfish")
        .version("0.1.0")
        .author("Shrikrishna Bhat")
        .about("Cat Fishing")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input Files")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number Lines")
                .conflicts_with("number_nonblank")
                .num_args(0),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non blank lines")
                .num_args(0),
        )
        .get_matches();
    Ok(Config {
        file: matches
            .get_many("file")
            .expect("Files needed")
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number"),
        number_non_blank_lines: matches.get_flag("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for files in config.file {
        println!("{}", files);
    }
    Ok(())
}
