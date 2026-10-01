use std::collections::HashMap;

use crate::{KVDB, KVDBErrors};

pub struct InMemoryKVDB {
    hashmap: HashMap<String, String>,
}

impl InMemoryKVDB {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
}

impl KVDB for InMemoryKVDB {
    fn get(&self, key: &str) -> Result<String, KVDBErrors> {
        let val = self.hashmap.get(key).ok_or(KVDBErrors::NoSuchKey)?;
        Ok(val.to_owned())
    }

    fn set(&mut self, key: String, value: String) -> Result<(), KVDBErrors> {
        self.hashmap.insert(key, value.to_owned());
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<String, KVDBErrors> {
        match self.hashmap.get(key) {
            Some(_) => Ok(self.hashmap.remove(key).unwrap()),
            None => Err(KVDBErrors::NoSuchKey),
        }
    }
}
