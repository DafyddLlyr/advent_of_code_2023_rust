const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
  let games = parse_games(input);
  let playable_games = games.iter().filter(|&game| is_playable(game));
  let sum_ids: i32 = playable_games.map(|game| game.id).sum();
  
  sum_ids
}

struct Game {
  id: i32,
  max_red: i32,
  max_green: i32,
  max_blue: i32,
}

fn parse_games(input: &str) -> Vec<Game> {
  let mut games: Vec<Game> = Vec::new();
  for line in input.lines() {
    let split_line: Vec<&str> = line.split(":").collect();
    let (id, hands) = (split_line[0], split_line[1].to_string());
    let id = id.replace("Game", "").trim().parse::<i32>().expect("Unable to parse id");

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
      id,
      max_red: if red_balls.is_empty() { 0 } else { red_balls.iter().max().unwrap().clone() },
      max_green: if green_balls.is_empty() { 0 } else { green_balls.iter().max().unwrap().clone() },
      max_blue: if blue_balls.is_empty() { 0 } else { blue_balls.iter().max().unwrap().clone() },
    });

  }
  games
}

fn is_playable(game: &Game) -> bool {
  if game.max_red > MAX_RED { return false; }
  if game.max_green > MAX_GREEN { return false; }
  if game.max_blue > MAX_BLUE { return false; }

  true
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
        assert_eq!(result, 8);
    }
}