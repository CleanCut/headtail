# headtail

`head` and `tail` simultaneously

TODO:
- [x] (Soon) Get a hello-world version published to crates.io to reserve the name
- [ ] (Maybe next Day-of-Learning?) Make the thing

## Quick Start

You need to [have Rust installed](https://www.rust-lang.org/tools/install).

```shell
# Install headtail
$ cargo install headtail

# Use it on a file
$ headtail somebigfile.txt

# Pipe stdout to it
$ somecommand | headtail

# Get the first 25 and last 5 lines of a file
$ headtail somebigfile.txt -h 25 -t 5

# Get the default amount of first lines, but only 3 last lines
$ headtail somebigfile.txt -t 3

# Do the default thing...but then keep tailing...
$ headtail somebigfile.txt -f
```

## Software License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
