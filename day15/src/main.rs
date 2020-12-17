mod spoken_number;

use crate::spoken_number::SpokenNumber;
use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let numbers = content
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    // Part 1
    let result = SpokenNumber::new(numbers.clone()).nth(2020 - 1).unwrap();
    println!("2020th spoken number with start {:?}: {}", numbers, result);

    // Part 2
    let result = SpokenNumber::new(numbers.clone())
        .nth(30000000 - 1)
        .unwrap();
    println!(
        "30000000th spoken number with start {:?}: {}",
        numbers, result
    );

    Ok(())
}
