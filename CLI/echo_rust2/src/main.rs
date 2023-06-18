//! Author: Shrikrishna Bhat

use std::{env, process::exit, str::FromStr};

fn main() {
    let mut values = Vec::new();

    for arg in env::args().skip(1) {
        values.push(String::from_str(&arg).expect("Error in parsing"));
    }

    if values.len() < 1 {
        println!("Usage: cargo run <enter text>");
        exit(1);
    }

    if values[0] == "-n" {
        for i in 1..values.len() - 1 {
            print!("{} ", values[i]);
        }
        print!("{}", values[values.len() - 1]);
    } else if values[values.len() - 1] == "-n" {
        for i in 0..values.len() - 2 {
            print!("{} ", values[i]);
        }
        print!("{}", values[values.len() - 2]);
    }
    // println!("{}", values[0]);
    else {
        for i in values {
            print!("{}", i);
            print!(" ");
        }
        println!();
    }
}
