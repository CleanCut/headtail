pub mod opts;

use std::{
    collections::VecDeque,
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
    let mut tail_buffer: VecDeque<String> = VecDeque::with_capacity(opts.tail + 1);
    let mut line_num = 0;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                for line in &tail_buffer {
                    let _ = writer.write(line.as_bytes())?;
                }
                let _ = writer.flush();
                break;
            }
            Ok(_) => {
                if opts.head > line_num {
                    line_num += 1;
                    let _ = writer.write(line.as_bytes())?;
                    let _ = writer.flush();
                } else {
                    tail_buffer.push_back(line);
                    if tail_buffer.len() > opts.tail {
                        tail_buffer.pop_front();
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    // Keep following(?)

    if opts.follow && opts.filename.is_some() {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {}
                Ok(_) => {
                    let _ = writer.write(line.as_bytes())?;
                    let _ = writer.flush();
                }
                Err(e) => return Err(e),
            }
            // TODO: Is this a busy loop? Do we need to sleep or something?
        }
    }

    Ok(())
}
