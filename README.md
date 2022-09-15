# headtail

`head` and `tail` simultaneously

TODO:
- [x] Get a hello-world version published to crates.io to reserve the name
- [x] Write integration tests. Run them with `cargo test`
- [x] Create cheat sheet so we don't get stuck when we do our collaboration session.
- [ ] Sep 15th 2022, Day-of-Learning: Make the thing!
  - [ ] Review the existing project, including the release scaffolding
  - [ ] Run the (failing) tests
  - [ ] Implement the utility, handling arguments for file name (optional, read from stdin otherwise), number of lines to head, number of lines to tail, and whether or not to follow (iff input is a file).
  - [ ] Publish a release to crates.io
- [ ] Parallel tasks folks could pick up for fun:
  - [ ] Set up GitHub Actions CI ([example from my rusty_engine project](https://github.com/CleanCut/rusty_engine/blob/main/.github/workflows/ci.yml))
  - [ ] Set up CodeQL
  - [ ] Set up Dependabot
  - [ ] Improve Readme
  - [ ] More integration tests
  - [ ] Benchmark tests (see [criterion](https://bheisler.github.io/criterion.rs/book/index.html))
- [ ] After initial implementation
  - [ ] Any `TODO`s
  - [ ] Option to output to file
  - [ ] Better error handling
  - [ ] Does this work on Windows?
  - [ ] Screencast video for readme (put the video in a PR/issue comment and use the URL in the readme)

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

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
