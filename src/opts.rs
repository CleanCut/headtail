use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
};

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

    /// Write output to file
    #[clap(short = 'o', long = "outfile")]
    pub outfile: Option<String>,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Stream to receive input from. Either the file passed, or stdin otherwise.
    pub fn input_stream(&self) -> std::io::Result<Box<dyn BufRead>> {
        let stream: Box<dyn BufRead> = match self.filename {
            Some(ref filename) => {
                let file = OpenOptions::new().read(true).open(filename)?;
                Box::new(BufReader::new(file))
            }
            None => Box::new(BufReader::new(std::io::stdin())),
        };
        Ok(stream)
    }

    /// Stream to write output to. Either the file passed, or stdout otherwise.
    pub fn output_stream(&self) -> std::io::Result<Box<dyn Write>> {
        let stream: Box<dyn Write> = match self.outfile {
            Some(ref filename) => {
                let file = OpenOptions::new().write(true).create(true).open(filename)?;
                Box::new(BufWriter::new(file))
            }
            None => Box::new(BufWriter::new(std::io::stdout())),
        };
        Ok(stream)
    }
}
