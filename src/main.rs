mod day01;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <day>");
        return;
    }

    let day = &args[1];

    match day.as_str() {
        "1" => day01::run(),
        // Keep adding days
        _ => println!("Invalid day: {}", day),
    }
}
