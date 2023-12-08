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

fn part1(input: &str) -> usize {
    let (path, graph) = parse_input(input);
    let step_count = simultaneously_traverse_graph(path, graph);
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
    
    let re = Regex::new(r"[A-Z0-9]{3}").unwrap();
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

fn traverse_graph(origin: &str, path: &Path, graph: &Graph) -> usize {
    let mut step_count = 0;
    let mut reached_destination = false;
    let mut current_node = graph.get(origin).expect("Unable to find node");

    while !reached_destination {
        for step in path.iter() {
            if current_node.id.ends_with("Z") { 
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

fn simultaneously_traverse_graph(path: Path, graph: Graph) -> usize {
    let origins: Vec<_> = graph.keys().filter(|key| key.ends_with("A")).collect();
    let step_counts: Vec<usize> = origins.iter().map(|origin| traverse_graph(origin, &path, &graph)).collect();

    let step_count = least_common_multiple(step_counts);

    step_count
}

// From https://github.com/TheAlgorithms/Rust
fn least_common_multiple(nums: Vec<usize>) -> usize {
    if nums.len() == 1 { 
        return nums[0]; 
    };
    let a = nums[0];
    let b = least_common_multiple(nums[1..].to_vec());
    a * b / greatest_common_divisor(a, b)
}

// From https://github.com/TheAlgorithms/Rust
fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    greatest_common_divisor(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part1(input);
        assert_eq!(result, 6);
    }
  }