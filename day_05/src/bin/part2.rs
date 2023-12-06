use std::ops::Range;
use std::thread;

type SeedRanges = Vec<Range<i64>>;

#[derive(Clone)]
struct PlantingMap {
  destination: Range<i64>,
  source: Range<i64>,
}

type Almanac = Vec<Vec<PlantingMap>>;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> i64 {
    let (seed_ranges, almanac) = parse_input(input);
    let nearest_locations = traverse_seed_ranges(seed_ranges, almanac);
    let nearest_location = nearest_locations.iter().min().expect("Unable to find min value").clone();

    nearest_location
}

fn traverse_seed_ranges(seed_ranges: Vec<Range<i64>>, almanac: Vec<Vec<PlantingMap>>) -> Vec<i64> {
    let mut handles: Vec<thread::JoinHandle<i64>> = Vec::new();

    for range in seed_ranges {
      // Clone almanac for each thread
      let almanac_clone = almanac.clone();

      // Open a new thread for each range
      let handle = thread::spawn(|| {
        let seeds = range.into_iter().collect();
        let range_locations = traverse_almanac(seeds, almanac_clone);
        let range_nearest_location = range_locations.iter().min().expect("Unable to find min value").clone();
  
        range_nearest_location
      });

      handles.push(handle);
    }
    // Wait for all threads to complete, then join results
    let nearest_locations: Vec<i64> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();

    nearest_locations
}

fn parse_input(input: &str) -> (SeedRanges, Almanac) {
  let seed_ranges: SeedRanges = input
    .lines()
    .nth(0)
    .expect("Unable to parse seeds")
    .split_whitespace()
    .filter_map(|s| s.parse::<i64>().ok())
    .collect::<Vec<i64>>()
    .chunks(2)
    .map(|item| {
        let start = item[0];
        let range = item[1];

        start..start + range
    })
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

  (seed_ranges, almanac)
}

fn traverse_almanac(seeds: Vec<i64>, almanac: Almanac) -> Vec<i64> {
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
        let result = part2(input);
        assert_eq!(result, 46);
    }
}