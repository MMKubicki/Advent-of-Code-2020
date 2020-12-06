use std::collections::HashSet;
use std::fs;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let groups = get_groups(&content);

    // part 1
    let union_count = groups.iter().map(count_union).sum::<usize>();
    println!("Part 1 sum: {}", union_count);

    // part 2
    let intersection_count = groups.iter().map(count_intersection).sum::<usize>();
    println!("Part 2 sum: {}", intersection_count);

    Ok(())
}

fn get_groups(content: &str) -> Vec<Vec<HashSet<char>>> {
    // groups delimited by empty line -> \n\n split
    // answers of people in group one per line -> lines()
    // answers = string of chars -> collect as HashSet<char>
    // collect answers in group as Vec
    // collect groups as Vec

    content
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|answers| answers.chars().collect())
                .collect()
        })
        .collect()
}

fn count_union(set: &Vec<HashSet<char>>) -> usize {
    // use op_on_set with union of sets
    op_on_set(set, |acc, next_set| &acc | next_set)
}

fn count_intersection(set: &Vec<HashSet<char>>) -> usize {
    // use op_on_set with intersection of sets
    op_on_set(set, |acc, next_set| &acc & next_set)
}

fn op_on_set(
    set: &Vec<HashSet<char>>,
    op: impl Fn(HashSet<char>, &HashSet<char>) -> HashSet<char>,
) -> usize {
    //take clone of first element as accumulator and fold on it with op

    let mut iter = set.iter();
    iter.next()
        .map(|first| iter.fold(first.clone(), op).len())
        .unwrap_or(0)
}
