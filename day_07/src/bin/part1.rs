use std::{collections::HashMap, cmp::Ordering};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
enum Card {
    A = 14, 
    K = 13, 
    Q = 12, 
    J = 11, 
    T = 10, 
    Nine = 9, 
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

type Bid = i32;

type Hand = Vec<Card>;

type Hands = Vec<(Hand, Bid, HandType)>;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("The answer for part 1 is: {}", output);
}   

fn part1(input: &str) -> usize {
    let mut hands: Hands = parse_input(input);
    hands.sort_by(|(cards_a, _, hand_type_a), (cards_b, _, hand_type_b) | {
        match hand_type_a.cmp(hand_type_b) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => compare_cards(cards_a, cards_b),
        }
    });
    let score: usize = hands
        .iter()
        .enumerate()
        // Multiply rank by bid
        .map(|(i, (_, bid, _))| (i + 1) * *bid as usize)
        .sum();
    
    score
}

fn compare_cards(a: &Vec<Card>, b: &Vec<Card>) -> Ordering {
    for (card_a, card_b) in a.iter().zip(b.iter()) {
        match card_a.cmp(card_b) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => continue,
        }
    }
    
    Ordering::Equal
}

fn parse_input(input: &str) -> Hands {
    let hands: Hands = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let hand = parse_hand(line);
            let bid = parse_bid(line);
            let hand_type = calculate_hand_type(&hand);
            (hand, bid, hand_type)
        })
        .collect();
    hands
}

fn parse_bid(line: &&str) -> i32 {
    let bid: Bid = line
        .split(" ")
        .nth(1)
        .expect("Unable to find bid")
        .parse::<i32>()
        .expect("Unable to parse bid");
    bid
}

fn parse_hand(line: &&str) -> Vec<Card> {
    let hand: Hand = line
        .split(" ")
        .nth(0)
        .expect("Unable to find hand")
        .chars()
        .map(char_to_card)
        .collect();
    hand
}

fn calculate_hand_type(hand: &Hand) -> HandType {
    let mut counts_map = HashMap::new();

    // Count instances of each card in a HashMap
    hand.iter().for_each(|card| { 
        counts_map.entry(card)
            .and_modify(|count| { *count += 1 })
            .or_insert(1); 
    });

    // Get counts and sort highest -> lowest
    let mut counts: Vec<i32> = counts_map.into_values().collect();
    counts.sort_by(|&a, &b| b.cmp(&a));

    // Pattern match card count -> HandType
    match counts[..] {
        [5] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, 1, ..] => HandType::OnePair,
        [1, ..] => HandType::HighCard,
        _ => panic!("Invalid hand, something went wrong!"),
    }
}

fn char_to_card(char: char) -> Card {
    match char {
        'A' => Card::A, 
        'K'  => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(input);
        assert_eq!(result, 6440);
    }
}