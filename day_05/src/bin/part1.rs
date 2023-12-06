use std::ops::Range;

type Seeds = Vec<i64>;

struct PlantingMap {
  destination: Range<i64>,
  source: Range<i64>,
}

type Almanac = Vec<Vec<PlantingMap>>;


fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i64 {
    let (seeds, almanac) = parse_input(input);
    let locations = traverse_almanac(seeds, almanac);
    let nearest_location =     locations.iter().min().expect("Unable to find min value").clone();

    nearest_location
}

fn parse_input(input: &str) -> (Seeds, Almanac) {
  let seeds: Seeds = input
    .lines()
    .nth(0)
    .expect("Unable to parse seeds")
    .split_whitespace()
    .filter_map(|s| s.parse::<i64>().ok())
    .collect();

  let almanac: Almanac = input
    // Break input on empty newlines
    .split("\n\n")
    // Break on newlines
    .map(|item| item.split("\n").collect())
    // Skip seeds line
    .skip(1)
    // Break almanac sections
    .map(|item: Vec<&str>| item
          .iter()
          // Skip text line
          .skip(1)
          // Parse string values to integers
          .map(|&str| str.split_whitespace().filter_map(|str| str.parse().ok()).collect())
          .map(|item: Vec<i64>| {
            let destination_val = item[0];
            let source_val = item[1];
            let range = item[2];
            PlantingMap { 
              source: source_val..source_val + range, 
              destination: destination_val..destination_val + range 
            }
          })
        .collect()
    )
    .collect();

  (seeds, almanac)
}

fn traverse_almanac(seeds: Seeds, almanac: Almanac) -> Vec<i64> {
  let locations: Vec<i64> = seeds.iter().map(|seed| {
    // Set initial value as default
    let mut current_value = seed.clone();

    // Traverse almanac tables
    almanac.iter().for_each(|table| {
      'table: for planting_map in table {
        // Check each planting map
        if planting_map.source.contains(&current_value) {
          // Map incoming value from source to destination
          let diff = current_value - planting_map.source.start;
          current_value = planting_map.destination.start + diff;
          break 'table;
        }
      }
    });

    current_value
  }).collect();

  locations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result, 35);
    }
}