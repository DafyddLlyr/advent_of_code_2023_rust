#[derive(Clone, Copy, Debug)]
enum Item {
    RoundRock,
    SquareRock,
    EmptySpace
}

type Platform = Vec<Vec<Item>>;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let platform = parse_input(input);
    let rearranged = slide_north(&platform);
    let result = calculate_total_load(&rearranged);

    result
}

fn parse_input(input: &str) -> Platform {
    input
        .lines()
        .map(|line| line.chars().map(char_to_item).collect())
        .collect()
}

fn char_to_item(char: char) -> Item {
    match char {
        'O' => Item::RoundRock,
        '#' => Item::SquareRock,
        '.' => Item::EmptySpace,
        _ => unreachable!(),
    }
}

fn slide_north(platform: &Platform) -> Platform {
    let mut rearranged: Platform = platform.clone();

    platform
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(row_index, row)| row
            .iter()
            .enumerate()
            .for_each(|(item_index, item)| {
                match item {
                    Item::EmptySpace | Item::SquareRock => (),
                    Item::RoundRock => slide_rock_north(&mut rearranged, &row_index, &item_index)
                }
            })
        );
    
    rearranged
}

fn slide_rock_north(rearranged: &mut Platform, row_index: &usize, item_index: &usize) {
    let mut current_index = row_index.clone();

    while current_index > 0 {
        let item_above = rearranged.get(current_index - 1).unwrap()[*item_index];
        match item_above {
            Item::RoundRock | Item::SquareRock => break,
            Item::EmptySpace => {
                // Swap positions
                rearranged[current_index - 1][*item_index] = Item::RoundRock;
                rearranged[current_index][*item_index] = Item::EmptySpace;

                // Check row above
                current_index -= 1;
            },
        }   
    };
}

fn calculate_total_load(platform: &Platform) -> i32 {
    let weighting = platform.len() as i32;

    platform
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .map(|&item| match item {
                    Item::EmptySpace | Item::SquareRock => 0,
                    Item::RoundRock => weighting - row_index as i32,
                })
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let result = part1(input);
        assert_eq!(result, 136);
    }
}