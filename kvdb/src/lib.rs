pub mod cli_parser;
mod in_memory_kvdb;

use cli_parser::Commands;
use in_memory_kvdb::InMemoryKVDB;

#[derive(PartialEq, Debug)]
pub enum KVDBErrors {
    NoSuchKey,
}

pub enum KVDBType {
    InMemory,
}

impl KVDBType {
    pub fn create_kvdb(kvdb_type: KVDBType) -> impl KVDB
    where
        Self: Sized,
    {
        match kvdb_type {
            KVDBType::InMemory => InMemoryKVDB::new(),
        }
    }
}

pub trait KVDB {
    fn get(&self, key: &str) -> Result<&str, KVDBErrors>;
    fn set(&mut self, key: String, value: String) -> Result<(), KVDBErrors>;
    fn delete(&mut self, key: &str) -> Result<String, KVDBErrors>;
}

pub fn apply_command_to_kvdb(kvdb: &mut impl KVDB, command: Commands) -> String {
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(KVDBType::InMemory)]
    fn test_in_memory_kvdb(#[case] db_type: KVDBType) {
        let mut db = KVDBType::create_kvdb(db_type);
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
}
