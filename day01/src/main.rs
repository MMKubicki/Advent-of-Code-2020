mod cli;

use itertools::Itertools;
use std::fs;

fn main() -> anyhow::Result<()> {
    let options = cli::Opts::get();

    let content = get_contents(fs::read_to_string(options.input)?);

    print_result(&content, 2020, 2, "two");

    print_result(&content, 2020, 3, "three");

    Ok(())
}

fn get_contents(input: String) -> Vec<usize> {
    input
        .lines()
        .map(str::parse::<usize>)
        .enumerate()
        .map(|(count, value)| {
            if let Ok(value) = value {
                Some(value)
            } else {
                println!("Error parsing value at position {}: {:?}", count, value);
                None
            }
        })
        .filter_map(|x| x)
        .collect()
}

fn get_all_summed_to_x(vec: &[usize], x: usize, len: usize) -> Vec<Vec<usize>> {
    vec.iter()
        .map(usize::clone)
        .combinations(len)
        .map(|i| (i.clone(), i.iter().sum::<usize>()))
        .filter_map(|(i, sum)| if sum == x { Some(i) } else { None })
        .collect()
}

fn get_first_summed_to_x(vec: &[usize], x: usize, len: usize) -> Option<Vec<usize>> {
    get_all_summed_to_x(vec, x, len).into_iter().next()
}

fn print_result(content: &[usize], x: usize, len: usize, len_str: &str) {
    println!("{} items:", len_str);

    match get_first_summed_to_x(content, x, len) {
        Some(values) => println!(
            "found {} values: {:?}\nSum: {}\nProduct: {}",
            len_str,
            values,
            values.iter().sum::<usize>(),
            values.iter().product::<usize>(),
        ),
        None => println!("found no {} values", len_str),
    }

    println!()
}
