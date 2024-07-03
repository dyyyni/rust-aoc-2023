use crate::utils::read_lines;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_race_data(lines: Vec<String>) -> Vec<Race> {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for line in lines {
        if line.starts_with("Time:") {
            times = line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
        } else if line.starts_with("Distance:") {
            distances = line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
        }
    }
    
    let mut races: Vec<Race> = Vec::new();

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        races.push(Race { time, distance });
    }

    return races;
}

pub fn run_part1() {
    println!("Running day 06 part 1 solution.");

    match read_lines("input/day06.txt") {
        Ok(lines) => {
            let lines: Vec<String> = lines.collect();
            let races = parse_race_data(lines);
            for race in &races {
                println!("{:?}", race);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}