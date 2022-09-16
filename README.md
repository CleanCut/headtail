# headtail

`head` and `tail` simultaneously

## Contributing

Contributions are welcome! Here are some [good first issues](https://github.com/CleanCut/headtail/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) you could look into.

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
