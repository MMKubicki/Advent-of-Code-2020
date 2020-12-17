mod ticket;

use std::fs;

use ticket::Input;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let mut input_data = content.parse::<Input>()?;

    // Part 1
    let result_1 = input_data.part_1_result();
    println!("Ticket scanning error rate: {}", result_1);

    // Part 2
    input_data.cleanup_invalid_tickets();
    let result_2 = input_data.part_2_result();
    println!("Product of \"departure\" fields on my ticket: {}", result_2);

    Ok(())
}
