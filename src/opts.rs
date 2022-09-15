use clap::{App, Arg};

#[derive(Debug)]
pub struct Opts {
    pub filename: Option<String>,
    pub head: usize,
    pub tail: usize,
    pub follow: bool,
}

impl Opts {
    pub fn parse_args() -> Self {
        let matches = App::new("headtail")
            .arg(Arg::with_name("filename").help("Read from a file instead of stdin"))
            .arg(
                Arg::with_name("head")
                    .short('H')
                    .long("head")
                    .takes_value(true)
                    .default_value("10")
                    .help("Number of first lines of a file to display"),
            )
            .arg(
                Arg::with_name("tail")
                    .short('T')
                    .long("tail")
                    .takes_value(true)
                    .default_value("10")
                    .help("Number of last lines of a file to display"),
            )
            .arg(
                Arg::with_name("follow")
                    .short('f')
                    .long("follow")
                    .help("Wait for additional data to be appended to a file. Ignored if standard input is a pipe."),
            )
            .get_matches();
        let filename = matches.value_of("filename").map(|f| f.to_string());
        let head = matches
            .value_of("head")
            .unwrap()
            .parse::<usize>()
            .expect("could not parse number for head"); // TODO: more graceful error handling
        let tail = matches
            .value_of("tail")
            .unwrap()
            .parse::<usize>()
            .expect("could not parse number for tail"); // TODO: more graceful error handling
        let follow = matches.is_present("follow");
        Self {
            filename,
            head,
            tail,
            follow,
        }
    }
}
