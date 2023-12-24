use std::collections::HashMap;

type Coord = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    EmptySpace,
    ForwardMirror,
    BackMirror,
    UpDownSplitter,
    LeftRightSplitter,
}

struct Grid {
    rows: Vec<Vec<Tile>>,
    visited: HashMap<Coord, (Direction, i32)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| 
                line
                .chars()
                .map(Grid::char_to_grid_tile).collect())
            .collect();
        
        Grid { rows, visited: HashMap::new() }
    }

    fn char_to_grid_tile(char: char) -> Tile {
         match char {
            '.' => Tile::EmptySpace,
            '/' => Tile::ForwardMirror,
            '\\' => Tile::BackMirror,
            '|' => Tile::UpDownSplitter,
            '-' => Tile::LeftRightSplitter,
            _ => unreachable!(),
        }
    }

    fn get(&self, coord: Coord) -> Option<Tile> {
        let (x, y) = coord;
        let result = self.rows.get(y as usize)?.get(x as usize)?;
        Some(*result)
    }

    fn is_visited(&mut self, coord: Coord, incoming: &Direction) -> bool {
        self.visited
            .get(&coord)
            .is_some_and(|(direction, _)| direction == incoming)
    }

    fn visit(&mut self, coord: Coord, direction: &Direction) {
         self.visited.entry(coord)
            .and_modify(|(_, count)| { *count += 1 })
            .or_insert((*direction, 1)); 
    }

    fn get_next(&mut self, incoming: Direction, start: Coord) -> Option<(Coord, Direction)> {
        let start_tile = self.get(start);

        if let Some(tile) = start_tile {
            return match (tile, incoming) {
                (Tile::EmptySpace, Direction::Left) => self.step_left(start),
                (Tile::EmptySpace, Direction::Right) => self.step_right(start),
                (Tile::EmptySpace, Direction::Up) => self.step_up(start),
                (Tile::EmptySpace, Direction::Down) => self.step_down(start),
                // Character = /
                (Tile::ForwardMirror, Direction::Left) => self.step_down(start),
                (Tile::ForwardMirror, Direction::Right) => self.step_up(start),
                (Tile::ForwardMirror, Direction::Up) => self.step_right(start),
                (Tile::ForwardMirror, Direction::Down) => self.step_left(start),
                // Character = \
                (Tile::BackMirror, Direction::Left) => self.step_up(start),
                (Tile::BackMirror, Direction::Right) => self.step_down(start),
                (Tile::BackMirror, Direction::Up) => self.step_left(start),
                (Tile::BackMirror, Direction::Down) => self.step_right(start),
                // Character = |
                (Tile::UpDownSplitter, Direction::Left | Direction::Right) => {
                    let up = self.step_up(start);
                    if up.is_some() {
                        let (next, _) = up.unwrap();
                        track_beam_of_light(self, Direction::Up, next);
                    };
                    let down = self.step_down(start);
                    if down.is_some() {
                        let (next, _) = down.unwrap();
                        track_beam_of_light(self, Direction::Down, next);
                    };

                    None
                },
                (Tile::UpDownSplitter, Direction::Up) => self.step_up(start),
                (Tile::UpDownSplitter, Direction::Down) => self.step_down(start),
                // Character = -
                (Tile::LeftRightSplitter, Direction::Left) => self.step_left(start),
                (Tile::LeftRightSplitter, Direction::Right) => self.step_right(start),
                (Tile::LeftRightSplitter, Direction::Up | Direction::Down) => {
                    let left = self.step_left(start);
                    if left.is_some() {
                        let (next, _) = left.unwrap();
                        track_beam_of_light(self, Direction::Left, next);
                    };
                    let right = self.step_right(start);
                    if right.is_some() {
                        let (next, _) = right.unwrap();
                        track_beam_of_light(self, Direction::Right, next);
                    };
                    
                    None
                },
            }
        }

        None
    }

    fn step_right(&self, (start_x, start_y): Coord) -> Option<(Coord, Direction)> {
        let right = (start_x + 1, start_y);
        let step = match self.get(right) {
            Some(_) => Some(right),
            None => None,
        };
        step.map(|step| (step, Direction::Right))
    }

    fn step_left(&self, (start_x, start_y): Coord) -> Option<(Coord, Direction)> {
        match start_x.checked_sub(1) {
            Some(_) => (),
            None => return None,
        };

        let left = (start_x - 1, start_y);
        let step = match self.get(left) {
            Some(_) => Some(left),
            None => None,
        };
        step.map(|step| (step, Direction::Left))
    }

    fn step_up(&self, (start_x, start_y): Coord) -> Option<(Coord, Direction)> {
        match start_y.checked_sub(1) {
            Some(_) => (),
            None => return None,
        };
        
        let up = (start_x, start_y - 1);
        let step = match self.get(up) {
            Some(_) => Some(up),
            None => None,
        };
        step.map(|step| (step, Direction::Up))
    }

    fn step_down(&self, (start_x, start_y): Coord) -> Option<(Coord, Direction)> {
        let down = (start_x, start_y + 1);
        let step = match self.get(down) {
            Some(_) => Some(down),
            None => None,
        };
        step.map(|step| (step, Direction::Down))
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    track_beam_of_light(&mut grid, Direction::Right, (0,0));
    grid.visited.len()
}

fn track_beam_of_light(grid: &mut Grid, incoming: Direction, start: Coord) -> Option<(Coord, Direction)> {
    if grid.is_visited(start, &incoming) { return None ;};

    grid.visit(start, &incoming);
    match grid.get_next(incoming, start) {
        None => None,
        Some((next, outgoing)) => track_beam_of_light(grid, outgoing, next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        let result = part1(input);
        assert_eq!(result, 46);
    }
}