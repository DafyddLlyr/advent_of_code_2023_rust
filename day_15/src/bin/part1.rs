fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> i32 {
    let steps: Vec<&str> = input.split(",").collect();
    let hashes: Vec<i32> = steps.iter().map(|step| hash(step)).collect();
    
    hashes.iter().sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part1(input);
        assert_eq!(result, 1320);
    }
}