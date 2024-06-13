use crate::utils::read_lines;

fn parse_line(line: &str) -> Vec<char> {
    let mut row: Vec<char> = Vec::new();
    for ch in line.chars() {
        row.push(ch);
    }
    return row;
}

fn is_symbol(c: char) -> bool {
    (c.is_ascii_punctuation() || c.is_ascii_graphic() && !c.is_alphanumeric()) && c != '.'
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


    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            let c = grid[row_index][col_index];
            println!("Is '{}' a symbol? {}", c, is_symbol(c))
        }
        println!();
    }
}