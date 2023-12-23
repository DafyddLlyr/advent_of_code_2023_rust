fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> usize {
    let mut result = 0;
    input
        .split("\n\n")
        .enumerate()
        .map(search_lava_field)
        .for_each(|(row, col)| {
            if row > 0 { result += row * 100; };
            if col > 0 { result += col; };
        });

    result
}

fn search_lava_field((i, field): (usize, &str)) -> (usize, usize) {
    let col_index = 0;

    let (rows, cols) = parse_field(field);

    println!("Checking lava field {}", i + 1);

    let (mut row_index, row_smudge_seen) = find_reflection(&rows, i);

    if !row_smudge_seen { 
        // Overwrite any non-smudge reflections
        row_index = 0;
        let (col_index, col_smudge_seen) = find_reflection(&cols, i); 
        if col_smudge_seen { println!("Smudge on col {}", col_index); } ;
    };

    match (row_index, col_index) {
        (0, 0) => println!("Field {} has no smudges\n", i + 1),
        _ => (),
    }

    (row_index, col_index)
}

fn find_reflection(input: &Vec<String>, field_index: usize) -> (usize, bool) {
    let mut reflection_index: usize = 0;
    let range = 0..input.len() - 1;
    let mut is_smudge_seen = false;

   'outer: for i in range {
        let current = &input[i];
        let next = &input[i + 1];

        match is_reflection(current, next) {
            // No reflection, skip ahead
            (false, false) => continue,
            // Perfect reflection - check outwards
            (true, false) => (),
            // Smudged reflection, toggle status and check outwards
            (false, true) => is_smudge_seen = true,
            // Unreachable
            (true, true) => unreachable!(),
        }

        // Work outwards to check
        reflection_index = i + 1;

        // Possible reflection
        if i == 0 { break 'outer; };

        let mut index_above = i - 1;
        let mut index_below = i + 2;

        while index_below < input.len() {
            let above = &input[index_above];
            let below = &input[index_below];

            match is_reflection(above, below) {
                // No reflection, break
                (false, false) => {
                    reflection_index = 0;
                    break;
                },
                // Perfect reflection, check outwards again
                (true, false) => {
                    if index_above == 0 || index_below == input.len() - 1 { 
                        match is_smudge_seen {
                            true => { break 'outer; },
                            false => { continue 'outer; },
                        }
                    }
                    index_above -= 1;
                    index_below += 1; 
                },
                // Smudged reflection, toggle status and check outwards
                (false, true) => match is_smudge_seen {
                    true => {
                        // Only matches on second smudge, break and move forwards
                        reflection_index = 0;
                        is_smudge_seen = false;
                        break 'outer;
                    },
                    false => {
                        is_smudge_seen = true;

                        if index_above == 0 { break 'outer; }
                        if index_below == input.len() - 1 { break 'outer; }

                        index_above -= 1;
                        index_below += 1; 
                    }
                }
                // Unreachable
                (true, true) => unreachable!(),
            }
        }  
    }

    (reflection_index, is_smudge_seen)
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

fn is_smudged_reflection(above: &str, below: &str) -> bool {
    let mut is_smudged = false;

    for i in 0..above.len() - 1 {
        let above_char = above.chars().nth(i).unwrap();
        let below_char = below.chars().nth(i).unwrap();

        // This character matches, check next...
        if  above_char == below_char { continue; };

        // Toggle above, check for match
        let mut smudged_above: String = String::from(above);
        smudged_above.replace_range(i..i + 1, &get_opposite(above_char));

        if smudged_above == below {
            is_smudged = true;
            break;
        }

        // Toggle below, check for match
        // XXX: This is never hit currently?
        let mut smudged_below: String = String::from(below);
        smudged_below.replace_range(i..i + 1, &get_opposite(below_char));

        if smudged_below == above {
            println!("{}", above);
            println!("{}", below);
            println!("Smudge index: {}\n", i);
            is_smudged = true;
            break;
        }
    }

    is_smudged
}

fn get_opposite(char: char) -> String {
    match char {
        '#' => '.'.to_string(),
        '.' => '#'.to_string(),
        _ => unreachable!(),
    }
}

fn is_reflection(above: &str, below: &str) -> (bool, bool) {
    let mut is_reflection = false;
    let mut is_smudged = false;

    if above == below { is_reflection = true; }
    else { is_smudged = is_smudged_reflection(above, below); };

    (is_reflection, is_smudged)
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
        let result = part2(input);
        assert_eq!(result, 300);
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
        let result = part2(input);
        assert_eq!(result, 100);
    }
}