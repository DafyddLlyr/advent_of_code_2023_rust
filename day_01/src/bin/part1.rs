fn main() {
    let input = include_str!("part1.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let mut calibration_values: Vec<i32> = vec![];

    for line in input.lines() {
        let mut first_and_last: Vec<String> = vec![];

        let numeric_chars: Vec<char> = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect();

        first_and_last.push(numeric_chars.first().expect("First not found").to_string());
        first_and_last.push(numeric_chars.last().expect("Last not found").to_string());

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
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }
}