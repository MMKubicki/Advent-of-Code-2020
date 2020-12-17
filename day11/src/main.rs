mod cells;

use std::fs;

use cells::{CellMap, CellState};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let initial_state = content.parse::<CellMap>()?;

    // Part 1
    let part_1_res = run_til_no_change(initial_state.clone(), CellMap::step_part_1);
    println!(
        "Part 1 Occupied seats: {}",
        part_1_res.count_in_state(CellState::OccupiedSeat)
    );

    // Part 2
    let part_2_res = run_til_no_change(initial_state, CellMap::step_part_2);
    println!(
        "Part 2 Occupied seats: {}",
        part_2_res.count_in_state(CellState::OccupiedSeat)
    );

    Ok(())
}

fn run_til_no_change(state: CellMap, step: impl Fn(&CellMap) -> CellMap) -> CellMap {
    let mut prev_state;
    let mut next_state = state;
    loop {
        prev_state = next_state;
        next_state = step(&prev_state);

        if prev_state == next_state {
            break;
        }
    }

    prev_state
}
