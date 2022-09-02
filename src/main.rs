use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Result},
};

use headtail::{args::Args, first_pass, follow};

fn main() -> Result<()> {
    let args = Args::parse();
    let writer = BufWriter::new(io::stdout());
    let writer2 = BufWriter::new(io::stdout());

    if args.filename.is_some() {
        let file = File::open(args.filename.as_ref().unwrap())?;
        first_pass(
            &args,
            Box::new(BufReader::new(file.try_clone().unwrap())),
            Box::new(writer),
        )?;

        if args.follow {
            follow(file, Box::new(writer2))?;
        }
    } else {
        first_pass(
            &args,
            Box::new(BufReader::new(io::stdin())),
            Box::new(writer),
        )?;
    }

    Ok(())
}
