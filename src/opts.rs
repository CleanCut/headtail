use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct Opts {
    #[clap(help = "Read from a file instead of stdin")]
    pub filename: Option<String>,
    #[clap(
        help = "Number of first lines of a file to display",
        short = 'H',
        long = "head",
        default_value_t = 10
    )]
    pub head: usize,
    #[clap(
        help = "Number of last lines of a file to display",
        short = 'T',
        long = "tail",
        default_value_t = 10
    )]
    pub tail: usize,
    #[clap(
        help = "Wait for additional data to be appended to a file. Ignored if standard input is a pipe.",
        short = 'f',
        long = "follow"
    )]
    pub follow: bool,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
