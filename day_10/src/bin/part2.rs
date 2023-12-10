use std::fs::File;
use std::io::Write;

use geo::{Polygon, LineString, Contains, Point};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Copy, Clone, Debug)]
struct Space {
    tile: Tile,
    is_path: bool,
}

type Coord = (usize, usize);

struct Maze {
    rows: Vec<Vec<Space>>,
    start: Coord,
    prev: Coord,
    current: Coord,
    path: Vec<Coord>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| 
                line
                .chars()
                .map(|char| Space {
                    tile: Maze::char_to_maze_tile(char),
                    is_path: false,
                }).collect())
            .collect();
        let start = Maze::find_start(&rows);
        let mut maze = Maze { rows, start, prev: start, current: (0,0), path: Vec::new() };
        maze.set_first_step();

        // Track initial starting spaces
        maze.set_is_path(start);
        maze.set_is_path(maze.current);

        // Track path
        maze.path.push(start);
        maze.path.push(maze.current);

        maze
    }

    fn char_to_maze_tile(char: char) -> Tile {
        match char {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }

    fn find_start(rows: &Vec<Vec<Space>>) -> Coord {
        for (row_index, row)  in rows.iter().enumerate() {
            let col_index = row.iter().position(|&space| space.tile == Tile::Start);
            match col_index {
                None => continue,
                Some(col_index) => return (row_index, col_index),
            }
        }
        panic!("Start not found in input");
    }

    fn get(&self, coord: Coord) -> Option<Space> {
        let (row, col) = coord;
        let result = self.rows.get(row)?.get(col)?;
        Some(*result)
    }

    fn set_is_path(&mut self, coord: Coord) {
        let (row, col) = coord;
        self.rows[row][col].is_path = true;
    }

    fn set_first_step(&mut self) {
        let mut first_step: Option<Coord> = None;

        let (start_row, start_col) = self.start;
        let mut possible_directions: Vec<Coord> = Vec::new();
        
        // Avoid maze "walls"
        // Don't start in impossible direction
        if start_row >= 1 {
            possible_directions.push((start_row - 1, start_col)); // Up
            possible_directions.push((start_row, start_col + 1)); // Right
        };
        if start_col >= 1 {
            possible_directions.push((start_row, start_col - 1)); // Left
            possible_directions.push((start_row + 1, start_col)); // Down
        };

        for direction in possible_directions {
            match self.get(direction) {
                None => continue,
                Some(space) => match space.tile {
                    Tile::Ground => continue,
                    Tile::Start => unreachable!(),
                    _ => first_step = Some(direction),
                }
            }
        }

        self.current = match first_step {
            None => panic!("Failed to find first step"),
            Some(first_step) => first_step,
        };
    }

    fn walk(&mut self) -> Coord {
        let current_space = self.get(self.current);
        let current_space = match current_space {
            None => panic!("Can't file tile"),
            Some(current_tile) => current_tile,
        };
        
        let (prev_row, prev_col) = self.prev;
        let (current_row, current_col) = self.current;

        let next = match current_space.tile {
            Tile::NorthSouth => match prev_row {
                north if north == (current_row - 1) => self.step_south(),
                south if south == (current_row + 1) => self.step_north(),
                _ => panic!("Can't find previous NorthSouth"),
            },
            Tile::EastWest => match prev_col {
                east if east == (current_col + 1) => self.step_west(),
                west if west == (current_col - 1) => self.step_east(),
                _ => panic!("Can't find previous EastWest"),
            },
            Tile::NorthEast => match (prev_row, prev_col) {
                (north, _) if north == (current_row - 1) => self.step_east(),
                (_, east) if east == (current_col + 1) => self.step_north(),
                _ => panic!("Can't find previous NorthEast"),
            },
            Tile::NorthWest => match (prev_row, prev_col) {
                (north, _) if north == (current_row - 1) => self.step_west(),
                (_, west) if west == (current_col - 1) => self.step_north(),
                _ => panic!("Can't find previous NorthWest"),
            },
            Tile::SouthWest => match (prev_row, prev_col) {
                (south, _) if south == (current_row + 1) => self.step_west(),
                (_, west) if west == (current_col - 1) => self.step_south(),
                _ => panic!("Can't find previous SouthWest"),
            },
            Tile::SouthEast => match (prev_row, prev_col) {
                (south, _) if south == (current_row + 1) => self.step_east(),
                (_, east) if east == (current_col + 1) => self.step_south(),
                _ => panic!("Can't find previous SouthEast"),
            },
            Tile::Start => self.start,
            Tile::Ground => unreachable!(),
        };

        self.prev = self.current;
        self.current = next;

        self.set_is_path(next);
        self.path.push(next);

        next
    }

    fn back_to_start(&self) -> bool {
        self.start == self.current
    }

    fn step_north(&self) -> Coord {
        let (row, col) = self.current;
        (row - 1, col)
    }

    fn step_south(&self) -> Coord {
        let (row, col) = self.current;
        (row + 1, col)
    }

    fn step_east(&self) -> Coord {
        let (row, col) = self.current;
        (row, col + 1)
    }

    fn step_west(&self) -> Coord {
        let (row, col) = self.current;
        (row, col - 1)
    }
    
    #[allow(dead_code)]
    fn map_tile_to_str(tile: Tile) -> String {
        match tile {
            Tile::NorthSouth  => "|".to_string(),
            Tile::EastWest  => "-".to_string(),
            Tile::NorthEast  => "L".to_string(),
            Tile::NorthWest  => "J".to_string(),
            Tile::SouthWest  => "7".to_string(),
            Tile::SouthEast  => "F".to_string(),
            Tile::Start  => "S".to_string(),
            _ => unreachable!(),
        }
    }

     #[allow(dead_code)]
    fn print(&self) {
        let visualisation: Vec<String> = self.rows
            .iter()
            .map(|row| row.iter().map(|space| {
                match space.is_path {
                    true => Maze::map_tile_to_str(space.tile),
                    false => ".".to_string()
                }
            }).collect::<Vec<String>>().join(""))
            .collect();

        let path = "print.txt";
        let mut file = File::create(path).unwrap();

        for line in &visualisation {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }

    fn get_path_as_polygon(&self) -> Polygon {
        let path_as_points = self.path
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect::<Vec<(f64, f64)>>();

        Polygon::new(LineString::from(path_as_points), vec![])
    }

    fn get_count_spaces_enclosed_by_path(&self) -> i32 {
        let polygon = self.get_path_as_polygon();
        let mut count: i32 = 0;
        let width = self.rows[0].len();
        let height = self.rows.len();

        // Iterate over all positions
        // Skip exterior rows as these can't be enclosed
        for x in 1..height - 1 {
            for y in  1..width - 1 {
                let point = Point::new(x as f64, y as f64);
                if polygon.contains(&point) { count += 1; };
            }
        }

        count
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let mut maze: Maze = Maze::new(input);

    while !maze.back_to_start() { 
        maze.walk(); 
    };

    maze.get_count_spaces_enclosed_by_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_first_loop_example() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_solves_second_loop_example() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn it_solves_third_loop_example() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result = part1(input);
        assert_eq!(result, 10);
    }
}