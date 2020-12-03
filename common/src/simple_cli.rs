use clap::Clap;

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
