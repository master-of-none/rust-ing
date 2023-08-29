// TreeHouse

use std::io::stdin;

fn main() {
    println!("Welcome, What is your name?");

    let mut name = String::new();
    stdin().read_line(&mut name).expect("Cannot read line");

    println!("Glad to have you, {}", name);
}
