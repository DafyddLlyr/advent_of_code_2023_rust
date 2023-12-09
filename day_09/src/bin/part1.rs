type Report = Vec<Vec<i32>>;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let report: Report = parse_input(input);
    let next_values = analyse_report(report);
    let result = next_values.iter().sum();
    
    result
}

fn parse_input(input: &str) -> Report {
   input
        .lines()
        .map(|line| line
            .split_whitespace()
            .filter_map(|str| str.parse::<i32>().ok())
            .collect())
        .collect()
}

fn analyse_report(report: Report) -> Vec<i32> {
    report.iter().map(|seq| calculate_next(seq)).collect()
}

fn calculate_next(seq: &Vec<i32>) -> i32 {
    let mut progressions: Vec<Vec<i32>> = Vec::new();
    let mut current_progression = calculate_progression(seq);
    progressions.push(current_progression.clone());
    let mut is_final_progression = false;

    // Travel "down" progressions
    while !is_final_progression {
        current_progression = calculate_progression(&current_progression);
        progressions.push(current_progression.clone());
        is_final_progression = check_is_final_progression(&current_progression);
    }

    // Track last number in progressions
    let last: Vec<i32> = progressions.iter().map(|p| p.last().unwrap().clone()).rev().collect();

    // Calculate next number in progressions
    let mut next: Vec<i32> = vec![0];

    // Iterate "back up" progressions
    for i in 0..last.len() - 1 {
        next.push(next[i] + last[i + 1]);
    }
    let next_in_seq = seq.last().unwrap() + next.last().unwrap();
    
    next_in_seq
}

fn calculate_progression(seq: &Vec<i32>) -> Vec<i32> {
    seq
        .windows(2)
        .map(|diff| diff[1] - diff[0])
        .collect()
}

fn check_is_final_progression(progression: &Vec<i32>) -> bool {
    progression[0] == 0 && progression[1] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(input);
        assert_eq!(result, 114);
    }
}