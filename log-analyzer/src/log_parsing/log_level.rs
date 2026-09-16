use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseLogLevelError(String);

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    InfoL,
    ErrorL,
    WarnL,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            LogLevel::InfoL => "INFO",
            LogLevel::ErrorL => "ERROR",
            LogLevel::WarnL => "WARN",
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for LogLevel {
    type Error = ParseLogLevelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "INFO" => Ok(LogLevel::InfoL),
            "WARN" => Ok(LogLevel::WarnL),
            "ERROR" => Ok(LogLevel::ErrorL),
            other => Err(ParseLogLevelError(
                format!("Unparsable log level: {other}",),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_info_log_level() {
        let log_level = LogLevel::try_from("INFO");
        assert!(log_level.is_ok());
        assert_eq!(log_level.unwrap(), LogLevel::InfoL);
    }

    #[test]
    fn test_parse_warn_log_level() {
        let log_level = LogLevel::try_from("WARN");
        assert!(log_level.is_ok());
        assert_eq!(log_level.unwrap(), LogLevel::WarnL);
    }

    #[test]
    fn test_parse_error_log_level() {
        let log_level = LogLevel::try_from("ERROR");
        assert!(log_level.is_ok());
        assert_eq!(log_level.unwrap(), LogLevel::ErrorL);
    }

    #[test]
    fn test_parse_unknown_log_level() {
        let log_level = LogLevel::try_from("unknown");
        assert!(log_level.is_err());
        assert_eq!(
            log_level.unwrap_err(),
            ParseLogLevelError(String::from("Unparsable log level: unknown"))
        );
    }
}
