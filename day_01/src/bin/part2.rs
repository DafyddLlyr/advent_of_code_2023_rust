use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}

fn part2(input: &str) -> i32 {
    let mut calibration_values: Vec<i32> = vec![];

    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    for line in input.lines() {
        let mut first_and_last: Vec<&str> = Vec::new();
        let matches: Vec<(&str, usize)> = re.find_iter(line).map(|m| (m.as_str(), m.start())).collect();

        if matches.is_empty() { panic!("No matches found, something went wrong") }

        let (first, _) = matches.first().unwrap();
        first_and_last.push(first);
        
        // Check for overlapping matches in remaining substring
        // Regex doesn't pick up overlaps
        let (last, last_index) = matches.last().unwrap();
        let overlapping_match = re.find(&line[(last_index + 1)..]).map(|m| m.as_str());

        if overlapping_match.is_some() {
          first_and_last.push(overlapping_match.unwrap());
        } else {
          first_and_last.push(last);
        };

        // convert first and last to number
        first_and_last = first_and_last.iter().map(|&digit| {
          match digit {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => digit,
          }
        }).collect();

        let value: i32 = first_and_last.join("").parse::<i32>().expect("Failed to parse to i32");

        calibration_values.push(value);
    }
    
    calibration_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "two1nine
          eightwothree
          abcone2threexyz
          xtwone3four
          4nineeightseven2
          zoneight234
          7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn it_handle_overlapping_numbers() {
      let input = "five2jzsconeightm";
      let result = part2(input);
      assert_eq!(result, 58);
    }
}