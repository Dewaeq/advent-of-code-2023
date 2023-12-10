use std::collections::HashMap;

fn hand_type(hand: &str, j: bool) -> u32 {
    let chars = hand.chars().collect::<Vec<_>>();
    let mut counts = HashMap::new();

    for c in &chars {
        if let Some(x) = counts.get_mut(&c) {
            *x += 1
        } else {
            counts.insert(c, 1);
        }
    }

    let mut sizes = counts
        .iter()
        .filter(|(&k, _)| *k != 'J')
        .map(|(_, &v)| v)
        .collect::<Vec<_>>();

    sizes.sort_unstable();

    let jokers = if j {
        *counts.get(&'J').unwrap_or(&0)
    } else {
        0
    };

    // Five of a kind
    if chars.iter().all(|&x| x == chars[0])
        || chars
            .iter()
            .any(|x| *x != 'J' && *counts.get(x).unwrap() + jokers == 5)
    {
        return 7000_000;
    }

    // Four of a kind
    if chars.iter().any(|x| {
        *counts.get(x).unwrap() == 4 || (*x != 'J' && *counts.get(x).unwrap() + jokers == 4)
    }) {
        return 6000_000;
    }

    // Full house
    if chars.iter().any(|x| {
        (*counts.get(x).unwrap() == 3
            && chars.iter().any(|y| y != x && *counts.get(y).unwrap() == 2))
            || 5 - sizes.iter().rev().take(2).sum::<i32>() == jokers
    }) {
        return 5000_000;
    }

    // Three of a kind
    if chars.iter().any(|x| {
        (*counts.get(x).unwrap() == 3) || *x != 'J' && *counts.get(x).unwrap() + jokers == 3
    }) {
        return 4000_000;
    }

    // Two pair
    if chars.iter().any(|x| {
        (*counts.get(x).unwrap() == 2
            && chars.iter().any(|y| y != x && *counts.get(y).unwrap() == 2))
            || jokers == 2
            || (jokers == 1 && *counts.get(x).unwrap() == 2)
    }) {
        return 3000_000;
    }

    // One pair
    if jokers > 0 || chars.iter().any(|x| *counts.get(x).unwrap() == 2) {
        return 2000_000;
    }

    // High card
    1000_000
}

fn score_hand(hand: &str, j: bool) -> u32 {
    let chars = hand.chars().collect::<Vec<_>>();
    let mut score = hand_type(hand, j);

    const CARDS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    const J_CARDS: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    for (i, c) in chars.iter().enumerate() {
        score += if j {
            (13_u32).pow(4 - i as u32) * (12 - J_CARDS.iter().position(|x| x == c).unwrap()) as u32
        } else {
            (13_u32).pow(4 - i as u32) * (12 - CARDS.iter().position(|x| x == c).unwrap()) as u32
        };
    }

    score
}

pub fn part_one(input: &str) -> u32 {
    let mut a = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (score_hand(hand, false), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    a.sort_by_key(|x| x.0);

    a.iter()
        .enumerate()
        .map(|(i, e)| e.1 * (i as u32 + 1))
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut a = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();

            (score_hand(hand, true), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    a.sort_by_key(|x| x.0);

    a.iter()
        .enumerate()
        .map(|(i, e)| e.1 * (i as u32 + 1))
        .sum()
}
