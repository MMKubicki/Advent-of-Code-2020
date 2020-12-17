mod ship;

use std::fs;

use ship::{error::ParseMovementDirectionError, MovementDirection, Part1Position, Part2Position};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let input = fs::read_to_string(options.input)?;

    let directions = get_movement_directions(&input)?;

    // Part 1
    let mut part_1_pos = Part1Position::default();
    part_1_pos += directions.iter();
    println!(
        "Part 1 distance from start to end: {}",
        part_1_pos.manhatten_distance()
    );

    // Part 2
    let mut part_2_pos = Part2Position::default();
    part_2_pos += directions.iter();
    println!(
        "Part 2 distance from start to end: {}",
        part_2_pos.manhatten_distance()
    );
    Ok(())
}

fn get_movement_directions(
    input: &str,
) -> Result<Vec<MovementDirection>, ParseMovementDirectionError> {
    input.lines().map(str::parse).collect()
}
