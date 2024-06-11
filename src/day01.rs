use crate::utils::read_lines;

pub fn run_part1() {
    println!("Running day 01 solution.");

    match read_lines("input/day01.txt") {
        Ok(lines) => {
            let mut calibration_sum = 0;
            for line in lines {
                calibration_sum += add_calibration_number(&line)
            }
            println!("Calibration sum: {}", calibration_sum);
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn run_part2() {
    println!("Hello main");
}

fn add_calibration_number(line: &str) -> i32 {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rev().find(|c| c.is_digit(10));

    if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
        let first_digit_str = first_digit.to_string();
        let last_digit_str = last_digit.to_string();

        let concat_str = first_digit_str + &last_digit_str;
        let number: i32 = concat_str.parse().expect("Failed to parse concatenated number");
        return number;
    }
    else {
        println!("No digits found.");
        return 0;
    }
}