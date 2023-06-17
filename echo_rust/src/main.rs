use clap::App;

fn main() {
    //println!("{:#?}", std::env::args());
    let _matches = App::new("echo_rust")
        .version("0.1.0")
        .author("master-of-none <shrikrishna.bht@gmail.com>")
        .about("Rust echo comand")
        .get_matches();
}
