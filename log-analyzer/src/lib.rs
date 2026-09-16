pub mod log_parsing;

use crate::log_parsing::log_entry::ParseLogError;
use log_parsing::log_entry::LogEntry;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
pub enum ErrorReadingFile {
    ErrorOpeningFile(Error),
    ErrorDuringFileReading,
}

impl PartialEq for ErrorReadingFile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ErrorOpeningFile(l0), Self::ErrorOpeningFile(r0)) => l0.kind() == r0.kind(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub fn parse_logs_from_file(
    path: std::path::PathBuf,
) -> Result<(Vec<LogEntry>, Vec<ParseLogError>), ErrorReadingFile> {
    let f = File::open(path).map_err(ErrorReadingFile::ErrorOpeningFile)?;
    let reader = BufReader::new(f);
    parse_logs_from_bufreader(reader)
}

fn parse_logs_from_bufreader(
    reader: impl BufRead,
) -> Result<(Vec<LogEntry>, Vec<ParseLogError>), ErrorReadingFile> {
    let mut logs = Vec::new();
    let mut errors = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|_| ErrorReadingFile::ErrorDuringFileReading)?;

        match LogEntry::try_from(line.as_str()) {
            Ok(log) => logs.push(log),
            Err(error) => errors.push(error),
        }
    }
    Ok((logs, errors))
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Error, ErrorKind, Read};

    use super::*;

    #[test]
    fn test_file_exists() {
        let log_entries = parse_logs_from_file(std::path::PathBuf::from("tests/data/example.logs"));
        assert!(log_entries.is_ok());
        let (logs, errors) = log_entries.unwrap();
        assert_eq!(logs.len(), 3);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_file_not_exists() {
        let log_entries =
            parse_logs_from_file(std::path::PathBuf::from("some/path/not/exists/file.txt"));
        assert!(log_entries.is_err());
        let err = log_entries.unwrap_err();
        assert_eq!(
            err,
            ErrorReadingFile::ErrorOpeningFile(Error::new(ErrorKind::NotFound, ""))
        );
    }

    #[test]
    fn test_error_during_file_reading() {
        // Whole implementation of custom buf_reader which supposed to fail when reading line "fail_at".
        // Custom implementation starts here
        #[derive(Default)]
        struct MyBuffReader {
            buf_reader: Cursor<String>,
            fail_at: u8,
            current: u8,
        }

        impl Read for MyBuffReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                self.buf_reader.read(buf)
            }
        }

        impl BufRead for MyBuffReader {
            fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
                self.buf_reader.fill_buf()
            }

            fn consume(&mut self, amount: usize) {
                self.buf_reader.consume(amount);
            }

            fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
                if self.current < self.fail_at {
                    self.current += 1;
                    return self.buf_reader.read_line(buf);
                }
                Err(Error::other("Simulate error"))
            }
        }
        // Custom implementation ends here. Below is the actual test
        let buf_reader = MyBuffReader {
            buf_reader: Cursor::new(String::from(
                "2026-09-11T10:01:03Z INFO server started\n2026-09-11T10:01:03Z INFO server started\n2026-09-11T10:01:03Z INFO server started",
            )),
            fail_at: 1,
            ..Default::default()
        };
        let log_entries = parse_logs_from_bufreader(buf_reader);
        assert!(log_entries.is_err());
        let err = log_entries.unwrap_err();
        assert_eq!(err, ErrorReadingFile::ErrorDuringFileReading);
    }
}
