use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    // Part 1
    let (timestamp, busses) = parse_input_part_1(&content);
    let result_1 = part_1(timestamp, &busses);
    println!(
        "Take bus {} at {} (wait time: {}). Result: {}",
        result_1.bus_id,
        result_1.earliest_time,
        result_1.wait_time,
        result_1.bus_id * result_1.wait_time
    );

    // Part 2
    let busses = parse_input_part_2(&content);
    let result_2 = part_2(&busses);
    println!(
        "Earliest timestamp such that all busses depart at offsets matching their position: {}",
        result_2
    );

    Ok(())
}

// Part 1

fn parse_input_part_1(content: &str) -> (usize, Vec<usize>) {
    let mut lines = content.lines();
    let timestamp = lines
        .next()
        .expect("malformed input - timestamp")
        .parse()
        .expect("error parsing timestamp");
    let busses = lines
        .next()
        .expect("malformed input - bus")
        .split(',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    (timestamp, busses)
}

struct Part1Result {
    bus_id: usize,
    earliest_time: usize,
    wait_time: usize,
}

fn part_1(timestamp: usize, busses: &[usize]) -> Part1Result {
    let (id, wait_time) = busses
        .iter()
        .map(|bus| (bus, bus - (timestamp % bus)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    let earliest_time = timestamp + wait_time;

    Part1Result {
        bus_id: *id,
        earliest_time,
        wait_time,
    }
}

// Part 2
// https://www.reddit.com/r/rust/comments/kc5phc/advent_of_code_2020_day_13/gfob95b/

fn parse_input_part_2(content: &str) -> Vec<(usize, usize)> {
    content
        .lines()
        .nth(1)
        .expect("malformed input - bus")
        .split(',')
        .enumerate()
        .filter_map(|(idx, elem)| elem.parse::<usize>().ok().map(|v| (idx, v)))
        .collect()
}

fn inv_mod(x: isize, p: isize) -> isize {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

fn part_2(busses: &[(usize, usize)]) -> usize {
    let prod = busses.iter().map(|(_, val)| val).product::<usize>() as isize;

    busses
        .iter()
        .map(|&(pos, val)| (pos as isize, val as isize))
        .map(|(pos, val)| -pos * (prod / val) * inv_mod(prod / val, val))
        .sum::<isize>()
        .rem_euclid(prod) as usize
}
