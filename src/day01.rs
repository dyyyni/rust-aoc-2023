use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;

pub fn run_part1() {
    println!("Running day 01 part 1 solution.");

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

pub fn run_part2() {
    println!("Running day 01 part 2 solution.");

    match read_lines("input/day01.txt") {
        Ok(lines) => {
            let mut calibration_sum = 0;
            for line in lines {
                let result = add_calibration_number_part2(&line);
                println!("{} | {}", line, result);
                calibration_sum += result;
            }
            println!("Calibration number: {}", calibration_sum);
        }
    Err(e) => println!("Error: {}", e),
    }
}

fn add_calibration_number_part2(line: &str) -> i32 {
    let spelled_numbers: HashMap<&str, char> = [
        ("one", '1'), ("two", '2'), ("three", '3'),
        ("four", '4'), ("five", '5'), ("six", '6'), 
        ("seven", '7'), ("eight", '8'), ("nine", '9') 
    ].iter().cloned().collect();

    let re = 
    Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();

    let mut found_numbers = Vec::new();

    for hit in re.find_iter(line) {
        let match_str = hit.as_str();
        if let Some(&digit_char) = spelled_numbers.get(match_str) {
            found_numbers.push(digit_char);
        } else if let Ok(digit_char) = match_str.parse::<char>() {
            found_numbers.push(digit_char);
        }
    }

    if let (Some(&first_digit), Some(&last_digit)) = (found_numbers.first(), found_numbers.last()) {
        let concatenated_str = format!("{}{}", first_digit, last_digit);
        let concatenated_number: i32 = concatenated_str.parse().expect("Failed to parse the concatenated string.");
        return concatenated_number;
    } else {
        println!("Could not find the first digit or the last digit.");
        return 0;
    }
}
