use std::collections::HashMap;
use crate::utils::read_lines;

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<String>,
    right: Option<String>,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions.chars().map(|c| {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction char: {}", c),
        }
    }).collect()
}

fn parse_nodes(node_lines: &[String]) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for line in node_lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let name = parts[0].trim().to_string();
        let connections: Vec<&str> = parts[1].trim_matches(|c| c == '(' || c == ')')
                                            .split(", ")
                                            .collect();

        let left_name = connections.get(0).map(|&n| n.to_string());
        let right_name = connections.get(1).map(|&n| n.to_string());

        nodes.insert(name.clone(), Node { name, left: left_name, right: right_name });
    }

    nodes
}

fn navigation_steps(instructions: &Vec<Instruction>, nodes: &HashMap<String, Node>, start_node: &str) -> Result<usize, String> {
    let mut steps = 0;
    let mut current_node = nodes.get(start_node).ok_or_else(|| format!("Start node {} not found", start_node))?;

    loop {
        if current_node.name == "ZZZ" {
            return Ok(steps);
        }
        
        let instruction = &instructions[steps % instructions.len()];
        steps += 1;

        current_node = match instruction {
            Instruction::Left => {
                if let Some(left_name) = &current_node.left {
                    nodes.get(left_name).ok_or_else(|| format!("No left node from {}", current_node.name))?
                } else {
                    return Err(format!("No left node from {}", current_node.name));
                }
            }
            Instruction::Right => {
                if let Some(right_name) = &current_node.right {
                    nodes.get(right_name).ok_or_else(|| format!("No right node from {}", current_node.name))?
                } else {
                    return Err(format!("No right node from {}", current_node.name));
                }
            }
        };
    }
}

pub fn run_part1() {
    println!("Running day 08 part 1 solution.");
    match read_lines("input/day08.txt") {
        Ok(lines) => {
            let lines: Vec<String> = lines.collect();

            let mut instruction_end_index = 0;
            for (i, line) in lines.iter().enumerate() {
                if line.contains(" = ") {
                    instruction_end_index = i;
                    break;
                }
            }

            let instruction_str = lines[..instruction_end_index].join("");

            let instructions: Vec<Instruction> = parse_instructions(&instruction_str);

            let node_lines = &lines[instruction_end_index..];
            let nodes = parse_nodes(node_lines);

            match navigation_steps(&instructions, &nodes, "AAA") {
                Ok(steps) => println!("It takes {} steps to reach ZZZ.", steps),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}