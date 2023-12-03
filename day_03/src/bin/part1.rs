use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let input_lines: Vec<&str> = input.lines().collect();
    let symbol_re = Regex::new(r"[^0-9.]").unwrap();
    let number_re = Regex::new(r"[0-9]+").unwrap();

    let mut parts: Vec<i32> = Vec::new();

    for (line_index, line) in input_lines.iter().enumerate() {
        let symbols = symbol_re.find_iter(line);
        let is_first_line = line_index > 0;
        let is_final_line = line_index < input_lines.len() - 1;

        for symbol in symbols.into_iter() {
            let symbol_index = symbol.start();
 
            if !is_first_line {
                let line_above = number_re.find_iter(input_lines[line_index - 1]);
                check_line_above(line_above, symbol_index, &mut parts);
            }
            
            let current_line = number_re.find_iter(input_lines[line_index]);
            check_current_line(current_line, symbol_index, &mut parts);

            if !is_final_line {
                let line_below = number_re.find_iter(input_lines[line_index + 1]);
                check_line_below(line_below, symbol_index, &mut parts);
            }
                
        }
    }
    parts.iter().sum()
}

fn check_line_above(line_above: regex::Matches<'_, '_>, symbol_index: usize, parts: &mut Vec<i32>) {
    line_above.for_each(|num| {
        let start = if num.start() == 0 { 0 } else { num.start() - 1 };
        let end = num.end() + 1;
        let range = start..end;
        if range.contains(&symbol_index) {
            let part = num.as_str().parse::<i32>().expect("Unable to part part to i32");
            parts.push(part)
        }
    })
}

fn check_current_line(current_line: regex::Matches<'_, '_>, symbol_index: usize, parts: &mut Vec<i32>) {
    current_line.for_each(|num| {
        let is_touching = num.end() == symbol_index || (num.start() > 0 && num.start() - 1 == symbol_index);
        if is_touching {
            let part = num.as_str().parse::<i32>().expect("Unable to part part to i32");
            parts.push(part)
        }
    })
}

fn check_line_below(line_below: regex::Matches<'_, '_>, symbol_index: usize, parts: &mut Vec<i32>) {
    line_below.for_each(|num| {
        let start = if num.start() == 0 { 0 } else { num.start() - 1 };
        let end = num.end() + 1;
        let range = start..end;
        if range.contains(&symbol_index) {
            let part = num.as_str().parse::<i32>().expect("Unable to part part to i32");
            parts.push(part)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}