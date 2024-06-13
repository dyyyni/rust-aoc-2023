use crate::utils::read_lines;

fn parse_line(line: &str) -> Vec<char> {
    let mut row: Vec<char> = Vec::new();
    for ch in line.chars() {
        row.push(ch);
    }
    return row;
}

pub fn run_part1() {
    println!("Running day 03 part 1 solution.");

    let mut grid: Vec<Vec<char>> = Vec::new();

    match read_lines("input/day03.txt") {
        Ok(lines) => {
            for line in lines {
                grid.push(parse_line(&line));
            }

        }
        Err(e) => println!("Error: {}", e),
        
    }

    for row in &grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}