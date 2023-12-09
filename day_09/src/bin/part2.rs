type Report = Vec<Vec<i32>>;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> i32 {
    let report: Report = parse_input(input);
    let prev_values = analyse_report(report);
    let result = prev_values.iter().sum();
    
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
    report.iter().map(|seq| calculate_prev(seq)).collect()
}

fn calculate_prev(seq: &Vec<i32>) -> i32 {
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

    // Track first number in progressions
    let first: Vec<i32> = progressions.iter().map(|p| p.first().unwrap().clone()).rev().collect();

    // Calculate prev number in progressions
    let mut prev: Vec<i32> = vec![0];

    // Iterate "back up" progressions
    for i in 0..first.len() - 1 {
        prev.push(first[i + 1] - prev[i]);
    }
    let prev_in_seq = seq.first().unwrap() - prev.last().unwrap();
    
    prev_in_seq
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
        let input = "10  13  16  21  30  45";
        let result = part2(input);
        assert_eq!(result, 5);
    }
}