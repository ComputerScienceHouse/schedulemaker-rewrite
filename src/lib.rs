use serde::Deserialize;
use std::io::{BufRead, Read, Cursor};

pub struct RecordReader<R> {
    reader: R,
    config: RecordReaderConfig,
    current_field: u32,
    push_newline: bool,
    line_buffer: Vec<u8>,
    record_cursor: Cursor<String>,
}

pub struct RecordReaderConfig {
    field_count: u32,
    delimiter: u8,
}

impl Default for RecordReaderConfig {
    fn default() -> Self {
        RecordReaderConfig {
            field_count: 24,
            delimiter: b'|',
        }
    }
}

impl<R: BufRead> RecordReader<R> {
    pub fn new(reader: R) -> Self {
        RecordReader {
            reader,
            config: Default::default(),
            current_field: 0,
            push_newline: false,
            line_buffer: Vec::new(),
            record_cursor: Cursor::new(String::new()),
        }
    }

    pub fn with_config(reader: R, config: RecordReaderConfig) -> Self {
        RecordReader {
            reader,
            config,
            current_field: 0,
            push_newline: false,
            line_buffer: Vec::new(),
            record_cursor: Cursor::new(String::new()),
        }
    }
}

impl<R: BufRead> Read for RecordReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        // Read bytes from the buffered cursor
        let len = self.record_cursor.read(buf)?;
        if len != 0 { return Ok(len); }

        // If the buffered cursor is empty, read a record to fill it
        loop {
            // Read a field at a time by reading until each pipe
            let len = self.reader.read_until(self.config.delimiter, &mut self.line_buffer)?;
            if len == 0 { return Ok(0); }

            self.current_field += 1;
            if self.current_field >= self.config.field_count {
                self.current_field = 0;
                let buffer = std::mem::replace(&mut self.line_buffer, Vec::new());
                let record = String::from_utf8_lossy(&buffer).trim().replace("\n", "\\n").to_string();

                let string = if self.push_newline {
                    format!("\n{}", record)
                } else {
                    self.push_newline = true;
                    record
                };

                let string_buffer = Cursor::new(string);
                std::mem::replace(&mut self.record_cursor, string_buffer);
                return self.record_cursor.read(buf);
            }
        }
    }
}

//#[derive(Debug, Deserialize)]
//pub struct ClassRecord {
//    course_id: u32,
//    course_offer_number: u32,
//    strm: u32,
//    session_code: String,
//    class_section: String,
//    subject: String,
//    catalog_number: String,
//    description: String,
//    topic: String,
//    class_number: u32,
//    ssr_component: String,
//    units: u32,
//    enrollment_status: char,
//    class_status: char,
//    class_type: char,
//    schedule_print: char,
//    enrollment_capacity: u32,
//    enrollment_total: u32,
//    institution: String,
//    academic_org: String,
//    academic_group: String,
//    academic_career: String,
//    instruction_mode: String,
//    course_description_long: String,
//}

#[derive(Debug, Deserialize)]
pub struct ClassRecord {
    course_id: u32,
    course_offer_number: String,
    strm: String,
    session_code: String,
    class_section: String,
    subject: String,
    catalog_number: String,
    description: String,
    topic: String,
    class_number: String,
    ssr_component: String,
    units: String,
    enrollment_status: String,
    class_status: String,
    class_type: String,
    schedule_print: String,
    enrollment_capacity: String,
    enrollment_total: String,
    institution: String,
    academic_org: String,
    academic_group: String,
    academic_career: String,
    instruction_mode: String,
    course_description_long: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
