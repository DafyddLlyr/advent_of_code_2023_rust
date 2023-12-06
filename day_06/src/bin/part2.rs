fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> i64 {  
    let (time, distance) = parse_input(input);
    let result = calculate_ways_to_win_race(time, distance);
    
    result
}

fn parse_input(input: &str) -> (i64, i64) {
    let time: String = input.lines().collect::<Vec<&str>>()[0].replace(" ", "");
    let distance: String = input.lines().collect::<Vec<&str>>()[1].replace(" ", "");

    let time: i64 = time
      .split(":")
      .skip(1)
      .find_map(|time| time.parse::<i64>().ok())
      .expect("Failed to parse time");

    let distance: i64 = distance
      .split(":")
      .skip(1)
      .find_map(|distance| distance.parse::<i64>().ok())
      .expect("Failed to parse distance");

    (time, distance)
}

fn calculate_ways_to_win_race(time: i64, total_distance: i64) -> i64 {
    let mut count: i64 = 0;
    (1..time - 1).for_each(|mps: i64| {
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
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}