mod utils;
mod day01;
mod day02;
mod day03;
mod day04;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- <day> <part>");
        return;
    }

    let day = &args[1];
    let part = &args[2];

    match (day.as_str(), part.as_str()) {
        ("1", "1") => day01::run_part1(),
        ("1", "2") => day01::run_part2(),
        ("2", "1") => day02::run_part1(),
        ("2", "2") => day02::run_part2(),
        ("3", "1") => day03::run_part1(),
        ("4", "1") => day04::run_part1(),
        // Keep adding days
        _ => println!("Invalid day or part: {} {}", day, part),
    }
}
