use prse::Parse;
use std::collections::HashSet;

type Input = ParsedCard;

#[derive(Parse, Debug, Clone)]
#[prse = "Card {_number}: {winning} | {yours}"]
pub struct Card {
    _number: String,
    winning: String,
    yours: String,
}

pub struct ParsedCard {
    winning: Vec<u32>,
    yours: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|s| Card::from_str(s).unwrap())
        .map(|card| ParsedCard {
            winning: card
                .winning
                .split(' ')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            yours: card
                .yours
                .split(' ')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Input]) -> u64 {
    input
        .iter()
        .map(|card| {
            let winning: HashSet<_> = card.winning.iter().copied().collect();
            let yours: HashSet<_> = card.yours.iter().copied().collect();
            let won = winning.intersection(&yours).count();
            if won == 0 {
                0
            } else {
                1 << (won - 1) as u32
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Input]) -> u64 {
    let winning_numbers: Vec<_> = input
        .iter()
        .map(|card| {
            let winning: HashSet<_> = card.winning.iter().copied().collect();
            let yours: HashSet<_> = card.yours.iter().copied().collect();
            let won = winning.intersection(&yours).count();
            won
        })
        .collect();
    let len = winning_numbers.len();
    let mut card_counts = vec![1 as u64; len];
    let mut ret: u64 = 0;
    for (i, n) in winning_numbers.into_iter().enumerate() {
        let count = card_counts[i];
        ret += count;
        if n > 0 {
            for x in &mut card_counts[i + 1..=(i + n).min(len - 1)] {
                *x += count;
            }
        }
    }
    ret
}
