# headtail

`head` and `tail` simultaneously

Easy Ways to Contribute!
- [ ] Set up CodeQL
- [ ] Set up Dependabot
- [ ] Improve Readme
- [ ] Add integration test for -f/--follow
- [ ] Add integration test for -s/--sleep-interval
- [ ] Add (any) unit tests
- [ ] Add (any) benchmark tests (see [criterion](https://bheisler.github.io/criterion.rs/book/index.html))
- [ ] Add option to output to file
- [ ] Find out if this even works on Windows
- [ ] Make demo video for readme (put the video in a PR/issue comment and use the URL in the readme)

## Quick Start

You need to [have Rust installed](https://www.rust-lang.org/tools/install).

```shell
# Install latest *release* version of headtail
$ cargo install headtail

# Install local development version of headtail from inside the git repo
$ cargo install --path .
```

```
# Use it on a file - prints the first 10 and last 10 lines
$ headtail somebigfile.txt

# Pipe stdout to it - prints the first 10 and last 10 lines
$ somecommand | headtail

# Print the first 25 and last 5 lines of a file
$ headtail somebigfile.txt -H 25 -T 5

# Print the default amount of first lines, but only 3 last lines
$ headtail somebigfile.txt -T 3

# Do the default thing...but then keep tailing the file and print
# out anything new that is appended to it.
$ headtail somebigfile.txt -f
```

See `headtail -h` for a full list of command-line options.

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
