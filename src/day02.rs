
use crate::utils::read_lines;

#[derive(Debug)]
struct GameLimits {
    red_cubes: u16,
    green_cubes: u16,
    blue_cubes: u16,
}

const GAME_LIMITS: GameLimits = GameLimits {
    red_cubes: 12,
    green_cubes: 13,
    blue_cubes: 14,
};

struct CubeGame {
    game_id: u16,
    subsets: Vec<Subset>,
}

#[derive(Debug)]
struct Subset {
    red_cubes: u16,
    green_cubes: u16,
    blue_cubes: u16,
}

fn parse_game_line(line: &str) -> Option<CubeGame> {
    let parts: Vec<&str> = line.splitn(2, ": ").collect();
    if parts.len() != 2 {
        return None;
    }

    let game_id_part = parts[0];
    let game_id_str = game_id_part.trim_start_matches("Game ");
    let game_id: u16 = game_id_str.parse().ok()?;

    let game_played_part = parts[1];
    let subsets: Vec<Subset> = game_played_part
    .split(';')
    .map(|s| parse_subset(s.trim()))
    .collect::<Option<Vec<_>>>()?;

    Some(CubeGame { game_id, subsets })

}

fn parse_subset(subset: &str) -> Option<Subset> {
    let mut red_cubes = 0;
    let mut green_cubes = 0;
    let mut blue_cubes = 0;

    for item in subset.split(',') {
        let parts: Vec<&str> = item.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let count: u16 = parts[0].parse().ok()?;
        match parts[1] {
            "red"   => red_cubes    += count,
            "green" => green_cubes  += count,
            "blue"  => blue_cubes   += count,
            _ => return None,
        }
    }

    Some(Subset {
        red_cubes,
        green_cubes,
        blue_cubes,
    })
}

fn is_game_legal(subsets: &Vec<Subset>) -> bool {
    let mut is_legal = true;
    for subset in subsets {
        if subset.red_cubes > GAME_LIMITS.red_cubes {
            is_legal = false;
        }
        if subset.green_cubes > GAME_LIMITS.green_cubes {
            is_legal = false;
        }
        if subset.blue_cubes > GAME_LIMITS.blue_cubes {
            is_legal = false;
        }
    }
    return is_legal;
}

pub fn run_part1() {
    println!("Running day 02 part 1 solution.");

    match read_lines("input/day02.txt") {
        Ok(lines) => {
            let mut accepted_games: Vec<u16> = Vec::new();
            for line in lines {
                if let Some(cube_game) = parse_game_line(&line) {
                    if is_game_legal(&cube_game.subsets) {
                        accepted_games.push(cube_game.game_id);
                    }
                } else {
                    println!("Failed to parse the line.");
                }
            }
            let mut sum_accepted_ids = 0;
            for id in accepted_games {
                sum_accepted_ids += id;
            }
            println!("The sum of legal game IDs: {}", sum_accepted_ids);
        }
        Err(e) => println!("Error: {}" ,e), 
    }
}
