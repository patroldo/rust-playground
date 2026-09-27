#[derive(PartialEq, Debug)]
pub enum Commands<'a> {
    Exit,
    Get(&'a str),
    Set(&'a str, &'a str),
    Delete(&'a str),
}

trait ReverseOptionToResult<T> {
    fn none_or_err(self) -> Result<(), T>;
}

// option<Some(v), None> -> result<Ok(v), Err(custom)>
// option<Some(v), None> -> result<Err(v), Ok(custom>)

impl<T> ReverseOptionToResult<T> for Option<T> {
    fn none_or_err(self) -> Result<(), T> {
        match self {
            Some(v) => Err(v),
            None => Ok(()),
        }
    }
}

impl<'a> TryFrom<&'a str> for Commands<'a> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut splitted_iter = value.trim().splitn(3, " ");
        let command = splitted_iter.next().unwrap();
        match command {
            "/q" => Ok(Self::Exit),
            "Get" => {
                let key = splitted_iter.next().ok_or("Coudn't fetch a key")?;
                splitted_iter
                    .next()
                    .none_or_err()
                    .map_err(|_| "Incorrect format of get command")?;
                if key.is_empty() {
                    Err("Empty key")
                } else {
                    Ok(Self::Get(key))
                }
            }
            "Set" => {
                let key = splitted_iter.next().ok_or("Coudn't fetch a key")?;
                let value = splitted_iter.next().ok_or("Coudn't fetch a value")?;
                if key.is_empty() {
                    Err("Empty key")
                } else {
                    Ok(Self::Set(key, value))
                }
            }
            "Delete" => {
                let key = splitted_iter.next().ok_or("Coudn't fetch a key")?;
                splitted_iter
                    .next()
                    .none_or_err()
                    .map_err(|_| "Incorrect format of delete command")?;
                if key.is_empty() {
                    Err("Empty key")
                } else {
                    Ok(Self::Get(key))
                }
            }
            _ => Err("Incorrect command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_command_tests {
        use super::*;

        #[test]
        fn test_parse_get_command() {
            let command_str = "Get key1";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_ok());
            let parsed_command = parsed_command.unwrap();
            assert_eq!(parsed_command, Commands::Get("key1"));
        }

        #[test]
        fn test_parse_get_with_empty_key() {
            let command_str = "Get  ";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a key");
        }

        #[test]
        fn test_parse_get_with_no_key_passed() {
            let command_str = "Get";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a key");
        }

        #[test]
        fn test_parse_get_key_extra_param() {
            let command_str = "Get key1 random";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Incorrect format of get command");
        }
    }

    mod delete_command_tests {
        use super::*;

        #[test]
        fn test_parse_delete_command() {
            let command_str = "Delete key1";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_ok());
            let parsed_command = parsed_command.unwrap();
            assert_eq!(parsed_command, Commands::Get("key1"));
        }

        #[test]
        fn test_parse_delete_with_empty_key() {
            let command_str = "Delete  ";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a key");
        }

        #[test]
        fn test_parse_delete_with_no_key_passed() {
            let command_str = "Delete";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a key");
        }

        #[test]
        fn test_parse_delete_key_extra_param() {
            let command_str = "Delete key1 random";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Incorrect format of delete command");
        }
    }

    mod set_command_tests {
        use super::*;

        #[test]
        fn test_parse_set_command() {
            let command_str = "Set key1 val1";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_ok());
            let parsed_command = parsed_command.unwrap();
            assert_eq!(parsed_command, Commands::Set("key1", "val1"));
        }

        #[test]
        fn test_parse_set_command_multi_value_string() {
            let command_str = "Set key1 Sentence for the test";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_ok());
            let parsed_command = parsed_command.unwrap();
            assert_eq!(
                parsed_command,
                Commands::Set("key1", "Sentence for the test")
            );
        }

        #[test]
        fn test_parse_empty_key() {
            let command_str = "Set  val1";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Empty key");
        }

        #[test]
        fn test_parse_empty_value() {
            let command_str = "Set key1 ";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a value");
        }

        #[test]
        fn test_parse_value_not_passed() {
            let command_str = "Set key1";
            let parsed_command = Commands::try_from(command_str);
            assert!(parsed_command.is_err());
            let parsed_command = parsed_command.unwrap_err();
            assert_eq!(parsed_command, "Coudn't fetch a value");
        }
    }

    #[test]
    fn test_fail_to_parse_command() {
        let command_str = "Gett";
        let parsed_command = Commands::try_from(command_str);
        assert!(parsed_command.is_err());
        let parsed_command = parsed_command.unwrap_err();
        assert_eq!(parsed_command, "Incorrect command");
    }
}
