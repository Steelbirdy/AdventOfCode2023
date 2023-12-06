#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(char::is_ascii_digit);
            let first = chars.next().unwrap_or('0');
            let last = chars.next_back().unwrap_or(first);
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    regex!(PATTERN = "([0-9]|one|two|three|four|five|six|seven|eight|nine)");
    regex!(BACK_PATTERN = ".*([0-9]|one|two|three|four|five|six|seven|eight|nine)");

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
            _ => s.chars().next().unwrap(),
        }
    }

    input
        .lines()
        .map(|s| {
            let mut captures = PATTERN.captures_iter(s);
            let back_captures = BACK_PATTERN.captures_iter(s);

            let first_digit = captures
                .next()
                .and_then(|c| c.get(1))
                .map(|m| get_digit(m.as_str()))
                .unwrap();

            let last_digit = back_captures
                .last()
                .and_then(|c| c.get(1))
                .map(|m| get_digit(m.as_str()))
                .unwrap_or(first_digit);

            format!("{first_digit}{last_digit}").parse::<u32>().unwrap()
        })
        .sum()
}
