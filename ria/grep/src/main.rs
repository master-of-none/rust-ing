use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("Searches for the pattern")
        .arg(
            Arg::new("pattern")
                .help("Pattern to search for")
                .required(true),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "\
Every photo, every picture, every face, bedroom window, is a picture in search of what
It is same with books.
What do we seek - a picture, through million of pages.";

    for line in quote.lines() {
        let contains_substring = re.find(line);

        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
