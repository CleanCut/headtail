use clap::{App, Arg, Parser};

#[derive(Debug, Parser)]
pub struct Opts {
    /// Read from a file instead of stdin.
    pub filename: Option<String>,

    /// Number of first lines of a file to display.
    #[clap(short = 'H', long, default_value = "10")]
    pub head: usize,

    /// Number of last lines of a file to display.
    #[clap(short = 'T', long, default_value = "10")]
    pub tail: usize,

    /// Wait for additional data to be appended to a file. Ignored if standard input is a pipe.
    #[clap(short = 'f', long)]
    pub follow: bool,
}

impl Opts {
    pub fn parse_args() -> Self {
        Opts::parse()
    }
}
