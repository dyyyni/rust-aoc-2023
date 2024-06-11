use crate::utils::read_lines;

pub fn run() {
    println!("Running day 01 solution.");

    match read_lines("input/day01.txt") {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}