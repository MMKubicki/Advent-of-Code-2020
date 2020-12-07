mod rules;

use std::fs;

use rules::{BagInfo, Rules};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let rules = Rules::from(content);

    let gold_bag = BagInfo::from("shiny gold");

    // part 1
    let can_contain = rules.can_contain(&gold_bag);
    println!(
        "Bags that can contain at least one {}: {}",
        gold_bag,
        can_contain.len()
    );

    // part 2
    let count = rules.get_contents_bag_count(&gold_bag);
    println!("Bags needed in a single {}: {}", gold_bag, count);

    Ok(())
}
