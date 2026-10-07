use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use tempfile::Builder;

use crate::{KVDB, KVDBErrors};

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct FileBasedKVDB {
    file: PathBuf,
    hashmap: HashMap<String, String>,
}

impl FileBasedKVDB {
    pub fn new(path_str: &str) -> Result<Self, KVDBErrors> {
        let pathbuf = PathBuf::from(path_str);
        if pathbuf.is_dir() {
            return Err(KVDBErrors::Other("Path is a folder".to_owned()));
        }
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path_str)
            .map_err(|e| KVDBErrors::Other(e.to_string()))?;
        let parsed_hashmap: Result<Vec<Row>, KVDBErrors> = rdr
            .deserialize()
            .map(|input_row| input_row.map_err(|e| KVDBErrors::Other(e.to_string())))
            .collect();
        let hashmap = parsed_hashmap?
            .into_iter()
            .map(|row| (row.key, row.value))
            .collect();
        Ok(Self {
            file: pathbuf,
            hashmap,
        })
    }
}

impl KVDB for FileBasedKVDB {
    fn get(&self, key: &str) -> Result<String, crate::KVDBErrors> {
        self.hashmap
            .iter()
            .find(|item| item.0 == key)
            .map(|item| item.1.to_owned())
            .ok_or(KVDBErrors::NoSuchKey)
    }

    fn set(&mut self, key: String, value: String) -> Result<(), crate::KVDBErrors> {
        let mut tmp_hashmap = self.hashmap.clone();
        let tmp_file = Builder::new()
            .prefix("test_kvdb")
            .tempfile()
            .map_err(|_| KVDBErrors::Other("Coudn't create temp file".to_owned()))?;
        tmp_hashmap.insert(key.to_owned(), value);
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path(tmp_file.path())
            .map_err(|e| KVDBErrors::Other(e.to_string()))?;
        let write_result: Result<Vec<()>, KVDBErrors> = tmp_hashmap
            .iter()
            .map(|(key, value)| Row {
                key: key.to_owned(),
                value: value.to_owned(),
            })
            .map(|item| {
                writer
                    .serialize(item)
                    .map_err(|e| KVDBErrors::Other(e.to_string()))
            })
            .collect();
        write_result?;
        writer
            .flush()
            .map_err(|e| KVDBErrors::Other(e.to_string()))?;
        fs::copy(&tmp_file, &self.file).map_err(|e| KVDBErrors::Other(e.to_string()))?;
        self.hashmap = tmp_hashmap;
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<String, crate::KVDBErrors> {
        let mut tmp_hashmap = self.hashmap.clone();
        let tmp_file = Builder::new()
            .prefix("test_kvdb")
            .tempfile()
            .map_err(|_| KVDBErrors::Other("Coudn't create temp file".to_owned()))?;
        let val = tmp_hashmap.remove(key).ok_or(KVDBErrors::NoSuchKey)?;
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path(tmp_file.path())
            .map_err(|e| KVDBErrors::Other(e.to_string()))?;
        let write_result: Result<Vec<()>, KVDBErrors> = tmp_hashmap
            .iter()
            .map(|(key, value)| Row {
                key: key.to_owned(),
                value: value.to_owned(),
            })
            .map(|item| {
                writer
                    .serialize(item)
                    .map_err(|e| KVDBErrors::Other(e.to_string()))
            })
            .collect();
        write_result?;
        writer
            .flush()
            .map_err(|e| KVDBErrors::Other(e.to_string()))?;
        fs::copy(&tmp_file, &self.file).map_err(|e| KVDBErrors::Other(e.to_string()))?;
        self.hashmap = tmp_hashmap;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::Builder;

    use super::*;

    fn try_to_find_key_in_file(key: &str, tmp_file: &Path) -> Option<String> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(tmp_file)
            .unwrap();
        rdr.deserialize()
            .map(|input_row| {
                let row: Row = input_row.unwrap();
                row
            })
            .find(|row| row.key == key)
            .map(|row| row.value)
    }

    #[test]
    fn test_get() {
        let key = "test";
        let original_file = PathBuf::from("tests/data/file.db");
        let tmp_file = Builder::new().prefix("test_kvdb").tempfile().unwrap();
        fs::copy(&original_file, &tmp_file).unwrap();
        let val = try_to_find_key_in_file(key, tmp_file.path());
        assert!(val.is_some());
        let val = val.unwrap();
        let kvdb = FileBasedKVDB::new(tmp_file.path().to_str().unwrap()).unwrap();
        let kvdb_get_res = kvdb.get(key);
        assert!(kvdb_get_res.is_ok());
        assert_eq!(kvdb_get_res.unwrap(), val);
    }

    #[test]
    fn test_set() {
        let key = "key_not_exist_yet".to_owned();
        let value = "value_for_it".to_owned();
        let original_file = PathBuf::from("tests/data/file.db");
        let tmp_file = Builder::new().prefix("test_tmp_kvdb").tempfile().unwrap();
        fs::copy(&original_file, &tmp_file).unwrap();
        let val = try_to_find_key_in_file(&key, tmp_file.path());
        assert!(val.is_none());
        let mut kvdb = FileBasedKVDB::new(tmp_file.path().to_str().unwrap()).unwrap();
        let kvdb_get_res = kvdb.set(key.clone(), value.clone());
        assert!(kvdb_get_res.is_ok());
        let val = try_to_find_key_in_file(&key, tmp_file.path());
        assert!(val.is_some());
        assert_eq!(val.unwrap(), value);
    }

    #[test]
    fn test_delete() {
        let key = "test";
        let original_file = PathBuf::from("tests/data/file.db");
        let tmp_file = Builder::new().prefix("test_kvdb").tempfile().unwrap();
        fs::copy(&original_file, &tmp_file).unwrap();
        let val = try_to_find_key_in_file(key, tmp_file.path());
        assert!(val.is_some());
        let mut kvdb = FileBasedKVDB::new(tmp_file.path().to_str().unwrap()).unwrap();
        let kvdb_get_res = kvdb.get(key);
        assert!(kvdb_get_res.is_ok());
        let deletion_result = kvdb.delete(key);
        assert!(deletion_result.is_ok());
        assert_eq!(deletion_result.unwrap(), val.unwrap());
        let val = try_to_find_key_in_file(key, tmp_file.path());
        assert!(val.is_none());
        let kvdb_get_res = kvdb.get(key);
        assert!(kvdb_get_res.is_err());
        assert_eq!(kvdb_get_res.unwrap_err(), KVDBErrors::NoSuchKey);
    }
}
