fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
  let games = parse_games(input);
  let game_power = games.iter().map(|game| calculate_game_power(&game)).sum();
  
  game_power
}

struct Game {
  max_red: i32,
  max_green: i32,
  max_blue: i32,
}

fn parse_games(input: &str) -> Vec<Game> {
  let mut games: Vec<Game> = Vec::new();
  for line in input.lines() {
    let split_line: Vec<&str> = line.split(":").collect();
    let (_, hands) = (split_line[0], split_line[1].to_string());

    let mut red_balls: Vec<i32> = Vec::new();
    let mut green_balls: Vec<i32> = Vec::new();
    let mut blue_balls: Vec<i32> = Vec::new();

    let hands: Vec<Vec<&str>> = hands.split(";").map(|game| game.split(", ").collect()).collect();
  
    for hand in hands {
      for ball_colour in hand {
        match ball_colour {
          red if red.ends_with("red") => 
            red_balls.push(ball_colour.replace(" red", "").trim().parse::<i32>().expect("Unable to parse to i32")),
          green if green.ends_with("green") => 
            green_balls.push(ball_colour.replace(" green", "").trim().parse::<i32>().expect("Unable to parse to i32")),
          blue if blue.ends_with("blue") => 
            blue_balls.push(ball_colour.replace(" blue", "").trim().parse::<i32>().expect("Unable to parse to i32")),
          _ => {
            println!("Ball colour: {}", ball_colour);
            panic!("Missing ball colour!");
          },
        }
      }
    }
  
    games.push(Game {
      max_red: if red_balls.is_empty() { 0 } else { red_balls.iter().max().unwrap().clone() },
      max_green: if green_balls.is_empty() { 0 } else { green_balls.iter().max().unwrap().clone() },
      max_blue: if blue_balls.is_empty() { 0 } else { blue_balls.iter().max().unwrap().clone() },
    });

  }
  games
}

fn calculate_game_power(game: &Game) -> i32 {
  game.max_red * game.max_green * game.max_blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
          Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
          Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
          Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
          Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 2286);
    }
}