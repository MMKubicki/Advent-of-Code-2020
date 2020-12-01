use clap::Clap;

/// Day 01 of Advent of Code 2020
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Michael Mario Kubicki <contact@michael-kubicki.de>"
)]
pub struct Opts {
    /// Path to input file
    pub input: String,
}

impl Opts {
    pub fn get() -> Self {
        Opts::parse()
    }
}
