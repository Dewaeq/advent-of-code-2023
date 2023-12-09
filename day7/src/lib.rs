use std::collections::HashMap;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn hand_type(hand: &str) -> u32 {
    let chars = hand.chars().collect::<Vec<_>>();
    let mut counts = HashMap::new();

    for c in &chars {
        if let Some(x) = counts.get_mut(&c) {
            *x += 1
        } else {
            counts.insert(c, 1);
        }
    }

    // Five of a kind
    if chars.iter().all(|&x| x == chars[0]) {
        return 7000_000;
    }

    // Four of a kind
    if chars.iter().any(|x| *counts.get(x).unwrap() == 4) {
        return 6000_000;
    }

    // Full house
    if chars.iter().any(|x| {
        *counts.get(x).unwrap() == 3 && chars.iter().any(|y| y != x && *counts.get(y).unwrap() == 2)
    }) {
        return 5000_000;
    }

    // Three of a kind
    if chars.iter().any(|x| {
        *counts.get(x).unwrap() == 3 && chars.iter().any(|y| y != x && *counts.get(y).unwrap() == 1)
    }) {
        return 4000_000;
    }

    // Two pair
    if chars.iter().any(|x| {
        *counts.get(x).unwrap() == 2 && chars.iter().any(|y| y != x && *counts.get(y).unwrap() == 2)
    }) {
        return 3000_000;
    }

    // One pair
    if chars.iter().any(|x| *counts.get(x).unwrap() == 2) {
        return 2000_000;
    }

    // High card
    1000_000
}

fn score_hand(hand: &str) -> u32 {
    let chars = hand.chars().collect::<Vec<_>>();
    let mut score = hand_type(hand);

    for (i, c) in chars.iter().enumerate() {
        score +=
            (13_u32).pow(4 - i as u32) * (12 - CARDS.iter().position(|x| x == c).unwrap()) as u32
    }

    score
}

pub fn part_one(input: &str) -> u32 {
    let mut a = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (score_hand(hand), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    a.sort_by_key(|x| x.0);

    a.iter()
        .enumerate()
        .map(|(i, e)| e.1 * (i as u32 + 1))
        .sum()
}
