pub mod opts;

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, ErrorKind, Result, Write},
};

use opts::Opts;

pub fn headtail(opts: &Opts) -> Result<()> {
    // Read from a file or stdout
    let mut reader: Box<dyn BufRead> = if opts.filename.is_some() {
        let file = File::open(opts.filename.as_ref().unwrap())?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // Write to stdout
    // TODO: ...or a file
    let mut writer: Box<dyn Write> = Box::new(BufWriter::new(io::stdout()));

    let mut tail_buffer = VecDeque::with_capacity(opts.tail + 1);
    let mut line_num = 0;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if opts.head > line_num {
                    line_num += 1;
                    if let Err(e) = writer.write(line.as_bytes()) {
                        if e.kind() == ErrorKind::BrokenPipe {
                            return Ok(());
                        } else {
                            return Err(e);
                        }
                    }
                    let _ = writer.flush();
                } else {
                    tail_buffer.push_back(line);
                    if tail_buffer.len() > opts.tail {
                        tail_buffer.pop_front();
                    }
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
        }
    }

    for line in tail_buffer {
        if let Err(e) = writer.write(line.as_bytes()) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
        let _ = writer.flush();
    }

    if opts.follow {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {}
                Ok(_) => {
                    if let Err(e) = writer.write(line.as_bytes()) {
                        if e.kind() == ErrorKind::BrokenPipe {
                            return Ok(());
                        } else {
                            return Err(e);
                        }
                    }
                    let _ = writer.flush();
                }
                Err(e) => {
                    if e.kind() == ErrorKind::BrokenPipe {
                        return Ok(());
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(())
}
