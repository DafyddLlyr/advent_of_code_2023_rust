#[derive(PartialEq, Eq, Copy, Clone)]
enum Cosmic {
    EmptySpace,
    Galaxy,
}

type Coord = (i32, i32);

type Matrix = Vec<Vec<Cosmic>>;

type Network = Vec<Vec<i32>>;

struct Universe {
    galaxies: Vec<Coord>,
}

impl Universe {
    fn new(input: &str) -> Self {
        let mut matrix = Universe::parse_matrix_from_input(input);
        matrix = Universe::cosmic_expansion(&mut matrix);
        let galaxies = Universe::find_galaxies(&matrix);
        
        Universe { galaxies }
    }

    fn parse_matrix_from_input(input: &str) -> Matrix {
        input
            .lines()
            .map(|line| line.chars().map(Universe::map_char_to_cosmic).collect())
            .collect::<Matrix>()
    }

    fn map_char_to_cosmic(char: char) -> Cosmic {
        match char {
            '#' => Cosmic::Galaxy,
            '.' => Cosmic::EmptySpace,
            _ => panic!("Invalid input"),
        }
    }

    fn cosmic_expansion(matrix: &mut Matrix) -> Matrix {
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        // Scan for empty rows
        let mut empty_rows: Vec<i32> = Vec::new();
        for row_index in 0..row_count { 
            match matrix[row_index].iter().all(|&cosmic| cosmic == Cosmic::EmptySpace) {
                true => empty_rows.insert(0, row_index as i32),
                false => continue,
            }
        }

        // Scan for empty columns
        let mut empty_cols: Vec<i32> = Vec::new();
        for col_index in 0..col_count {
            let mut col: Vec<Cosmic> = Vec::new();
            for row_index in 0..row_count { 
                col.push(matrix[row_index][col_index])
            }

            match col.iter().all(|&cosmic| cosmic == Cosmic::EmptySpace) {
                true => empty_cols.insert(0, col_index as i32),
                false => continue,
            }
        }

        // Add empty rows
        empty_rows
            .iter()
            .for_each(|row_index| matrix
                .insert(*row_index as usize, vec![Cosmic::EmptySpace; col_count]));

        // Add empty columns
        empty_cols
            .iter()
            .for_each(|col_index| (0..matrix.len())
                .for_each(|row_index| 
                    matrix[row_index].insert(*col_index as usize, Cosmic::EmptySpace)));

        matrix.clone()
    }

    fn find_galaxies(matrix: &Matrix) -> Vec<Coord> {
        let mut galaxies: Vec<Coord> = Vec::new();
        for (row_index, row) in matrix.iter().enumerate() {
            row
                .iter()
                .enumerate()
                .filter(|(_, &cosmic)| cosmic == Cosmic::Galaxy)
                .for_each(|(col_index, _)| galaxies.push((row_index as i32, col_index as i32)));
        }
        galaxies
    }

    fn build_network(&self) -> Network {
        let mut network: Network = Vec::new();

        for origin in self.galaxies.iter() {
            let galaxy_network = self.galaxies
                .iter()
                .map(|destination| self.diff_galaxies(&origin, &destination))
                .collect();
            network.push(galaxy_network);
        }

        network
    }

    fn diff_galaxies(&self, origin: &Coord, destination: &Coord) -> i32 {
        // Same galaxy - zero steps away
        if origin == destination { return 0; };

        let (origin_x, origin_y) = origin;
        let (destination_x, destination_y) = destination;

        // Don't match previous galaxies
        if origin_x > destination_x { return -1; };
        if origin_x == destination_x && origin_y > destination_y { return -1; };

        let diff_x = destination_x - origin_x;
        let diff_y = destination_y - origin_y;

        let distance = i32::abs(diff_x) + i32::abs(diff_y);
        distance
    }

}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let universe = Universe::new(input);
    let network = universe.build_network();
    let sum_shortest_paths: i32 = network
        .iter()
        .map(|galaxy_network| galaxy_network
            .iter()
            .filter(|&distance| distance > &0))
        .flatten()
        .sum();

    sum_shortest_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part1(input);
        assert_eq!(result, 374);
    }
}