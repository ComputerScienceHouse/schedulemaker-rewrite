use std::io::BufReader;

use rit_sis_parser::{
    dat::{Reader, Config},
    model::ClassRecord,
};
use rit_sis_parser::model::{MeetingRecord, InstructorRecord};

/// Constructs a command-line interface and argument parser.
fn app<'a, 'b>() -> clap::App<'a, 'b> {
    use clap::{App, Arg};
    App::new("rit-sis-parser")
        .setting(clap::AppSettings::ColorAlways)
        .setting(clap::AppSettings::ColoredHelp)
        .arg(Arg::with_name("classes")
            .long("--classes")
            .value_name("class file")
            .help("The data file describing classes")
            .takes_value(true))
        .arg(Arg::with_name("meetings")
            .long("--meetings")
            .value_name("meetings file")
            .help("The data file describing class meeting times")
            .takes_value(true))
        .arg(Arg::with_name("instructors")
            .long("--instructors")
            .value_name("instructors file")
            .help("The data file describing instructors assigned to classes")
            .takes_value(true))
}

fn main() {
    let matches = app().get_matches();
    let mut hit = false;

    macro_rules! read_data {
        ($arg:expr, $fields:expr, $record:ty) => {
            if let Some(filename) = matches.value_of($arg) {
                hit = true;
                let mut file = std::fs::File::open(&filename).expect("should open file");
                let mut reader = Reader::new(BufReader::new(&mut file), Config::new($fields, b'|'));
                let mut csv_reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .delimiter(b'|')
                    .flexible(true)
                    .from_reader(&mut reader);

                let results = csv_reader.deserialize::<$record>()
                    .filter_map(Result::ok);

                for record in results {
                    println!("{:?}", record);
                }
            }
        }
    };

    read_data!("classes", 24, ClassRecord);
    read_data!("meetings", 19, MeetingRecord);
    read_data!("instructors", 8, InstructorRecord);

    // If none of the other options were run, print the help menu
    if !hit {
        let _ = app().print_help();
    }
}
