<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/CleanCut/headtail/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/CleanCut/headtail/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/CleanCut/headtail/compare/v0.0.0...v0.1.0
