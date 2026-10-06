pub mod cli_parser;
mod kvdb_imps;

use cli_parser::Commands;
use kvdb_imps::in_memory_kvdb::InMemoryKVDB;

use crate::kvdb_imps::file_based_kvdb::FileBasedKVDB;

#[derive(PartialEq, Debug)]
pub enum KVDBErrors {
    NoSuchKey,
    Other(String),
}

pub enum KVDBType<'a> {
    InMemory,
    FileBased(Option<&'a str>),
}

impl<'a> KVDBType<'a> {
    pub fn create_kvdb(kvdb_type: KVDBType) -> Result<Box<dyn KVDB>, KVDBErrors>
    where
        Self: Sized,
    {
        match kvdb_type {
            KVDBType::InMemory => Ok(Box::new(InMemoryKVDB::new())),
            KVDBType::FileBased(file_path) => Ok(Box::new(FileBasedKVDB::new(file_path)?)),
        }
    }
}

pub trait KVDB {
    fn get(&self, key: &str) -> Result<String, KVDBErrors>;
    fn set(&mut self, key: String, value: String) -> Result<(), KVDBErrors>;
    fn delete(&mut self, key: &str) -> Result<String, KVDBErrors>;
}

pub fn apply_command_to_kvdb(kvdb: &mut dyn KVDB, command: Commands) -> String {
    match command {
        Commands::Get(key) => match kvdb.get(&key) {
            Ok(val) => val.to_owned(),
            Err(e) => format!("{:?}", e),
        },
        Commands::Set(key, val) => match kvdb.set(key, val) {
            Ok(_) => "Ok".to_owned(),
            Err(e) => format!("{:?}", e),
        },
        Commands::Delete(key) => match kvdb.delete(&key) {
            Ok(val) => val,
            Err(e) => format!("{:?}", e),
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::Builder;

    use super::*;

    fn test_kvdb_implementations(db_type: KVDBType) {
        let mut db = KVDBType::create_kvdb(db_type).unwrap();
        let res = db.get("key1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), KVDBErrors::NoSuchKey);
        //--------------------------------------------------
        let res = db.set("key1".to_string(), "val1".to_string());
        assert!(res.is_ok());
        let res = db.get("key1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "val1");
        //--------------------------------------------------
        let res = db.delete("key1");
        assert!(res.is_ok());
        let res = db.get("key1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), KVDBErrors::NoSuchKey);
    }

    #[test]
    fn test_in_memory_kvdb() {
        test_kvdb_implementations(KVDBType::InMemory);
    }

    #[test]
    fn test_file_based_kvdb() {
        let tmp_file = Builder::new().prefix("test_tmp_kvdb").tempfile().unwrap();
        test_kvdb_implementations(KVDBType::FileBased(tmp_file.path().to_str()));
        drop(tmp_file);
    }

    #[test]
    fn apply_command_to_kvdb_get() {
        let mut db = KVDBType::create_kvdb(KVDBType::InMemory).unwrap();
        let command = Commands::Get("key".to_owned());
        assert_eq!("NoSuchKey", apply_command_to_kvdb(db.as_mut(), command));
        db.set("key".to_owned(), "val".to_owned()).unwrap();
        let command = Commands::Get("key".to_owned());
        assert_eq!("val", apply_command_to_kvdb(db.as_mut(), command));
        let command = Commands::Get("key".to_owned());
        assert_eq!("val", apply_command_to_kvdb(db.as_mut(), command));
    }

    #[test]
    fn apply_command_to_kvdb_set() {
        let mut db = KVDBType::create_kvdb(KVDBType::InMemory).unwrap();
        assert_eq!(KVDBErrors::NoSuchKey, db.get("key").unwrap_err());
        let command = Commands::Set("key".to_owned(), "val321".to_owned());
        assert_eq!("Ok", apply_command_to_kvdb(db.as_mut(), command));
        assert_eq!("val321", db.get("key").unwrap());
    }

    #[test]
    fn apply_command_to_kvdb_delete() {
        let mut db = KVDBType::create_kvdb(KVDBType::InMemory).unwrap();
        assert_eq!(KVDBErrors::NoSuchKey, db.get("key").unwrap_err());
        db.set("key".to_owned(), "val321".to_owned()).unwrap();
        assert_eq!("val321", db.get("key").unwrap());
        let command = Commands::Delete("key".to_owned());
        assert_eq!("val321", apply_command_to_kvdb(db.as_mut(), command));
        let command = Commands::Delete("key".to_owned());
        assert_eq!("NoSuchKey", apply_command_to_kvdb(db.as_mut(), command));
    }
}
