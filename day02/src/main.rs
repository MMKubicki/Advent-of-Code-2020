mod password;

use password::{Password, Requirement, RequirementAndPassword};
use std::convert::TryFrom;
use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let input = fs::read_to_string(options.input)?;

    let content = parse(&input);

    println!("Part 1 rules:");
    check_passwords(&content, password::check_password_part1);

    println!();

    println!("Part 2 rules:");
    check_passwords(&content, password::check_password_part2);

    Ok(())
}

fn parse(input: &str) -> Vec<(Requirement, Password)> {
    // parse each line for requirement and password
    // label them with line number
    // partition whether the parse succeeded or failed
    let (succeeded, fail): (Vec<_>, Vec<_>) = input
        .lines()
        .map(RequirementAndPassword::try_from)
        .enumerate()
        .partition(|(_, val)| val.is_ok());

    // ignore line number from succeeded & unwrap from Result
    let results = succeeded
        .into_iter()
        .map(|(_, v)| v.unwrap().deconstruct())
        .collect::<Vec<_>>();

    // just unwrap the failed
    let errors = fail
        .into_iter()
        .map(|(c, v)| (c, v.unwrap_err()))
        .collect::<Vec<_>>();

    // print out failed parses
    for (count, error) in errors {
        println!("Line {}. {:#?}", count + 1, error);
    }

    results
}

fn check_passwords(
    content: &[(Requirement, Password)],
    check: impl Fn(&(Requirement, Password)) -> bool,
) {
    let (valid, invalid): (Vec<_>, Vec<_>) = content.iter().map(check).partition(|v| *v);

    println!("{} passwords invalid", invalid.len());
    println!("{} passwords valid", valid.len());
}
