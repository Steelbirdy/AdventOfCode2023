use once_cell::sync::Lazy;
use regex::Regex;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit))
        .map(|mut s| {
            let first = s.next().unwrap();
            let last = s.next_back().unwrap_or(first);
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap());
    static END_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new("^.*(one|two|three|four|five|six|seven|eight|nine)").unwrap());

    fn get_digit(s: &str) -> char {
        match s {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => unreachable!(),
        }
    }

    fn process_line(s: &str) -> u32 {
        let first_digit = s.find(|c: char| c.is_ascii_digit());
        let last_digit = s.rfind(|c: char| c.is_ascii_digit());
        let first_slice = &s[..first_digit.unwrap_or(s.len())];
        let last_slice = &s[last_digit.unwrap_or(0)..];

        let first_digit = match PATTERN.captures(first_slice) {
            Some(m) => Some(get_digit(m.get(1).unwrap().as_str())),
            None => first_digit.map(|i| s.chars().nth(i).unwrap()),
        }
        .map_or_else(String::default, |c| c.to_string());

        let last_digit = match END_PATTERN.captures(last_slice) {
            Some(m) => Some(get_digit(m.get(1).unwrap().as_str())),
            None => last_digit.map(|i| s.chars().nth(i).unwrap()),
        }
        .map_or_else(String::default, |c| c.to_string());

        format!("{first_digit}{last_digit}").parse().unwrap()
    }

    input.lines().map(process_line).sum()
}
