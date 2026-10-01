#[derive(PartialEq, Debug)]
pub enum ParseErrors {
    FailedToParseCommand,
    EmptyCommand,
    UnknownCommand,
    IncorrectSyntaxGet,
    IncorrectSyntaxSet,
    IncorrectSyntaxDelete,
}

#[derive(PartialEq, Debug)]
pub enum Commands {
    Exit,
    Get(String),
    Set(String, String),
    Delete(String),
}

fn try_to_parse_val(command_str: &str) -> Result<Vec<String>, ParseErrors> {
    shell_words::split(command_str).map_err(|_| ParseErrors::FailedToParseCommand)
}

impl TryFrom<&str> for Commands {
    type Error = ParseErrors;

    fn try_from(command_str: &str) -> Result<Self, Self::Error> {
        let mut splitted = try_to_parse_val(command_str)?;
        if splitted.is_empty() {
            return Err(ParseErrors::EmptyCommand);
        }
        let option_command = splitted.remove(0);
        match option_command.as_str() {
            "exit" => Ok(Commands::Exit),
            "get" => {
                if splitted.len() != 1 {
                    return Err(ParseErrors::IncorrectSyntaxGet);
                }
                let key = splitted.remove(0);
                Ok(Commands::Get(key))
            }
            "set" => {
                if splitted.len() != 2 {
                    return Err(ParseErrors::IncorrectSyntaxSet);
                }
                let key = splitted.remove(0);
                let val = splitted.remove(0);
                Ok(Commands::Set(key, val))
            }
            "delete" => {
                if splitted.len() != 1 {
                    return Err(ParseErrors::IncorrectSyntaxDelete);
                }
                let key = splitted.remove(0);
                Ok(Commands::Delete(key))
            }
            _ => Err(ParseErrors::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_string_parser {
        use super::*;

        #[test]
        fn test_single_key_single_value() {
            let command_str = "set key1 val1";
            let command_parsed = try_to_parse_val(command_str).unwrap();
            assert_eq!(command_parsed, ["set", "key1", "val1"]);
        }

        #[test]
        fn test_single_key_multi_value() {
            let command_str = "set key1 \"val11 subval12\"";
            let command_parsed = shell_words::split(command_str).unwrap();
            assert_eq!(command_parsed, ["set", "key1", "val11 subval12"]);
        }

        #[test]
        fn test_multi_key_multi_value() {
            let command_str = "set \"key1 key2\" \"val11 subval12\"";
            let command_parsed = shell_words::split(command_str).unwrap();
            assert_eq!(command_parsed, ["set", "key1 key2", "val11 subval12"]);
        }
    }

    mod test_command_parser_error_cases {
        use super::*;

        #[test]
        fn incorrect_syntax() {
            let command_str = "get \"key1 key2\" \"val11 subval12";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::FailedToParseCommand);
        }

        #[test]
        fn empty_command() {
            let command_str = "";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::EmptyCommand);
        }

        #[test]
        fn non_existing_command() {
            let command_str = "ewqewq \"key1 key2\" \"val11 subval12\"";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::UnknownCommand);
        }
    }

    mod test_exit_command {
        use super::*;

        #[test]
        fn test_command_parser_exit() {
            let command_str = "exit";
            let command = Commands::try_from(command_str);
            assert!(command.is_ok());
            let command = command.unwrap();
            assert_eq!(command, Commands::Exit);
        }
    }

    mod test_get_command {
        use super::*;

        #[test]
        fn get_with_one_arg() {
            let command_str = "get key1";
            let command = Commands::try_from(command_str);
            assert!(command.is_ok());
            let command = command.unwrap();
            assert_eq!(command, Commands::Get("key1".to_owned()));
        }

        #[test]
        fn get_with_multiple_args() {
            let command_str = "get key1 key2";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxGet);
        }

        #[test]
        fn get_with_no_args() {
            let command_str = "get";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxGet);
        }
    }

    mod test_delete_command {
        use super::*;

        #[test]
        fn delete_with_one_arg() {
            let command_str = "delete key1";
            let command = Commands::try_from(command_str);
            assert!(command.is_ok());
            let command = command.unwrap();
            assert_eq!(command, Commands::Delete("key1".to_owned()));
        }

        #[test]
        fn delete_with_multiple_args() {
            let command_str = "delete key1 key2";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxDelete);
        }

        #[test]
        fn delete_with_no_args() {
            let command_str = "delete";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxDelete);
        }
    }

    mod test_set_command {
        use super::*;

        #[test]
        fn set_with_two_args() {
            let command_str = "set key1 val1";
            let command = Commands::try_from(command_str);
            assert!(command.is_ok());
            let command = command.unwrap();
            assert_eq!(command, Commands::Set("key1".to_owned(), "val1".to_owned()));
        }

        #[test]
        fn set_with_one_arg() {
            let command_str = "set key1";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxSet);
        }

        #[test]
        fn set_with_three_args() {
            let command_str = "set key1 val1 val2";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxSet);
        }

        #[test]
        fn set_with_no_args() {
            let command_str = "set";
            let command = Commands::try_from(command_str);
            assert!(command.is_err());
            let command_err = command.unwrap_err();
            assert_eq!(command_err, ParseErrors::IncorrectSyntaxSet);
        }
    }
}
