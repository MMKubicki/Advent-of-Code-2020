mod boarding_pass;

use std::collections::HashSet;
use std::fs;

use boarding_pass::Seat;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let seats = get_seats(&content)?;

    // Part 1
    let highest_seat_id = seats.iter().max().unwrap_or(&0);
    println!("Highest seat id: {}", highest_seat_id);

    // Part 2
    let empty_seat = get_empty_seat(&seats);
    println!("Empty seat: {}", empty_seat);

    Ok(())
}

/// returns seat ids as sorted list
fn get_seats(content: &str) -> anyhow::Result<Vec<usize>> {
    // parse each line to Seat and map successful parses to seat id
    let mut seats = content
        .lines()
        .map(str::parse::<Seat>)
        .map(|parse_result| match parse_result {
            Ok(value) => Ok(value.get_id()),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<usize>, _>>()?;

    seats.sort_unstable();

    Ok(seats)
}

fn get_empty_seat(seats: &[usize]) -> usize {
    // take smallest and largest seat id
    // generate a hashset containing all ids from smallest to largest
    // get differences between given and generated ids. should result to ids missing in given list
    // check that ids next to missing id is given.

    let first = *seats.first().unwrap();
    let last = *seats.last().unwrap();

    let control_group = (first..last).collect::<HashSet<_>>();
    let seats = seats.iter().copied().collect::<HashSet<_>>();

    let empty_seat = control_group
        .difference(&seats)
        .filter_map(|v| {
            if seats.contains(&(v - 1)) && seats.contains(&(v + 1)) {
                Some(*v)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(empty_seat.len(), 1);

    empty_seat[0]
}
