use clap::Command;

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Shrikrishna Bhat")
        .about("Rust Echo")
        .get_matches();
}
