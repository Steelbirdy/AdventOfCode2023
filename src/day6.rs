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

#[aoc(day6, part1)]
pub fn part1((times, distances): &Input) -> Output {
    regex!(NUM = r"(\d+)");

    let times = NUM.captures_iter(&times);
    let distances = NUM.captures_iter(&distances);
    let races = times.zip(distances);

    races
        .map(|(time, distance)| {
            let time = time.get(1).unwrap().as_str().parse::<Output>().unwrap();
            let distance = distance.get(1).unwrap().as_str().parse::<Output>().unwrap();
            (0..time)
                .filter(|&hold| total_distance(hold, time) > distance)
                .count() as Output
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2((time, distance): &Input) -> Output {
    let time = time.replace(" ", "").parse::<Output>().unwrap();
    let distance = distance.replace(" ", "").parse::<Output>().unwrap();
    (0..time)
        .filter(|&hold| total_distance(hold, time) > distance)
        .count() as Output
}
