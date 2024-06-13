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

fn collect_adjacent_numbers(grid: &Vec<Vec<char>>, row: usize, col: usize) {
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let left = (0, -1);
    let right = (0, 1);

    let mut adjacent_numbers: Vec<u32> = Vec::new();

    for &(d_row, d_col) in &directions {
        let mut num_str = String::new();
        let new_row = row.wrapping_add(d_row as usize);
        let new_col = col.wrapping_add(d_col as usize);

        if new_row < grid.len() && new_col < grid[new_row].len() {
            let c: char = grid[new_row][new_col];
            if c.is_numeric() {
                while c.is_numeric() {
                    
                }
            }
        }
    }
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
            if is_symbol(c) {
                collect_adjacent_numbers(&grid, row_index, col_index)
            }
        }
        println!();
    }
}