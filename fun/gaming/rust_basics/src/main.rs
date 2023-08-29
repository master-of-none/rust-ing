// TreeHouse

use std::io::stdin;

fn main() {
    println!("Welcome, What is your name?");

    let name = enter_name();

    let visitor_list = ["bert", "steve", "fred", "kishan"];

    let mut allowed = false;

    for visitor in &visitor_list {
        if visitor == &name {
            allowed = true;
        }
    }

    if allowed {
        println!("Glad to you have you {}", name)
    } else {
        println!("Sorry, You are not in the visitor list")
    }
}

fn enter_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Cannot read line");

    name.trim().to_lowercase()
}
