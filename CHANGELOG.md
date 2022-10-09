<!-- next-header -->
## [Unreleased] - ReleaseDate

### Improved

- We now run CI on Windows. `headtail` _appears_ to work on Windows. If you run into problems, please report them!

## [0.3.2] - 2022-09-22

### Improved

- Internal improvements to how we use clap

## [0.3.1] - 2022-09-21

### Added

- CI now runs on macOS in addition to Linux. Now we just need someone to help us [support Windows](https://github.com/CleanCut/headtail/issues/21)!

### Improved

- We now use a notify-based watcher (inotify on Linux, etc.) when available to avoid polling.
- Internal improvements.

## [0.3.0] - 2022-09-15

### Added

- CI builds are now available on pull requests (including caching, rustfmt, and clippy)

### Fixed

- Don't crash when writing to a broken pipe
- Reduce follow from a busy loop (100% CPU) to a user-configurable sleep interval that defaults to 25ms (0.2% CPU in local testing)

### Other

- Refactored where the input streams are created


## [0.2.0] - 2022-09-15

### New!

- Made the project! It supports some flags. Here's the help output:

```
headtail 

USAGE:
    headtail [OPTIONS] [FILENAME]

ARGS:
    <FILENAME>    Read from a file instead of stdin

OPTIONS:
    -f, --follow         Wait for additional data to be appended to a file. Ignored if standard
                         input is a pipe.
    -h, --help           Print help information
    -H, --head <HEAD>    Number of first lines of a file to display [default: 10]
    -T, --tail <TAIL>    Number of last lines of a file to display [default: 10]
```

## [0.1.0] - 2022-08-24

Placeholder release to reserve the name.

<!-- next-url -->
[Unreleased]: https://github.com/CleanCut/headtail/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/CleanCut/headtail/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/CleanCut/headtail/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/CleanCut/headtail/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/CleanCut/headtail/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/CleanCut/headtail/compare/v0.0.0...v0.1.0
