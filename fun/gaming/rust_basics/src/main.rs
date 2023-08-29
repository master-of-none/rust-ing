// TreeHouse

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
fn main() {
    println!("Welcome, What is your name?");
    let visitors = [
        Visitor::new("kishan", "Hello Kishan enjoy"),
        Visitor::new("LUFFY", "Kaizoku niro otakotoa"),
        Visitor::new("NaRuTo", "Hokage, Dattebaayo"),
    ];

    println!("Enter your name");
    let name = enter_name();

    let allowed = visitors.iter().find(|visitor| visitor.name == name);

    match allowed {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Not allowed exit"),
    }
}

fn enter_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Cannot read line");

    name.trim().to_lowercase()
}
