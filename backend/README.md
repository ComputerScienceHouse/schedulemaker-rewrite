# rit-sis-parser

A crate for parsing, storing, and querying RIT course information.

This is an extraction and rewrite of some code that currently lives
in the [ScheduleMaker] repository. Computer Science House has an
arrangement with the Student Info System (SIS) group where they
deliver `.dat` files containing data about current courses, including
class codes, descriptions, etc. This crate parses the `.dat` files
and provides an interface for interacting with that data in various
ways.

[ScheduleMaker]: https://github.com/computersciencehouse/schedulemaker

## Getting Started

Make sure you have the latest version of Rust installed, then you can
build and run the project like this to get a help message:

```
$ cargo run

rit-sis-parser
USAGE:
    rit-sis-parser [OPTIONS]
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
        --classes <class file>              The data file describing classes
        --instructors <instructors file>    The data file describing instructors assigned to classes
        --meetings <meetings file>          The data file describing class meeting times
```

You can point to various `.dat` files like this:

```
$ cargo run -- --classes ./path/to/cshclass.dat --instructors ./path/to/cshinstr.dat
```

This will print all of the records that were parsed from the `.dat` file.
