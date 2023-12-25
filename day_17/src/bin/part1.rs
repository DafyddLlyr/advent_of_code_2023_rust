use std::collections::HashMap;

struct Matrix(Vec<Vec<u32>>);

impl Matrix {
    fn get(&self, coord: Coord) -> Option<u32> {
        let (x, y) = coord;
        let result = self.0.get(y)?.get(x)?;
        Some(*result)
    }
    
    fn get_width(&self) -> usize {
        self.0.get(0).expect("Matrix is empty").len()
    }
    
    fn get_height(&self) -> usize {
        self.0.len()
    }

    fn build_edges(&self, (x, y): Coord) -> Vec<(Coord, u32)> {
        let mut edges = Vec::new();
        // Up
        if y > 0 {
            let up = (x, y - 1);
            let weight = self.get(up).unwrap();
            edges.push((up, weight));
        }
        // Down
        if y < self.get_height() {
            let down = (x, y + 1);
            let weight = self.get(down).unwrap();
            edges.push((down, weight));
        }
        // Left
        if x > 0 {
            let left = (x - 1, y);
            let weight = self.get(left).unwrap();
            edges.push((left, weight));
        }
        // Right
        if x < self.get_width() {
            let right = (x + 1, y);
            let weight = self.get(right).unwrap();
            edges.push((right, weight));
        }
        
        edges
    }
}

type Coord = (usize, usize);

struct Node {
    id: Coord,
    edges: Vec<(Coord, u32)>,
}

struct Graph {
    nodes: HashMap<Coord, Node>,
    last_node: Coord,
}

impl Graph {
    fn new(input: &str) -> Self {
        let matrix = Graph::parse_matrix(input);
        let nodes = Graph::build_nodes_from_matrix(&matrix);
        let last_node = (matrix.get_width(), matrix.get_height());

        Graph { nodes, last_node }
    }

    fn parse_matrix(input: &str) -> Matrix {
        Matrix(input
            .lines()
            .map(|line| line
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect())
            .collect()
        )
    }

    fn build_nodes_from_matrix(matrix: &Matrix) -> HashMap<Coord, Node> {
        let mut nodes = HashMap::<Coord, Node>::new();

        let (width, height) = (matrix.get_width(), matrix.get_height());

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let id = (x, y);
                let edges = matrix.build_edges(id);
                nodes.insert(id, Node { id, edges });
            }
        }
        nodes
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> u32 {
    let graph = Graph::new(input);
    let first_node = (0 as usize, 0 as usize);
    let last_node = graph.last_node;
    // Use Dijkstra's algorithm to find shortest path to end
    // Add check for number of consecutive steps
    123
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result = part1(input);
        assert_eq!(result, 102);
    }
}