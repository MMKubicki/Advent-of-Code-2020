use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let numbers = parse_input(&content)?;

    // part 1
    let wrong_number = find_first_wrong_number(&numbers, 25)
        .expect("There should be one if task/input is correct");
    println!("Wrong number: {}", wrong_number);

    // part 2
    let weakness = solve_encryption_weakness(&numbers, wrong_number);
    println!("Encryption weakness: {}", weakness);

    Ok(())
}

fn parse_input(content: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    content.lines().map(str::parse).collect()
}

// part 1

/// Find the first number in input that cannot be a result of an addition of two different numbers
/// in the last preamble_size numbers.
/// Returns None if there is none.
fn find_first_wrong_number(input: &[usize], preamble_size: usize) -> Option<usize> {
    // iterate over indexes, skip the first preamble_size ones
    // check if for that index there is a sum in the last preamble_size ones.
    // if there is none, that is the searched for number
    (0..input.len())
        .skip(preamble_size)
        .find(|idx| !is_sum_in_previous_x(input, *idx, preamble_size))
        .map(|v| input[v])
}

/// Check if the number at position in input can be the result of two numbers from the x numbers
/// before position
fn is_sum_in_previous_x(input: &[usize], position: usize, x: usize) -> bool {
    let search_for = input[position];
    let previous_numbers = &input[(position - x)..(position)];

    // go through every number from position - x till second to last (sum always needs 2 different values)
    // combine that number with every number other number till position
    // the first pair that sums to number at position returns true
    // return false if there is no pair
    (0..(previous_numbers.len() - 1))
        .map(|v| {
            ((v + 1)..previous_numbers.len())
                .map(move |i| (previous_numbers[v], previous_numbers[i]))
        })
        .flatten()
        .any(|(a, b)| a + b == search_for)
}

// part 2

/// returns the sum of the smallest and largest number in the continuous range, that when summed up itself
/// results in the given number
fn solve_encryption_weakness(input: &[usize], number: usize) -> usize {
    let found_range = find_set_for_number(input, number);
    add_largest_and_smallest(found_range)
}

/// finds the continuous range that sums up to number
/// returns empty slice if nothing is found
fn find_set_for_number(input: &[usize], number: usize) -> &[usize] {
    // go through all indexes in input
    // starting from this pos + 3 (as we need at least 2 continuous values) take slices from first
    // to second and sum them. if they are equal to number we found the range
    for idx in 0..input.len() {
        for idy in (idx + 3)..input.len() {
            if input[idx..idy].iter().sum::<usize>() == number {
                return &input[idx..idy];
            }
        }
    }

    // if nothing found return empty slice
    &input[0..0]
}

/// sums together the largest and the smallest value of input
fn add_largest_and_smallest(input: &[usize]) -> usize {
    let mut sorted = input.to_owned();
    sorted.sort_unstable();

    let smallest = sorted.first().unwrap();
    let largest = sorted.last().unwrap();
    smallest + largest
}
