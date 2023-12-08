use std::collections::HashMap;
use regex::Regex;

enum Direction {
    L,
    R,
}

type Path = Vec<Direction>;

type Graph = HashMap<String, Node>;

struct Node {
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let (path, graph) = parse_input(input);
    let step_count = traverse_graph(path, graph);
    step_count
}

fn parse_input(input: &str) -> (Path, Graph) {
    let path: Path = input
        .lines()
        .nth(0)
        .expect("Failed to read first line")
        .chars()
        .map(|char| match char {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid directions"),
        })
        .collect();
    
    let re = Regex::new(r"[A-Z]{3}").unwrap();
    let mut graph: Graph = Graph::new();
    input
        .lines()
        .skip(2)
        .for_each(|line| {
            let line_parts: Vec<String> = re.find_iter(line).map(|m| m.as_str().to_string()).collect();
            let id = line_parts[0].clone();
            let left = line_parts[1].clone();
            let right = line_parts[2].clone();

            graph.insert(id.clone(), Node { id, left, right });
        });

    (path, graph)
}

fn traverse_graph(path: Path, graph: Graph) -> i32 {
    let origin = "AAA";
    let destination = "ZZZ";
    let mut step_count = 0;
    let mut reached_destination = false;
    let mut current_node = graph.get(origin).expect("Unable to find node");

    while !reached_destination {
        for step in path.iter() {
            if current_node.id == destination { 
                reached_destination = true; 
                break;
            };
            
            step_count += 1;
            current_node = match step {
                Direction::L => graph.get(&current_node.left).expect("Unable to find node"),
                Direction::R => graph.get(&current_node.right).expect("Unable to find node"),
            };
        }
    }

    step_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_travels_linear_path() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_travels_repeating_path() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 6);
    }
}