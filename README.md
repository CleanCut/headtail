# headtail

`head` and `tail` simultaneously

TODO:
- [x] Get a hello-world version published to crates.io to reserve the name
- [x] Write integration tests. Run them with `cargo test`
- [x] Create cheat sheet so we don't get stuck when we do our collaboration session.
- [ ] Sep 15th 2022, Day-of-Learning: Make the thing!
  - [ ] Review the existing project, including the release scaffolding
  - [ ] Run the (failing) tests
  - [ ] Implement the utility
  - [ ] Publish a release

## Quick Start

You need to [have Rust installed](https://www.rust-lang.org/tools/install).

```shell
# Install headtail
$ cargo install headtail

# Use it on a file - prints the first 10 and last 10 lines
$ headtail somebigfile.txt

# Pipe stdout to it - prints the first 10 and last 10 lines
$ somecommand | headtail

# Print the first 25 and last 5 lines of a file
$ headtail somebigfile.txt -H 25 -T 5

# Print the default amount of first lines, but only 3 last lines
$ headtail somebigfile.txt -T 3

# Do the default thing...but then keep tailing the file and print out anything new appended to it
$ headtail somebigfile.txt -f
```

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
