mod cube;

use std::fs;

use cube::{Collection, Position3D, Position4D, Position5D};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    // Part 1
    let result_1 = Collection::<Position3D>::from(&content).multi_step(6);
    println!(
        "Active 3D cubes after 6th cycle: {}",
        result_1.count_active()
    );

    // Part 2
    let result_2 = Collection::<Position4D>::from(&content).multi_step(6);
    println!(
        "Active 4D cubes after 6th cycle: {}",
        result_2.count_active()
    );

    let result_5d = Collection::<Position5D>::from(&content).multi_step(6);
    println!(
        "Active 5D cubes after 6th cycle: {}",
        result_5d.count_active()
    );

    Ok(())
}
