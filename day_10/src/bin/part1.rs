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

type Coord = (usize, usize);

struct Maze {
    rows: Vec<Vec<Tile>>,
    start: Coord,
    prev: Coord,
    current: Coord,
}

impl Maze {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| 
                line
                .chars()
                .map(Maze::char_to_maze_tile).collect())
            .collect();
        let start = Maze::find_start(&rows);
        let mut maze = Maze { rows, start, prev: start, current: (0,0) };
        maze.set_first_step();

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

    fn find_start(rows: &Vec<Vec<Tile>>) -> Coord {
        for (row_index, row)  in rows.iter().enumerate() {
            let col_index = row.iter().position(|&tile| tile == Tile::Start);
            match col_index {
                None => continue,
                Some(col_index) => return (row_index, col_index),
            }
        }
        panic!("Start not found in input");
    }

    fn get(&self, coord: Coord) -> Option<Tile> {
        let (row, col) = coord;
        let result = self.rows.get(row)?.get(col)?;
        Some(*result)
    }

    fn set_first_step(&mut self) {
        let mut first_step: Option<Coord> = None;

        let (start_row, start_col) = self.start;
        let mut possible_directions: Vec<Coord> = Vec::new();
        
        match self.start {
            (start_row, _) if start_row >= 1 => {
                possible_directions.push((start_row - 1, start_col)); // Up
                possible_directions.push((start_row, start_col + 1)); // Right
            },
            (_, start_col) if start_col >= 1 => {
                possible_directions.push((start_row, start_col - 1)); // Left
                possible_directions.push((start_row + 1, start_col)); // Down
            },
            _ => unreachable!()
        }

        for direction in possible_directions {
            match self.get(direction) {
                None => continue,
                Some(result) => match result {
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
        let current_tile = self.get(self.current);
        let current_tile = match current_tile {
            None => panic!("Can't file tile"),
            Some(current_tile) => current_tile,
        };
        
        let (prev_row, prev_col) = self.prev;
        let (current_row, current_col) = self.current;
    
        let next = match current_tile {
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
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let mut maze: Maze = Maze::new(input);
    let mut step_count = 1;

    while !maze.back_to_start() {
        maze.walk();
        step_count += 1;
    }

    let farthers_point = step_count / 2;

    farthers_point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_simple_example() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_solves_more_complex_example() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let result = part1(input);
        assert_eq!(result, 8);
    }
}