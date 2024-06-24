use crate::utils::read_lines;

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let trimmed_line = line.trim();

    let parts: Vec<&str> = trimmed_line.split('|').collect();

    let left_part: Vec<&str> = parts[0].split(':').collect();
    let left_numbers = left_part[1].split_whitespace();

    let mut winning_numbers: Vec<u32> = left_numbers.filter_map(|s| s.parse().ok()).collect();

    winning_numbers.sort();

    let right_numbers = parts[1].split_whitespace();
    let lottery_numbers: Vec<u32> = right_numbers.filter_map(|s| s.parse().ok()).collect(); 

    return (winning_numbers, lottery_numbers);
}

pub fn run_part1() {
    println!("Running day 04 part 1 solution.");

    match read_lines("input/day04.txt") {
        Ok(lines) => {
            let mut point_total = 0;

            for line in lines {
                let nums = parse_line(&line);

                let mut matches: Vec<u32> = Vec::new();

                for number in nums.1 {
                    match nums.0.binary_search(&number) {
                        Ok(_) => matches.push(number),
                        Err(_) => ()
                    }
                }
                if matches.len() > 0 {
                    point_total += 1 * (2_i32.pow((matches.len() - 1) as u32));
                }
            }
            println!("Point total: {}", point_total);

        }
        Err(e) => println!("Error: {}", e),
    } 
}