  fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let mut points: Vec<i32> = Vec::new();
    let (results, cards) = parse_input(input);

    for (i, result) in results.iter().enumerate() {
      let mut match_count = 0;
      let mut card_points: i32 = 0;
      let card = &cards[i];
      for num in result {
        if card.contains(num) {
          match match_count {
            0 => card_points += 1,
            _ => card_points *= 2,
          };
          match_count += 1;
        }
      }
      points.push(card_points);
    };

    points.iter().sum()
}

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut results: Vec<Vec<i32>> = Vec::new();
    let mut cards: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
      let line = line.split(":").nth(1).expect("Unable to strip prefix from line");
      let parts: Vec<Vec<i32>> = line.split("|").map(|part| parse_card_and_result(part)).collect();
      results.push(parts[0].clone());
      cards.push(parts[1].clone());
    }

    (results, cards)
}

fn parse_card_and_result(input: &str) -> Vec<i32> {
  input
    .trim()
    .split_whitespace()
    .map(|r| r.parse::<i32>().expect("Unable to parse to number"))
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }
}