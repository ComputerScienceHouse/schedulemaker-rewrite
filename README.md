# rit-sis-parser

A crate for parsing, storing, and querying RIT course information.

This is an extraction and rewrite of some code that currently lives
in the [ScheduleMaker] repository. Computer Science House has an
arrangement with the Student Info System (SIS) group where they
deliver `.DAT` files containing data about current courses, including
class codes, descriptions, etc. This crate parses the `.DAT` files
and provides an interface for interacting with that data in various
ways.

[ScheduleMaker]: https://github.com/computersciencehouse/schedulemaker

## Getting Started

Make sure you have the latest version of Rust installed, then:

```
cargo run -- path/to/data.dat
```

This will print all of the records that were parsed from the `.DAT` file.
