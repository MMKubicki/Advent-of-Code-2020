use itertools::Itertools;
use std::collections::HashMap;
use std::{fs, num};

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let mut joltages = parse_input(&content)?;
    joltages.push(0); // Voltage of outlet
    joltages.sort_unstable();
    joltages.push(joltages.last().unwrap() + 3); // Voltage of device

    // part 1
    let mul = get_product_differences(&joltages);
    println!(
        "Number of 1-jolt differences * number of 3-jolt differences: {}",
        mul
    );

    // part 2
    let count = find_chains(&joltages);
    println!("Total number of distinct ways: {}", count);

    Ok(())
}

fn parse_input(content: &str) -> Result<Vec<usize>, num::ParseIntError> {
    content.lines().map(str::parse).collect()
}

fn get_product_differences(input: &[usize]) -> usize {
    let diff = get_differences(input);
    diff[0] * diff[2]
}

fn get_differences(input: &[usize]) -> [usize; 3] {
    // sort list
    // get differences of each element with the previous one (skip first element)
    let input = input.iter().copied().sorted().collect::<Vec<_>>();

    let mut count = [0; 3];

    for idx in (0..input.len()).skip(1) {
        let diff = input[idx] - input[idx - 1];
        count[diff - 1] += 1;
    }

    count
}

fn find_chains(input: &[usize]) -> usize {
    // Idea:
    // go through input back to front -> WORKING_INPUT
    // target (value on path end) will be initialized with path value 1
    // every other with path value 0
    // iterate through WORKING_INPUT
    // for every value of it (CURR_VALUE) search the input for values that could reach it -> PREV_VALUES
    // add to the path value of PREV_VALUES the path value of CURR_VALUE
    // this way, by the time the old PREV_VALUE is CURR_VALUE, all larger values will have been CURR_VALUE and
    // therefore every path from the new CURR_VALUE to them will have been looked at
    // after going over whole input the initial node 0 should have a count of every possible path in its path value

    let mut storage = Storage::new();
    // sort it largest to smallest
    let input = input.iter().copied().sorted().rev().collect::<Vec<_>>();

    // end of all paths
    let target = *input.first().unwrap();
    // beginning of all paths
    let first = *input.last().unwrap();

    storage.set(target, 1);

    for value in input.iter() {
        input
            .iter()
            // filter for every node that could reach value
            .filter(|next| {
                let diff = value.wrapping_sub(**next);
                0 < diff && diff <= 3
            })
            // add path value of value to their path value
            .for_each(|v| {
                storage.add(*v, storage.get(*value));
            });
    }

    // path value at begin of all paths -> count of all paths
    storage.get(first)
}

/// Struct to abstract HashMap to make access ez
/// mostly just how to add to a field and initialize it if it is empty
struct Storage {
    intern: HashMap<usize, usize>,
}
impl Storage {
    fn new() -> Self {
        Self {
            intern: HashMap::new(),
        }
    }

    fn get(&self, key: usize) -> usize {
        match self.intern.get(&key) {
            Some(value) => *value,
            None => 0,
        }
    }

    fn set(&mut self, key: usize, value: usize) {
        self.intern.insert(key, value);
    }

    fn add(&mut self, key: usize, to_add: usize) {
        let init = self.get(key);
        self.set(key, init + to_add);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_chains() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22, 0];
        assert_eq!(find_chains(&input), 8)
    }
}
