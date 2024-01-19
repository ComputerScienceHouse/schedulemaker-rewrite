//! Provides tools for converting a DAT data stream to proper CSV.
//!
//! The DAT files given by SIS are very similar to CSV in that they're tabular
//! data which have fields separated by delimiters. The difference is that
//! the DAT files for whatever reason allow newlines to appear _inside_ of
//! a field value. This means that we can't just split rows by newlines.
//! Instead, we have to know the number of fields in advance and split
//! records by counting the fields as we encounter them.

use std::io::{BufRead, Cursor, Read};

/// Given a reader to a DAT file, provides a Read interface that's CSV formatted.
pub struct Reader<R> {
    /// A reader of the source DAT file
    reader: R,
    /// Config options for reading the DAT file
    config: Config,
    /// While traversing fields, keep the current field number
    current_field: u32,
    /// Whether to push a newline. Prevents trailing newline.
    push_newline: bool,
    /// A buffer that collects data belonging to the current record.
    line_buffer: Vec<u8>,
    /// A cursor to the well-formatted CSV record to output. This is used
    /// as the output reader interface to be consumed by clients.
    record_cursor: Cursor<String>,
}

/// Configuration options for the DAT reader.
pub struct Config {
    /// The number of fields that belong to the DAT record format.
    pub field_count: u32,
    /// The delimiter used in the DAT file.
    pub delimiter: u8,
}

impl Config {
    pub fn new(field_count: u32, delimiter: u8) -> Self {
        Config {
            field_count,
            delimiter,
        }
    }
}

impl<R: BufRead> Reader<R> {
    /// Construct a new Reader from a Read object to a DAT file and a config.
    pub fn new(reader: R, config: Config) -> Self {
        Reader {
            reader,
            config,
            current_field: 0,
            push_newline: false,
            line_buffer: Vec::new(),
            record_cursor: Cursor::new(String::new()),
        }
    }
}

impl<R: BufRead> Read for Reader<R> {
    /// Lazily read bytes from the DAT reader and convert them to CSV.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        // Read bytes from the buffered cursor
        let len = self.record_cursor.read(buf)?;
        if len != 0 {
            return Ok(len);
        }

        // If the buffered cursor is empty, read a record to fill it
        loop {
            // Read a field at a time by reading until each pipe
            let len = self
                .reader
                .read_until(self.config.delimiter, &mut self.line_buffer)?;
            if len == 0 {
                return Ok(0);
            }

            // Increment the field number we're visiting
            self.current_field += 1;

            // If this is the last field, move the record buffer into a cursor
            if self.current_field >= self.config.field_count {
                // Reset the field count for starting to read next record
                self.current_field = 0;

                // Swap out the internal buffer with a fresh empty one
                let buffer = std::mem::take(&mut self.line_buffer);

                // Convert the record buffer to a String and trim all the fields
                let record = String::from_utf8_lossy(&buffer)
                    .trim()
                    .replace('\n', "\\n")
                    .split(self.config.delimiter as char)
                    .map(|field| field.trim())
                    .collect::<Vec<&str>>()
                    .join(std::str::from_utf8(&[self.config.delimiter]).unwrap());

                let string = if self.push_newline {
                    format!("\n{}", record)
                } else {
                    self.push_newline = true;
                    record
                };

                // Put the now-formatted data into a Cursor to be read by clients
                let string_buffer = Cursor::new(string);
                self.record_cursor = string_buffer;
                return self.record_cursor.read(buf);
            }
        }
    }
}
