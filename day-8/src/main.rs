use std::collections::HashMap;

use num::Integer;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }).collect();
    let mut nodes = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(" = ");
        let name = parts.next().unwrap();
        let parts = parts.next().unwrap().split(", ");
        let left = parts.clone().nth(0).unwrap().replace("(", "");
        let right = parts.clone().nth(1).unwrap().replace(")", "");
        nodes.insert(name.to_string(), Node {
            left: left.to_string(),
            right: right.to_string(),
        });
    }
    (instructions, nodes)
}

fn find_steps(instructions: &[Direction], nodes: &HashMap<String, Node>) -> usize {
    let goal = "ZZZ".to_string();
    let mut current = "AAA".to_string();
    let mut steps = 0;
    while current != goal {
        let node = nodes.get(&current).unwrap();
        let direction = instructions[steps % instructions.len()].clone();
        current = match direction {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
        steps += 1;
    }
    steps
}

fn find_steps_lower_bound(instructions: &[Direction], nodes: &HashMap<String, Node>, start: String) -> usize {
    let mut current = start;
    let mut steps = 0;
    while !current.ends_with("Z") {
        let node = nodes.get(&current).unwrap();
        let direction = instructions[steps % instructions.len()].clone();
        current = match direction {
            Direction::Left => node.left.clone(),
            Direction::Right => node.right.clone(),
        };
        steps += 1;
    }
    steps
}

fn find_ghost_steps(instructions: &[Direction], nodes: &HashMap<String, Node>) -> usize {
    let mut current = nodes.keys().filter(|k| k.ends_with("A")).map(|k| k.clone()).collect::<Vec<String>>();
    let min_steps = current.iter().map(|c| find_steps_lower_bound(&instructions, &nodes, c.clone()));
    min_steps.fold(1, |acc, x| acc.lcm(&x))
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (instructions, nodes) = parse_input(&input);
    let steps = find_steps(&instructions, &nodes);
    println!("Steps: {}", steps);

    let steps = find_ghost_steps(&instructions, &nodes);
    println!("Ghost steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_steps() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (instructions, nodes) = parse_input(&input);
        let steps = find_steps(&instructions, &nodes);
        assert_eq!(2, steps);
    }

    #[test]
    fn six_steps() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (instructions, nodes) = parse_input(&input);
        let steps = find_steps(&instructions, &nodes);
        assert_eq!(6, steps);
    }

    #[test]
    fn ghost_steps() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (instructions, nodes) = parse_input(&input);
        let steps = find_ghost_steps(&instructions, &nodes);
        assert_eq!(6, steps);
    }
}
