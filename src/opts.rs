use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;

#[derive(Clone, Debug, clap::Parser)]
pub struct Opts {
    /// Read from a file instead of stdin
    pub filename: Option<String>,
    /// Number of first lines of a file to display
    #[arg(short = 'H', long, default_value_t = 10)]
    pub head: usize,
    /// Number of last lines of a file to display
    #[arg(short = 'T', long, default_value_t = 10)]
    pub tail: usize,
    /// Wait for additional data to be appended to a file. Ignored if standard
    /// input is a pipe. If a `notify`-compatible filesystem watcher is
    /// available, that will be used. If not, we will fall back to a polling
    /// watcher.
    #[arg(short, long)]
    pub follow: bool,
    /// When following a file, sleep this amount in seconds between polling for changes. Ignored if
    /// a `notify`-compatible watcher is available.
    #[arg(short, long, default_value_t = 0.025)]
    pub sleep_interval: f64,

    /// Write output to file
    #[arg(short, long)]
    pub outfile: Option<String>,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Stream to receive input from. Either the file passed, or stdin otherwise.
    pub fn input_stream(&self) -> std::io::Result<Box<dyn BufRead + Send>> {
        let stream: Box<dyn BufRead + Send> = match self.filename {
            Some(ref filename) => {
                let file = OpenOptions::new().read(true).open(filename)?;
                Box::new(BufReader::new(file))
            }
            None => Box::new(BufReader::new(std::io::stdin())),
        };
        Ok(stream)
    }

    /// Stream to write output to. Either the file passed, or stdout otherwise.
    pub fn output_stream(&self) -> std::io::Result<Box<dyn Write + Send>> {
        let stream: Box<dyn Write + Send> = match self.outfile {
            Some(ref filename) => {
                let file = OpenOptions::new().write(true).create(true).open(filename)?;
                Box::new(BufWriter::new(file))
            }
            None => Box::new(BufWriter::new(std::io::stdout())),
        };
        Ok(stream)
    }
}
