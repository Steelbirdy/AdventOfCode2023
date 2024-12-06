use prse::Parse;
use std::ops::RangeInclusive;

#[derive(Parse)]
#[prse = "seeds: {seeds: :}\n\n{maps:\n\n:}"]
pub struct Day5Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Parse)]
#[prse = "{_input}-to-{_output} map:\n{lines:\n:}"]
struct Map {
    _input: String,
    _output: String,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct ProcessedMap {
    lines: Vec<ProcessedLine>,
}

impl ProcessedMap {
    fn map(&self, x: u64) -> u64 {
        self.lines.iter().find_map(|line| line.map(x)).unwrap_or(x)
    }

    fn unmap(&self, y: u64) -> u64 {
        self.lines
            .iter()
            .find_map(|line| line.unmap(y))
            .unwrap_or(y)
    }

    fn map_range(&self, r: RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
        match self
            .lines
            .iter()
            .find_map(|line| line.map(*r.start()).map(|start| (line, start)))
        {
            Some((line, start)) => line.map(*r.end()).map(|end| start..=end),
            None => Some(r),
        }
    }
}

#[derive(Parse)]
#[prse = "{dst_start} {src_start} {len}"]
struct Line {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug)]
struct ProcessedLine {
    src: RangeInclusive<u64>,
    dst: RangeInclusive<u64>,
}

impl ProcessedLine {
    fn new(line: &Line) -> Self {
        Self {
            src: line.src_start..=line.src_start + line.len - 1,
            dst: line.dst_start..=line.dst_start + line.len - 1,
        }
    }

    fn map(&self, x: u64) -> Option<u64> {
        if !self.src.contains(&x) {
            return None;
        }
        let index = x - self.src.start();
        Some(self.dst.clone().nth(index as _).unwrap())
    }

    fn unmap(&self, y: u64) -> Option<u64> {
        if !self.dst.contains(&y) {
            return None;
        }
        let index = y - self.dst.start();
        Some(self.src.clone().nth(index as _).unwrap())
    }
}

#[aoc_generator(day5)]
pub fn generate_part1(input: &str) -> Day5Input {
    Parse::from_str(input).unwrap()
}

fn generate_input_critical_points(max: u64, map: &ProcessedMap) -> Vec<u64> {
    let min_found = map.lines.iter().map(|line| line.src.start()).min().unwrap();
    let max_found = map.lines.iter().map(|line| line.src.end()).max().unwrap();

    let mut ret = vec![
        0,
        min_found.saturating_sub(1),
        (max_found + 1).min(max),
        max,
    ];
    for line in &map.lines {
        let input_range = line.src.clone();
        ret.push(*input_range.start());
        ret.push(*input_range.end());
    }

    ret.sort();
    ret.dedup();
    ret
}

fn generate_output_critical_points(max: u64, map: &ProcessedMap) -> Vec<u64> {
    let min_found = map.lines.iter().map(|line| line.dst.start()).min().unwrap();
    let max_found = map.lines.iter().map(|line| line.dst.end()).max().unwrap();

    let mut ret = vec![
        0,
        min_found.saturating_sub(1),
        (max_found + 1).min(max),
        max,
    ];
    for line in &map.lines {
        let output_range = line.dst.clone();
        ret.push(*output_range.start());
        ret.push(*output_range.end());
    }

    ret.sort();
    ret.dedup();
    ret
}

fn condense(max: u64, input: &ProcessedMap, output: &ProcessedMap) -> ProcessedMap {
    let mut input_cp = generate_input_critical_points(max, input);
    let output_cp = generate_input_critical_points(max, output);

    input_cp.extend(output_cp.into_iter().map(|x| input.unmap(x)));
    input_cp.sort();
    input_cp.dedup();

    let mut skip_next = false;
    let lines = input_cp
        .windows(2)
        .filter_map(|w| {
            if skip_next {
                skip_next = false;
                return None;
            }
            let range = w[0]..=w[1];
            let out_range = input.map_range(range.clone());
            let out_range = out_range.and_then(|range| output.map_range(range));

            match out_range {
                Some(out_range) => {
                    skip_next = true;
                    Some(ProcessedLine {
                        src: range,
                        dst: out_range,
                    })
                }
                None => {
                    let val = w[0];
                    let out_val = output.map(input.map(val));
                    Some(ProcessedLine {
                        src: val..=val,
                        dst: out_val..=out_val,
                    })
                }
            }
        })
        .collect();
    ProcessedMap { lines }
}

#[aoc(day5, part1)]
pub fn part1(input: &Day5Input) -> u64 {
    let maps: Vec<ProcessedMap> = input
        .maps
        .iter()
        .map(|map| ProcessedMap {
            lines: map.lines.iter().map(ProcessedLine::new).collect(),
        })
        .collect();

    let max_value = maps
        .iter()
        .flat_map(|mp| &mp.lines)
        .flat_map(|line| [*line.src.end(), *line.dst.end()])
        .max()
        .unwrap();

    let condensed = maps
        .into_iter()
        .reduce(|map1, map2| condense(max_value, &map1, &map2))
        .unwrap();

    input
        .seeds
        .iter()
        .map(|&seed| condensed.map(seed))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Day5Input) -> u64 {
    let maps: Vec<ProcessedMap> = input
        .maps
        .iter()
        .map(|map| ProcessedMap {
            lines: map.lines.iter().map(ProcessedLine::new).collect(),
        })
        .collect();

    let max_value = maps
        .iter()
        .flat_map(|mp| &mp.lines)
        .flat_map(|line| [*line.src.end(), *line.dst.end()])
        .max()
        .unwrap();

    let condensed = maps
        .into_iter()
        .reduce(|map1, map2| condense(max_value, &map1, &map2))
        .unwrap();

    let critical_points = generate_output_critical_points(max_value, &condensed);

    let seed_ranges: Vec<_> = input.seeds.chunks(2).map(|w| w[0]..w[0] + w[1]).collect();
    let is_seed = |x: u64| {
        let corresponding_input = condensed.unmap(x);
        seed_ranges
            .iter()
            .any(|range| range.contains(&corresponding_input))
    };

    critical_points
        .into_iter()
        .filter(|x| is_seed(*x))
        .min()
        .unwrap()
}
