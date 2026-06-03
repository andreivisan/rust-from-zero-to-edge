/*
 * Simple Rust CLI for learning purposes
 * */
use std::{env, io::{self, Write}};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No arguments provided");
    }
    let nickname = &args[1];

    println!("Hello, {name} aka {nickname}!");
}
