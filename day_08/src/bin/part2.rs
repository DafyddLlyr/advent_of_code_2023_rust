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

fn simultaneously_traverse_graph(path: Path, graph: Graph) -> i32 {
    let mut step_count = 0;
    let origins: Vec<_> = graph.keys().filter(|key| key.ends_with("A")).collect();
    let count_origins = origins.len().clone();

    let mut current_nodes = origins.clone();

    'traverse: loop {
        for step in path.iter() {
            let mut count_destinations_reached = 0;
            
            for (i, &id) in current_nodes.clone().iter().enumerate() {
                let current_node = graph.get(id).expect("Unable to find node");
                if id.ends_with("Z") { 
                    count_destinations_reached += 1;
                    if count_destinations_reached == count_origins { break 'traverse }
                };
                current_nodes[i] = match step {
                    Direction::L => &graph.get(&current_node.left).expect("Unable to find node").id,
                    Direction::R => &graph.get(&current_node.right).expect("Unable to find node").id,
                };
            }

            step_count += 1;
        };
    };
    
    step_count
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