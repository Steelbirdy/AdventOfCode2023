use prse::parse;

type Input = (String, String);

#[aoc_generator(day6)]
pub fn generate(input: &str) -> Input {
    parse!(input, "Time:{}\nDistance:{}")
}

fn solve(time: u64, distance: u64) -> u64 {
    let (time, distance) = (time as f64, distance as f64);
    let a = time / 2.;
    let b = (a * a - distance).sqrt();
    let lower = (1. + a - b).floor() as u64;
    let upper = (-1. + a + b).ceil() as u64;
    upper - lower + 1
}

regex!(NUM = r"\d+");

#[aoc(day6, part1)]
pub fn part1((times, distances): &Input) -> u64 {
    let times = NUM.find_iter(&times);
    let distances = NUM.find_iter(&distances);
    let races = times.zip(distances);

    races
        .map(|(time, distance)| {
            let time = time.as_str().parse().unwrap();
            let distance = distance.as_str().parse().unwrap();
            solve(time, distance)
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2((time, distance): &Input) -> u64 {
    let time = time.replace(' ', "").parse().unwrap();
    let distance = distance.replace(' ', "").parse().unwrap();
    solve(time, distance)
}
