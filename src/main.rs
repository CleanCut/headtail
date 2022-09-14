use std::io::Result;

use headtail::{headtail, opts::Opts};

fn main() -> Result<()> {
    let opts = Opts::parse_args();
    headtail(&opts)?;
    Ok(())
}
