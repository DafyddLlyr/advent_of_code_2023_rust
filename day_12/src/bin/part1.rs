use std::collections::HashSet;
use rayon::prelude::*;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(i, line)| {
            println!("Line {} / 1000", i + 1);
            find_arrangements(line)
        })
        .sum()
}

fn find_arrangements(input: &str) -> usize {
    let (line, pattern) = parse_input(input);

    let pattern_len = pattern.len() as i32;
    let sum_pattern = pattern.clone().iter().sum::<i32>() + pattern_len - 1;

    // Only one valid arrangement
    if line.len() as i32 == sum_pattern { return 1; };

    let possible_re = build_possible_re_for_pattern(&pattern);
    let exact_re = build_exact_re_for_pattern(&pattern);

    let mut unique_arrangements: HashSet<String> = HashSet::new();
    search_line(&line, &possible_re, &exact_re, &mut unique_arrangements, 0);

    unique_arrangements.len()
}

fn search_line(line: &String, possible_re: &Regex, exact_re: &Regex, arrangements: &mut HashSet<String>, start_index: usize) {
    for (i, char) in line.chars().enumerate().skip(start_index) {
        match char {
            '.' | '#' => continue,
            '?' => {
                let possible_dot = &generate_possible(&line, i, '.');
                let possible_hash = &generate_possible(&line, i, '#');

                let dot_match = possible_re.is_match(possible_dot);
                let hash_match = possible_re.is_match(possible_hash);

                if !dot_match && !hash_match { break; };
                
                match dot_match {
                    true => {
                        if exact_re.is_match(possible_dot) {
                            arrangements.insert(possible_dot.clone());
                        }
                        search_line(possible_dot, possible_re, exact_re, arrangements, i);
                    },
                    false => (),
                }
                
                match hash_match {
                    true => {
                        if exact_re.is_match(possible_hash) {
                            arrangements.insert(possible_hash.clone());
                        }
                        search_line(possible_hash, possible_re, exact_re, arrangements, i);
                    },
                    false => (),
                }
            }
            _ => panic!("Invalid input!")
        }
    }
}

fn generate_possible(line: &String, i: usize, change_char: char) -> String {
    line
        .clone()
        .chars()
        .enumerate()
        .map(|(j, char)| 
            match char {
                '.' | '#' => char.to_string(),
                '?' => if j <= i { change_char.to_string() } else { char.to_string() },
                _ => panic!("Invalid input!")
            })
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}

fn build_possible_re_for_pattern(pattern: &Vec<i32>) -> Regex {
    let patterns = pattern
        .iter()
        .map(|num| format!(r"([\?|#]{{{}}})", num))
        .collect::<Vec<String>>();

    let start = r"^(\.|\?)*".to_owned();
    let end = r"(\.|\?)*$".to_owned();
    let pattern_string = patterns.join(r"([\?|\.])*");

    let re = start + &pattern_string + &end;

    Regex::new(&re).expect("Invalid regex")
}

fn build_exact_re_for_pattern(pattern: &Vec<i32>) -> Regex {
    let patterns = pattern
        .iter()
        .map(|num| format!(r"(#{{{}}})", num))
        .collect::<Vec<String>>();

    let start = r"^(\.*)".to_owned();
    let end = r"(\.*)$".to_owned();
    let pattern_string = patterns.join(r"(\.+)");

    let re = start + &pattern_string + &end;

    Regex::new(&re).expect("Invalid regex")
}

fn parse_input(input: &str) -> (String, Vec<i32>) {
    let line = input
        .split(" ")
        .nth(0)
        .expect("Unable to parse row")
        .to_owned();
    
    let pattern: Vec<i32> = input
        .split(" ")
        .nth(1)
        .expect("Unable to parse pattern")
        .split(",").filter_map(|str| str.parse::<i32>().ok())
        .collect();

    (line, pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_test_case_1() {
        let input = "???.### 1,1,3";
        let result = find_arrangements(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_solves_test_case_2() {
        let input = ".??..??...?##. 1,1,3";
        let result = find_arrangements(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_solves_test_case_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let result = find_arrangements(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_solves_test_case_4() {
        let input = "????.#...#... 4,1,1";
        let result = find_arrangements(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_solves_test_case_5() {
        let input = "????.######..#####. 1,6,5";
        let result = find_arrangements(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_solves_test_case_6() {
        let input = "?###???????? 3,2,1";
        let result = find_arrangements(input);
        assert_eq!(result, 10);
    }
}