#[derive(PartialEq, Eq, Copy, Clone)]
enum Cosmic {
    EmptySpace,
    Galaxy,
}

type Coord = (i64, i64);

type Matrix = Vec<Vec<Cosmic>>;

type Network = Vec<Vec<i64>>;

struct Universe {
    expansion_rows: Vec<i64>,
    expansion_cols: Vec<i64>,
    galaxies: Vec<Coord>,
}

impl Universe {
    fn new(input: &str) -> Self {
        let mut matrix = Universe::parse_matrix_from_input(input);
        let (expansion_rows, expansion_cols) = Universe::cosmic_expansion(&mut matrix);
        let galaxies = Universe::find_galaxies(&matrix);
        
        Universe { galaxies, expansion_rows, expansion_cols }
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

    fn cosmic_expansion(matrix: &mut Matrix) -> (Vec<i64>, Vec<i64>) {
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        // Scan for empty rows
        let mut empty_rows: Vec<i64> = Vec::new();
        for row_index in 0..row_count { 
            match matrix[row_index].iter().all(|&cosmic| cosmic == Cosmic::EmptySpace) {
                true => empty_rows.insert(0, row_index as i64),
                false => continue,
            }
        }

        // Scan for empty columns
        let mut empty_cols: Vec<i64> = Vec::new();
        for col_index in 0..col_count {
            let mut col: Vec<Cosmic> = Vec::new();
            for row_index in 0..row_count { 
                col.push(matrix[row_index][col_index])
            }

            match col.iter().all(|&cosmic| cosmic == Cosmic::EmptySpace) {
                true => empty_cols.insert(0, col_index as i64),
                false => continue,
            }
        }
      
      (empty_rows, empty_cols)
    }

    fn find_galaxies(matrix: &Matrix) -> Vec<Coord> {
        let mut galaxies: Vec<Coord> = Vec::new();
        for (row_index, row) in matrix.iter().enumerate() {
            row
                .iter()
                .enumerate()
                .filter(|(_, &cosmic)| cosmic == Cosmic::Galaxy)
                .for_each(|(col_index, _)| galaxies.push((col_index as i64, row_index as i64)));
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

    fn diff_galaxies(&self, origin: &Coord, destination: &Coord) -> i64 {
        // Same galaxy - zero steps away
        if origin == destination { return 0; };

        let (origin_x, origin_y) = origin;
        let (destination_x, destination_y) = destination;

        // Don't match previously visited galaxies
        if origin_y > destination_y { return -1; }; // Above
        if origin_y == destination_y && destination_x < origin_x  { return -1; }; // Same row, to the left

        let mut diff_x = i64::abs(destination_x - origin_x);
        let mut diff_y = i64::abs(destination_y - origin_y);

        // Add cosmic expansion to diffs
        let mut range_x = *origin_x..*destination_x;
        let mut range_y = *origin_y..*destination_y;

        // Use positive ranges
        if range_x.start > range_x.end { range_x = range_x.end..range_x.start; };
        if range_y.start > range_y.end { range_y = range_y.end..range_y.start; };

        for step in range_y {
          if self.expansion_rows.contains(&step) { 
            diff_x += 999_999;
          };
        };

        for step in range_x {
          if self.expansion_cols.contains(&step) {
            diff_y += 999_999; 
          };
        };

        let distance = diff_x + diff_y;

        distance
    }

}

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> i64 {
    let universe = Universe::new(input);
    let network = universe.build_network();
    let sum_shortest_paths: i64 = network
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
    fn it_solves_example_with_expansion_10() {
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
        let result = part2(input);
        assert_eq!(result, 1030);
    }
    #[test]
    fn it_solves_example_with_expansion_100() {
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
        let result = part2(input);
        assert_eq!(result, 8410);
    }
}