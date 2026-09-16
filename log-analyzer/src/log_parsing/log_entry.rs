use chrono::{DateTime, FixedOffset};

use crate::log_parsing::log_level::{LogLevel, ParseLogLevelError};

#[derive(Debug, PartialEq)]
pub enum ParseLogError {
    IncorrectLogEntryFormat(String),
    UnparsableLogLevelType(String, ParseLogLevelError),
    UnparsableTimeStamp(String),
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    timestamp: DateTime<FixedOffset>,
    pub log_level: LogLevel,
    message: String,
}

fn try_to_split_str(value: &str) -> Result<(&str, &str, &str), ParseLogError> {
    let (timestamp_str, log_level_str, message_str): (&str, &str, &str);
    let mut iter = value.trim().splitn(3, " ");
    timestamp_str = iter
        .next()
        .ok_or(ParseLogError::IncorrectLogEntryFormat(String::from(value)))?;
    log_level_str = iter
        .next()
        .ok_or(ParseLogError::IncorrectLogEntryFormat(String::from(value)))?;
    message_str = iter
        .next()
        .ok_or(ParseLogError::IncorrectLogEntryFormat(String::from(value)))?;
    Ok((timestamp_str, log_level_str, message_str))
}

impl TryFrom<&str> for LogEntry {
    type Error = ParseLogError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (timestamp_str, log_level_str, message_str): (&str, &str, &str) =
            try_to_split_str(value)?;

        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .map_err(|_| ParseLogError::UnparsableTimeStamp(String::from(timestamp_str)))?;
        let log_level: LogLevel = LogLevel::try_from(log_level_str)
            .map_err(|e| ParseLogError::UnparsableLogLevelType(String::from(value), e))?;
        let message = String::from(message_str);

        Ok(LogEntry {
            timestamp,
            log_level,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rfc3339_parsing_happy_path() {
        fn parse_time_stamp(str: &str) -> Result<DateTime<FixedOffset>, ParseLogError> {
            DateTime::parse_from_rfc3339(str)
                .map_err(|_| ParseLogError::UnparsableTimeStamp(String::from(str)))
        }

        let str = "2026-09-11T10:01:03Z";
        let timestamp_result = parse_time_stamp(str);
        assert!(timestamp_result.is_ok());
        assert_eq!(timestamp_result.unwrap().timestamp(), 1789120863);

        let str = "2026-09-11T10:01:05Z";
        let timestamp_result = parse_time_stamp(str);
        assert!(timestamp_result.is_ok());
        assert_eq!(timestamp_result.unwrap().timestamp(), 1789120865);

        let str = "2026-09-11T10:01:07Z";
        let timestamp_result = parse_time_stamp(str);
        assert!(timestamp_result.is_ok());
        assert_eq!(timestamp_result.unwrap().timestamp(), 1789120867);
    }

    #[test]
    fn parse_info_log() {
        let log_entry = LogEntry::try_from("2026-09-11T10:01:03Z INFO server started");
        assert!(log_entry.is_ok());
        let log_entry = log_entry.unwrap();
        assert_eq!(log_entry.message, "server started");
        assert_eq!(log_entry.log_level, LogLevel::InfoL);
        assert_eq!(log_entry.timestamp.timestamp(), 1789120863);
    }

    #[test]
    fn parse_error_log() {
        let log_entry = LogEntry::try_from("2026-09-11T10:01:05Z ERROR connection refused");
        assert!(log_entry.is_ok());
        let log_entry = log_entry.unwrap();
        assert_eq!(log_entry.message, "connection refused");
        assert_eq!(log_entry.log_level, LogLevel::ErrorL);
        assert_eq!(log_entry.timestamp.timestamp(), 1789120865);
    }

    #[test]
    fn parse_warn_log() {
        let log_entry = LogEntry::try_from("2026-09-11T10:01:07Z WARN retrying");
        assert!(log_entry.is_ok());
        let log_entry = log_entry.unwrap();
        assert_eq!(log_entry.message, "retrying");
        assert_eq!(log_entry.log_level, LogLevel::WarnL);
        assert_eq!(log_entry.timestamp.timestamp(), 1789120867);
    }

    #[test]
    fn parse_log_absent_timestamp() {
        let log_entry = LogEntry::try_from("WARN retrying");
        assert!(log_entry.is_err());
        let parse_error = log_entry.unwrap_err();
        assert_eq!(
            parse_error,
            ParseLogError::IncorrectLogEntryFormat(String::from("WARN retrying"))
        );
    }

    #[test]
    fn parse_log_absent_log_level() {
        let log_entry = LogEntry::try_from("2026-09-11T10:01:07Z retrying");
        assert!(log_entry.is_err());
        let parse_error = log_entry.unwrap_err();
        assert_eq!(
            parse_error,
            ParseLogError::IncorrectLogEntryFormat(String::from("2026-09-11T10:01:07Z retrying"))
        );
    }

    #[test]
    fn parse_log_absent_log_message() {
        let log_entry = LogEntry::try_from("2026-09-11T10:01:07Z INFO");
        assert!(log_entry.is_err());
        let parse_error = log_entry.unwrap_err();
        assert_eq!(
            parse_error,
            ParseLogError::IncorrectLogEntryFormat(String::from("2026-09-11T10:01:07Z INFO"))
        );
    }
}
