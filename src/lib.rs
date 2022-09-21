pub mod errors;
pub mod opts;

use std::{
    collections::VecDeque,
    io::{BufRead, ErrorKind, Write},
    path::Path,
    time::Duration,
};

use errors::HeadTailError;
use log::trace;
use notify::{event::EventKind, Watcher};

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
                    trace!(target: "head line", "read line: {}", line.trim_end());
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
        // Use a channel to send lines read back to the main thread
        // TODO: 1024 is an arbitrary number. Let's benchmark different values.
        let (tx, rx) = crossbeam_channel::bounded::<Result<String, HeadTailError>>(1024);

        // If using a polling watcher, respect the `--sleep-interval` argument.
        let sleep_interval = Duration::from_secs_f64(opts.sleep_interval);
        let config = notify::Config::default().with_poll_interval(sleep_interval);

        // Setup the file watcher
        let opts2 = opts.clone(); // TODO: Refactor so we don't need to clone opts
        let mut watcher = notify::RecommendedWatcher::new(
            move |res: notify::Result<notify::Event>| {
                match res {
                    Ok(event) => {
                        match event.kind {
                            EventKind::Any => trace!("EventKind::Any encountered"),
                            EventKind::Modify(m) => {
                                // TODO: Should(can?) we handle the truncation of a file? On macOS
                                // file truncation shows up as an EventKind::Modify(Metadata(Any)),
                                // which seems like could apply to events other than truncation.
                                trace!(target: "following file", "modified: {:?}", m);
                                let mut line = String::new();
                                match reader.read_line(&mut line) {
                                    Ok(0) => {}
                                    Ok(_) => {
                                        // If the main thread has closed the channel, it will soon cause
                                        // us to exit cleanly, so we can ignore the error.
                                        let _ = tx.send(Ok(line));
                                    }
                                    Err(e) => {
                                        // Can ignore channel send error for the same reason as above...
                                        trace!(target: "following file", "normal read error");
                                        let _ = tx.send(Err(e.into()));
                                    }
                                }
                            }
                            EventKind::Create(_) => {
                                trace!(target: "following file", "detected possible file (re)creation");
                                // The file has been recreated, so we need to re-open the input stream,
                                // read *everything* that is in the new file, and resume tailing.
                                let result = opts2.input_stream();
                                match result {
                                    Ok(new_reader) => {
                                        trace!(target: "following file", "succeeded reopening file");
                                        reader = new_reader;
                                    }
                                    Err(e) => {
                                        if let ErrorKind::NotFound = e.kind() {
                                            trace!(target: "following file", "cannot find file...aborting reopen");
                                            return;
                                        }
                                        // Can ignore channel send error for the same reason as above...
                                        let _ = tx.send(Err(e.into()));
                                    }
                                }
                                loop {
                                    let mut line = String::new();
                                    match reader.read_line(&mut line) {
                                        Ok(0) => {
                                            trace!(target: "following file", "catchup done");
                                            break;
                                        }
                                        Ok(_) => {
                                            trace!(target: "following file", "catchup read line: {}", line.trim_end());
                                            // If the main thread has closed the channel, it will soon cause us to
                                            // exit cleanly, so we can ignore the error.
                                            let _ = tx.send(Ok(line));
                                        }
                                        Err(e) => {
                                            // Can ignore sending error for same reason as 👆🏻
                                            let _ = tx.send(Err(e.into()));
                                            break;
                                        }
                                    }
                                }
                            }
                            EventKind::Remove(r) => {
                                trace!(target: "following file", "file removed: {:?}", r)
                            }
                            // We are being explicit about the variants we are ignoring just in case we
                            // need to research them.
                            EventKind::Access(_) => {}
                            EventKind::Other => {
                                trace!(target: "following file", "EventKind::Other encountered")
                            }
                        };
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e.into()));
                    }
                }
            },
            config,
        )?;

        // TODO: Figure out what to do about the possibility of a race condition between the initial
        // headtail and the following. See https://github.com/CleanCut/headtail/pull/17/files#r973220630
        watcher.watch(
            Path::new(opts.filename.as_ref().unwrap()),
            notify::RecursiveMode::NonRecursive,
        )?;

        // Loop over the lines sent from the `notify` watcher over a channel. This will block the
        // main thread without sleeping.
        for result in rx {
            let line = result?;
            careful_write(&mut writer, &line)?;
            let _ = writer.flush();
        }
    }

    Ok(())
}
