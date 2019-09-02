use std::io::BufReader;

use rit_course_parser::{RecordReader, ClassRecord};

fn main() {
    let filename = std::env::args().nth(1).expect("should get filename");
    let mut file = std::fs::File::open(&filename).expect("should open file");

    let mut reader = RecordReader::new(BufReader::new(&mut file));
//    let mut string = String::new();
//    let _ = std::io::Read::read_to_string(&mut reader, &mut string);
//    println!("{}", string);

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .flexible(true)
        .from_reader(&mut reader);

    for result in csv_reader.deserialize() {
        let record: Result<ClassRecord, _> = result;
        println!("{:?}", record);
    }
}
