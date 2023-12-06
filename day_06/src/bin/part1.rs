use regex::Regex;
fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {  
    let races: Vec<(i32, i32)> = parse_input(input);
    let mut results: Vec<i32> = Vec::new();

    for (time, distance) in races {
        results.push(calculate_ways_to_win_race(time, distance));
    };
    
    results.iter().product()
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let times: &str = input.lines().collect::<Vec<&str>>()[0];
    let distances: &str = input.lines().collect::<Vec<&str>>()[1];

    let times: Vec<i32> = re
        .find_iter(times)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect();
    let distances: Vec<i32> = re
        .find_iter(distances)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect();

    let races: Vec<(i32, i32)> = (0..times.len())
        .into_iter()
        .map(|i| (times[i], distances[i]))
        .collect();

    races
}

fn calculate_ways_to_win_race(time: i32, total_distance: i32) -> i32 {
    let mut count: i32 = 0;
    (1..time - 1).for_each(|mps: i32| {
        let distance_covered = mps * (time - mps);
        if distance_covered > total_distance { count += 1; }
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result, 288);
    }
}