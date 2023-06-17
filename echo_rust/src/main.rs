/*
This code is the old code
Must change the code after writing the complete echo program
*/

use clap::App;
use clap::Arg;

fn main() {
    let matches = App::new("echo_rust")
        .version("0.1.0")
        .author("master-of-none <shrikrishna.bht@gmail.com>")
        .about("Rust echo comand")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Dont print new line")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();

    println!("{:#?}", matches);
}
