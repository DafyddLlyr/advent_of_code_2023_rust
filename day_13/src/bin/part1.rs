use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> usize {
    let mut result = 0;
    input
        .split("\n\n")
        .map(search_lava_field)
        .for_each(|(row, col)| {
            if row > 0 { result += row * 100; };
            if col > 0 { result += col; };
        });

    result
}

fn search_lava_field(field: &str) -> (usize, usize) {
    let (rows, cols) = parse_field(field);

    let row_index = find_reflection(rows);
    let col_index = find_reflection(cols);

    (row_index, col_index)
}

fn find_reflection(input: Vec<String>) -> usize {
    let mut reflection_index: usize = 0;
    let range = 0..input.len() - 1;

   'outer: for i in range {
        let current = &input[i];
        let next = &input[i + 1];

        // Not a reflection
        if current != next { continue; };

        // Work outwards to check
        reflection_index = i + 1;

        // Possible reflection
        if i == 0 { break 'outer; };

        let mut index_above = i - 1;
        let mut index_below = i + 2;

        while index_below < input.len() {
            let above = &input[index_above];
            let below = &input[index_below];

            match above.cmp(&below) { 
                Ordering::Equal => {
                    if index_above == 0 { break 'outer; }
                    if index_below == input.len() - 1 { break 'outer; }

                    index_above -= 1;
                    index_below += 1; 
                },
                _ => {
                    reflection_index = 0;
                    break;
                },
            }
        }  
    }

    reflection_index
}

fn parse_field(input: &str) -> (Vec<String>, Vec<String>) {
    let rows: Vec<String> = input
        .split("\n\n")
        .flat_map(|block| block.lines().map(String::from))
        .collect();

    let mut columns: Vec<String> = Vec::new();

    let column_count = 0..rows[0].len();
    for i in column_count {
       let column = rows
        .iter()
        .filter_map(|row| row.chars().nth(i))
        .collect();

       columns.push(column);
    }

    (rows, columns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let result = part1(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_solves_example_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = part1(input);
        assert_eq!(result, 400);
    }
}