use crate::error::{KvError, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, k: &String) -> Result<String> {
        match self.store.get(k) {
            Some(v) => Ok(v.clone()),
            None => Err(KvError::KeyNotFound),
        }
    }

    pub fn rm(&mut self, k: &String) -> Result<String> {
        match self.store.remove(k) {
            Some(v) => Ok(v.clone()),
            None => Err(KvError::KeyNotFound),
        }
    }

    pub fn set(&mut self, k: &String, v: &String) -> Result<String> {
        let key = String::from(k);
        let value = String::from(v);
        match self.store.insert(key, value) {
            Some(v) => Ok(v),
            None => Ok(String::from("")),
        }
    }
}
