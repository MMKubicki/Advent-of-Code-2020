mod program;

use crate::program::{apply_list, parse_instruction_list, Memory};
use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let instructions = parse_instruction_list(&content)?;

    // part 1
    let mut memory = Memory::default();
    apply_list(&mut memory, Memory::apply_v1, &instructions);
    println!("V1: Sum of memory after completion: {}", memory.sum());

    // part 2
    let mut memory = Memory::default();
    apply_list(&mut memory, Memory::apply_v2, &instructions);
    println!("V2: Sum of memory after completion: {}", memory.sum());

    Ok(())
}
