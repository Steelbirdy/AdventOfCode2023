use prse::{Parse, parse};

type Input = String;
type Output = u32;

#[aoc_generator(day)]
pub fn generate(input: &str) -> Vec<Input> {
    input.lines().map(|s| Parse::from_str(s).unwrap()).collect()
}

#[aoc(day, part1)]
pub fn part1(input: &[Input]) -> Output {
    todo!()
}

#[aoc(day, part2)]
pub fn part2(input: &[Input]) -> Output {
    todo!()
}