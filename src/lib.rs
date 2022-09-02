pub mod args;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Result, Write},
};

use args::Args;

pub fn first_pass(args: &Args, reader: Box<dyn BufRead>, mut writer: Box<dyn Write>) -> Result<()> {
    let mut tail_buffer = VecDeque::with_capacity(args.tail);
    for (num, line_result) in reader.lines().enumerate() {
        if let Err(e) = line_result {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            } else {
                return Err(e);
            }
        }
        let line = line_result.unwrap();
        if args.head > num {
            if let Err(e) = writer.write(format!("{line}\n").as_bytes()) {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(e);
            }
        } else {
            tail_buffer.push_back(line);
            if tail_buffer.len() > args.tail {
                tail_buffer.pop_front();
            }
        }
    }

    for line in tail_buffer {
        if let Err(e) = writer.write(format!("{line}\n").as_bytes()) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

pub fn follow(file: File, mut writer: Box<dyn Write>) -> Result<()> {
    loop {
        for line_result in BufReader::new(&file).lines() {
            match line_result {
                Ok(line) => {
                    if let Err(e) = writer.write(format!("{line}\n").as_bytes()) {
                        if e.kind() == ErrorKind::BrokenPipe {
                            return Ok(());
                        }
                        return Err(e);
                    }
                }
                Err(e) => {
                    if e.kind() == ErrorKind::BrokenPipe {
                        return Ok(());
                    }
                    return Err(e);
                }
            }
        }
    }
}
