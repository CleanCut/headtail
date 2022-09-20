pub mod errors;
pub mod opts;

use std::{
    collections::VecDeque,
    io::{BufRead, ErrorKind, Write},
    path::Path,
    time::Duration,
};

use errors::HeadTailError;
use notify::{event::EventKind, Event, Watcher};

use opts::Opts;

fn careful_write(writer: &mut dyn Write, line: &str) -> Result<(), HeadTailError> {
    if let Err(e) = writer.write(line.as_bytes()) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(());
        } else {
            return Err(e.into());
        }
    }
    Ok(())
}

pub fn headtail(opts: &Opts) -> Result<(), HeadTailError> {
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

    // Keep following
    //
    // To avoid wasted CPU cycles, we can use a file system watcher (e.g.
    // `inotify(7)` on Linux).
    //
    // The `notify` crate provides several optimized file system watchers using
    // functionality built into operating systems. Should an optimized watcher
    // not be available, `notify` will default to a polling watcher.
    if opts.follow && opts.filename.is_some() {
        let mut line = String::new();

        // Use a channel for synchronization between the watcher and the main
        // thread.
        let (tx, rx) = std::sync::mpsc::channel();

        // If using a polling watcher, respect the `--sleep-interval` argument.
        let sleep_interval = Duration::from_secs_f64(opts.sleep_interval);
        let config = notify::Config::default().with_poll_interval(sleep_interval);

        // Setup the file watcher
        let mut watcher = notify::RecommendedWatcher::new(
            move |res: notify::Result<notify::Event>| {
                use EventKind::Modify;
                match res {
                    // TODO: This is probably the place to detect if a file was
                    // renamed/removed and a new one of the same name created
                    // (in a later PR)
                    Ok(event) if matches!(event.kind, Modify(_)) => {
                        line.clear();
                        match reader.read_line(&mut line) {
                            Ok(0) => {}
                            Ok(_) => {
                                match careful_write(&mut writer, &line) {
                                    Ok(_) => {}
                                    Err(e) => eprintln!("Write error: {:?}", e),
                                }
                                let _ = writer.flush();
                            }
                            Err(e) => {
                                eprintln!("The error is {:?}", e.kind());
                            }
                        }
                        tx.send(()).unwrap();
                    }
                    Ok(_) => {}
                    Err(e) => eprintln!("Watcher error: {:?}", e),
                }
            },
            config,
        )?;

        watcher.watch(
            Path::new(opts.filename.as_ref().unwrap()),
            notify::RecursiveMode::NonRecursive,
        )?;

        // Loop over the messages in the channel. This will block the main
        // thread without sleeping.
        //
        // TODO: use the channel to communicate errors with the watching thread,
        // and stop the process.
        for _ in rx {}
    }

    Ok(())
}
