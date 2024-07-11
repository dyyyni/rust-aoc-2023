use std::collections::HashMap;

use crate::utils::read_lines;

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<usize>,
    right: Option<usize>,
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
            _   => panic!("Invalid instruction char: {}", c),
        }
    }).collect()
}

fn parse_nodes(node_lines: &[String]) -> (Vec<Node>, HashMap<String, usize>) {
    let mut nodes = Vec::new();
    let mut node_map = HashMap::new();
    let mut pending_connections = Vec::new();

    // First pass: Create all nodes and store their names in the map
    for (index, line) in node_lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let name = parts[0].trim().to_string();
        let connections: Vec<&str> = parts[1].trim_matches(|c| c == '(' || c == ')')
                                            .split(", ")
                                            .collect();

        let left_name = connections.get(0).map(|&n| n.to_string());
        let right_name = connections.get(1).map(|&n| n.to_string());

        nodes.push(Node { name: name.clone(), left: None, right: None });
        node_map.insert(name.clone(), index);

        pending_connections.push((index, left_name, right_name));
    }

    // Resolve the connections
    for (index, left_name, right_name) in pending_connections {
        if let Some(left) = left_name {
            if let Some(&left_index) = node_map.get(&left) {
                nodes[index].left = Some(left_index);
            }
        }
        if let Some(right) = right_name {
            if let Some(&right_index) = node_map.get(&right) {
                nodes[index].right = Some(right_index);
            }
        }
    }

    (nodes, node_map)
}

fn navigation_steps(instructions: &Vec<Instruction>, nodes: &Vec<Node>) -> usize {
    let mut steps = 0;
    let mut current_node = &nodes[0];

    loop {
        if current_node.name == "ZZZ" {
            break;
        }
        // Make the instructions repeat with the modulo
        let instruction = &instructions[steps % instructions.len()];
        steps += 1;

        current_node = match instruction {
            Instruction::Left => {
                if let Some(left_index) = current_node.left {
                    &nodes[left_index]
                } else {
                    panic!("No left node from {}", current_node.name);
                }
            }
            Instruction::Right => {
                if let Some(right_index) = current_node.right {
                    &nodes[right_index]
                } else {
                    panic!("No right node from {}", current_node.name);
                }
            }
        };
    }

    steps
}

pub fn run_part1() {
    println!("Running day 08 part 1 solution.");

    match read_lines("input/day08.txt") {
        Ok(lines) => {
            let lines: Vec<String> = lines.collect();

            let instructions: Vec<Instruction> = parse_instructions(&lines[0]);
            
            let node_lines = &lines[2..];
            let (nodes, node_map) = parse_nodes(node_lines);

            let result = navigation_steps(&instructions, &nodes);
            println!("It takes {} steps to reach ZZZ.", result);
        }
        Err(e) => println!("Error: {}", e),
    }
}