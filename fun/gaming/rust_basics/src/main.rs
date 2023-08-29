// TreeHouse

use std::io::stdin;

#[derive(Debug)]
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
    let mut visitors = vec![
        Visitor::new("kishan", "Hello Kishan enjoy"),
        Visitor::new("LUFFY", "Kaizoku niro otakotoa"),
        Visitor::new("NaRuTo", "Hokage, Dattebaayo"),
    ];

    loop {
        println!("Enter your name");
        let name = enter_name();

        let allowed = visitors.iter().find(|visitor| visitor.name == name);

        match allowed {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not an authorized visitor", name);
                    println!("Press y to add else to exit");
                    let mut key = String::new();
                    stdin().read_line(&mut key).expect("Cant read string");

                    match key.trim().to_lowercase().as_str() {
                        "y" => visitors.push(Visitor::new(&name, "Welcome to the group")),
                        _ => break,
                    }
                }
            }
        }
    }

    println!("Visitors");
    println!("{:#?}", visitors);
}

fn enter_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Cannot read line");

    name.trim().to_lowercase()
}
