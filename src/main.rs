use headtail::{errors::HeadTailError, headtail, opts::Opts};

fn main() -> Result<(), HeadTailError> {
    let opts = Opts::parse_args();
    //println!("{opts:#?}");
    headtail(&opts)?;
    Ok(())
}
