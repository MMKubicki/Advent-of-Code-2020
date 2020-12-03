mod field;

use std::fs;

use field::Field;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let file_content = fs::read_to_string(options.input)?;

    let field = file_content.parse::<Field>()?;

    let result = [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|s| test_slope(&field, *s))
        .product::<usize>();

    println!("Product: {}", result);

    Ok(())
}

fn test_slope(field: &Field, slope: (usize, usize)) -> usize {
    let trees_hit = traverse_slope(field, slope.0, slope.1);
    println!(
        "With ({}, {}) slope: hit {} tree(s)",
        slope.0, slope.1, trees_hit
    );
    trees_hit
}

fn traverse_slope(field: &Field, delta_x: usize, delta_y: usize) -> usize {
    // start position
    let mut pos = Point(0usize, 0usize);
    // distance to bottom (last y position)
    let target = field.len() - 1;

    // tree counter
    let mut counter = 0usize;

    while pos.1 < target {
        pos += (delta_x, delta_y);

        if field.is_tree_at(pos.0, pos.1) {
            counter += 1;
        }
    }

    counter
}

#[derive(Debug)]
pub struct Point(usize, usize);

impl std::ops::AddAssign<(usize, usize)> for Point {
    fn add_assign(&mut self, (x, y): (usize, usize)) {
        self.0 += x;
        self.1 += y;
    }
}
