#[derive(PartialEq, Debug)]
pub enum Commands {
    Exit,
    Get(&str),
    Set(&str, &str),
    Delete(&str),
}

impl TryFrom<&str> for Commands {
    type Error = &str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut splitted_iter = value.trim().splitn(2, " ");
        let command = splitted_iter.next().unwrap();
        match command {
            "/q" => Ok(Self::Exit),
            "Get" => {
                let key = splitted_iter.next().ok_or_else(|| Err("Coudn't fetch a key"))?;
                Ok(Self::Get(key))
            },
            "Set" => {
                let key = splitted_iter.next().ok_or_else(|| Err("Coudn't fetch a key"))?;
                let value = splitted_iter.next().ok_or_else(|| Err("Coudn't fetch a value"))?;
                Ok(Self::Set(key, value))
            },
            "Delete" => {
                let key = splitted_iter.next().ok_or_else(|| Err("Coudn't fetch a key"))?;
                Ok(Self::Delete(key))
            },
            _ => Err("Incorrect command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get_key() {
        let command_str = "Get key1";
        let parsed_command = Commands::try_from(command_str);
        assert!(parsed_command.is_ok());
        let parsed_command = parsed_command.unwrap();
        assert_eq!(parsed_command, Commands::Get("key1"));
    }
}
