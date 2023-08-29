// TreeHouse

use std::io::stdin;

fn main() {
    println!("Welcome, What is your name?");

    let name = enter_name();

    println!("Glad to have you, {}", name);
}

fn enter_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Cannot read line");

    name.trim().to_lowercase()
}
