use indexmap::IndexMap;

#[derive(Debug)]
enum Operation {
  Equal,
  Dash
}

type Step = (String, i32, Operation, Option<i32>);

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("The answer for part 2 is: {}", output);
}   

fn part2(input: &str) -> i32 {
    let steps: Vec<Step> = parse_input(input);
    let boxes: Vec<IndexMap<String, i32>> = build_lightboxes(&steps);
    let result = calculate_focusing_power(&boxes);

    result
}

fn build_lightboxes(steps: &Vec<(String, i32, Operation, Option<i32>)>) -> Vec<IndexMap<String, i32>> {
    let mut boxes: Vec<IndexMap<String, i32>> = vec![IndexMap::new(); steps.len()];

    steps.iter().for_each(|(label, label_hash, operation, focal_length)| {
        match operation { 
            Operation::Dash => {
                let lightbox = &mut boxes[*label_hash as usize];
                lightbox.shift_remove(label);
            },
            Operation::Equal => {
                let lightbox = &mut boxes[*label_hash as usize];
                lightbox.insert(label.to_string(), focal_length.unwrap());
            },
        }
    });

    boxes
}

fn parse_input(input: &str) -> Vec<Step> {
    input.split(",").map(|step| {
        let split_step: Vec<&str> = step.split(|char| char == '=' || char == '-').collect();
        let label = split_step.get(0).unwrap().to_string();
        let label_hash = hash(&label);
        let operation = if step.contains(&"=") { Operation::Equal } else { Operation::Dash };
        let focal_length = split_step.get(1).and_then(|str| str.parse::<i32>().ok());

        (label, label_hash, operation, focal_length)
    }).collect()
}

fn hash(step: &str) -> i32 {
    let mut current_value = 0;
    step.chars().for_each(|char| {
        current_value += char as u32;
        current_value *= 17;
        current_value = current_value % 256;
    });

    current_value as i32
}

fn calculate_focusing_power(boxes: &Vec<IndexMap<String, i32>>) -> i32 {
    boxes.iter().enumerate().map(|(box_index, lightbox)| {
        match lightbox.is_empty() {
            true => 0,
            false => lightbox
                .iter()
                .enumerate()
                .map(|(index, (_, focal_length))| 
                    (box_index + 1) as i32 * (index + 1) as i32 * focal_length)
                .sum()
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part2(input);
        assert_eq!(result, 145);
    }
}