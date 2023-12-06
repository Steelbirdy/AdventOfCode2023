use prse::parse;

type Output = u64;
type Input = (String, String);

#[aoc_generator(day6)]
pub fn generate(input: &str) -> Input {
    parse!(input, "Time:{}\nDistance:{}")
}

fn total_distance(held_down: Output, total: Output) -> Output {
    (total - held_down) * held_down
}

fn solve(time: Output, distance: Output) -> Output {
    let a = time as f32 / 2.;
    let b = (a * a - distance as f32).sqrt();
    let lower = (1. + a - b).floor() as Output;
    let upper = (-1. + a + b).ceil() as Output;
    upper - lower + 1
}

regex!(NUM = r"\d+");

#[aoc(day6, part1)]
pub fn part1((times, distances): &Input) -> Output {
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
pub fn part2((time, distance): &Input) -> Output {
    let time = time.replace(' ', "").parse().unwrap();
    let distance = distance.replace(' ', "").parse().unwrap();
    solve(time, distance)
}
