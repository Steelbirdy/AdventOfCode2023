use prse::Parse;
use std::collections::HashMap;

type Input = Game;
type Output = u32;

#[derive(Parse, PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum Color {
    #[prse = "red"]
    Red,
    #[prse = "green"]
    Green,
    #[prse = "blue"]
    Blue,
}

#[derive(Parse)]
#[prse = "{num} {color}"]
struct Balls {
    num: u32,
    color: Color,
}

#[derive(Parse, PartialEq, Debug)]
#[prse = "Game {id}: {rest}"]
pub struct Game {
    id: u32,
    rest: String,
}

struct ParsedGame {
    id: u32,
    rounds: Vec<Vec<Balls>>,
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Input> {
    input.lines().map(|s| Parse::from_str(s).unwrap()).collect()
}

fn process_game(game: &Game) -> ParsedGame {
    let rounds = game
        .rest
        .split("; ")
        .map(|round| {
            round
                .split(", ")
                .map(|x| Balls::from_str(x).unwrap())
                .collect()
        })
        .collect();
    ParsedGame {
        id: game.id,
        rounds,
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[Input]) -> Output {
    let input: Vec<_> = input.iter().map(process_game).collect();
    let mut ret = 0;
    let mut map = HashMap::new();
    for game in input {
        for round in game.rounds {
            for Balls { num, color } in round {
                let value = map.entry(color).or_insert(num);
                *value = num.max(*value);
            }
        }
        if map.get(&Color::Red).is_some_and(|&x| x <= 12)
            && map.get(&Color::Green).is_some_and(|&x| x <= 13)
            && map.get(&Color::Blue).is_some_and(|&x| x <= 14)
        {
            ret += game.id;
        }
        map.clear();
    }
    ret
}

#[aoc(day2, part2)]
pub fn part2(input: &[Input]) -> Output {
    let input: Vec<_> = input.iter().map(process_game).collect();
    let mut ret = 0;
    let mut map = HashMap::new();
    for game in input {
        for round in game.rounds {
            for Balls { num, color } in round {
                let value = map.entry(color).or_insert(num);
                *value = num.max(*value);
            }
        }
        ret += map.get(&Color::Red).copied().unwrap_or(0)
            * map.get(&Color::Blue).copied().unwrap_or(0)
            * map.get(&Color::Green).copied().unwrap_or(0);
        map.clear();
    }
    ret
}
