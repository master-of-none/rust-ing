/*
This code is the old code
Must change the code after writing the complete echo program
*/

//! Check the STDOUT and STDERR using the below command  
//! cargo run -- Hello World 1>out 2>err
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

    let text = matches.values_of_lossy("text").unwrap();
    //println!("{}", text.join(" "));

    //println!("{:#?}", matches);
    //assert_eq!(text.join(" "), "Hello World");

    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
