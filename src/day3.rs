use std::collections::HashMap;

type Input = Vec<char>;
type Output = u64;

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Vec<Input> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn is_special_character(ch: &char) -> bool {
    !ch.is_ascii_digit() && *ch != '.'
}

fn is_gear_character(ch: &char) -> bool {
    *ch == '*'
}

#[aoc(day3, part1)]
pub fn part1(input: &[Input]) -> Output {
    let mut ret = 0;
    let mut buf = String::new();
    let width = input[0].len();
    let height = input.len();
    for (y, line) in input.iter().enumerate() {
        let mut line = line.iter().copied().enumerate().peekable();
        while line.len() > 0 {
            while line.next_if(|(_, ch)| !ch.is_ascii_digit()).is_some() {}
            if line.len() == 0 {
                break;
            }
            let (start, first_digit) = line.next().unwrap();
            let mut end = start;
            buf.push(first_digit);
            while let Some((x, digit)) = line.next_if(|(_, ch)| ch.is_ascii_digit()) {
                end = x;
                buf.push(digit);
            }
            let num: Output = buf.parse().unwrap();
            buf.clear();
            let span = start.saturating_sub(1)..=(end + 1).min(width - 1);
            let is_part_number = (y != 0
                && input[y - 1][span.clone()]
                    .iter()
                    .any(|ch| !ch.is_ascii_digit() && *ch != '.'))
                || (y != height - 1 && input[y + 1][span].iter().any(is_special_character))
                || (start != 0 && is_special_character(&input[y][start - 1]))
                || (end != width - 1 && is_special_character(&input[y][end + 1]));
            if is_part_number {
                ret += num;
            }
        }
    }
    ret
}

#[aoc(day3, part2)]
pub fn part2(input: &[Input]) -> Output {
    let mut symbols: HashMap<_, Vec<Output>> = HashMap::new();
    let mut buf = String::new();
    let width = input[0].len();
    let height = input.len();
    for (y, line) in input.iter().enumerate() {
        let mut line = line.iter().copied().enumerate().peekable();
        while line.len() > 0 {
            while line.next_if(|(_, ch)| !ch.is_ascii_digit()).is_some() {}
            if line.len() == 0 {
                break;
            }
            let (start, first_digit) = line.next().unwrap();
            let mut end = start;
            buf.push(first_digit);
            while let Some((x, digit)) = line.next_if(|(_, ch)| ch.is_ascii_digit()) {
                end = x;
                buf.push(digit);
            }
            let num: Output = buf.parse().unwrap();
            buf.clear();
            let span = start.saturating_sub(1)..=(end + 1).min(width - 1);
            if y != 0 {
                for (x, ch) in input[y - 1][span.clone()].iter().enumerate() {
                    if is_gear_character(ch) {
                        symbols
                            .entry((x + *span.start(), y - 1))
                            .or_default()
                            .push(num);
                    }
                }
            }
            if y != height - 1 {
                for (x, ch) in input[y + 1][span.clone()].iter().enumerate() {
                    if is_gear_character(ch) {
                        symbols
                            .entry((x + *span.start(), y + 1))
                            .or_default()
                            .push(num);
                    }
                }
            }
            if start != 0 && is_gear_character(&input[y][start - 1]) {
                symbols.entry((start - 1, y)).or_default().push(num);
            }
            if end != width - 1 && is_gear_character(&input[y][end + 1]) {
                symbols.entry((end + 1, y)).or_default().push(num);
            }
        }
    }

    symbols
        .into_values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}
