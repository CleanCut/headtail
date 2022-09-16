pub mod errors;
pub mod opts;

use std::{
    collections::VecDeque,
    io::{self, BufRead, ErrorKind, Write},
    time::Duration,
};

use errors::Result;
use opts::Opts;

fn careful_write(writer: &mut dyn Write, line: &str) -> io::Result<()> {
    if let Err(e) = writer.write(line.as_bytes()) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(());
        } else {
            return Err(e);
        }
    }
    Ok(())
}

pub fn headtail(opts: &Opts) -> Result<()> {
    let mut reader = opts.input_stream()?;
    let mut writer = opts.output_stream()?;

    // Do our head/tail thing
    let mut tail_buffer: VecDeque<String> = VecDeque::with_capacity(opts.tail + 1);
    let mut line_num = 0;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line)? {
            0 => {
                for tail_line in &tail_buffer {
                    careful_write(&mut writer, tail_line)?;
                }
                let _ = writer.flush();
                break;
            }
            _ => {
                if opts.head > line_num {
                    line_num += 1;
                    careful_write(&mut writer, &line)?;
                    let _ = writer.flush();
                } else {
                    tail_buffer.push_back(line);
                    if tail_buffer.len() > opts.tail {
                        tail_buffer.pop_front();
                    }
                }
            }
        }
    }

    // Keep following(?)

    if opts.follow && opts.filename.is_some() {
        let sleep_interval = Duration::from_secs_f64(opts.sleep_interval);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    // This is a busy loop, so add a little sleep to make it less CPU hungry
                    std::thread::sleep(sleep_interval);
                }
                Ok(_) => {
                    careful_write(&mut writer, &line)?;
                    let _ = writer.flush();
                }
                Err(e) => {
                    println!("The error is {:?}", e.kind());
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}
