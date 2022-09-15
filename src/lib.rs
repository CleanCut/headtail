pub mod opts;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Result, Write},
};

use opts::Opts;

pub fn headtail(opts: &Opts) -> Result<()> {
    // Way to read from a file or stdout
    let mut reader: Box<dyn BufRead> = if opts.filename.is_some() {
        let file = File::open(opts.filename.as_ref().unwrap())?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // Way to write to stdout (and maybe in the future to a file)
    let mut writer: Box<dyn Write> = Box::new(BufWriter::new(io::stdout()));

    // Do our head/tail thing

    // Keep following(?)
    Ok(())
}
