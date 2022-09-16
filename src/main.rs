use headtail::errors::Result;

use headtail::{headtail, opts::Opts};

fn main() -> Result<()> {
    let opts = Opts::parse_args();
    //println!("{opts:#?}");
    headtail(&opts)?;
    Ok(())
}
