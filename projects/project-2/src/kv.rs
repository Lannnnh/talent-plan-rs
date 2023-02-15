use crate::error::{KvError, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
// use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    store: HashMap<String, String>,
}

pub struct KvStore {
    path: PathBuf,
    entity: Entity,
}

impl KvStore {
    fn deserialize_from_kv_file(path: &Path) -> Result<HashMap<String, String>> {
        let content = fs::read_to_string(path)?;

        let hash_map: HashMap<String, String> = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => HashMap::new(),
        };

        Ok(hash_map)
    }

    fn serialize_to_kv_file(path: &Path, kv_store: &HashMap<String, String>) -> Result<()> {
        let serialized = serde_json::to_string(kv_store).unwrap();
        fs::write(path, serialized.as_bytes())?;
        Ok(())
    }

    pub fn open(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)?;

        let mut pathbuf = path.to_path_buf();
        pathbuf.push("log-1.json");

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&pathbuf)?;

        // 直接将文件信息同步到缓存中
        let mut s = Self {
            path: pathbuf,
            entity: Entity {
                store: HashMap::new(),
            },
        };
        let hash_map = KvStore::deserialize_from_kv_file(&s.path)?;
        s.entity.store.clone_from(&hash_map);
        Ok(s)
    }

    pub fn get(&mut self, k: &String) -> Result<Option<String>> {
        let cache = &mut self.entity.store;
        if let Some(v) = cache.get(k) {
            // 1. 在缓存中寻找
            Ok(Some(v.clone()))
        } else {
            // 2. 缓存中找不到，同步文件信息到缓存中，继续寻找
            let new_hash_map = KvStore::deserialize_from_kv_file(&self.path)?;
            cache.clone_from(&new_hash_map);
            if let Some(v) = cache.get(k) {
                Ok(Some(v.clone()))
            } else {
                // 3. 都找不到，返回 Key not found
                Err(KvError::KeyNotFound)
            }
        }
    }

    pub fn rm(&mut self, k: &String) -> Result<Option<String>> {
        let cache = &mut self.entity.store;
        let mut value: String = "".to_owned();
        if let Some(v) = cache.remove(k) {
            // 1. 在缓存中找到，直接移除
            value = v;
        } else {
            // 2. 在缓存中找不到，继而在文件中寻找，找到移除
            let hash_map = KvStore::deserialize_from_kv_file(&self.path)?;
            cache.clone_from(&hash_map);
            if let Some(v) = cache.remove(k) {
                value = v;
            } else {
                return Err(KvError::KeyNotFound);
            }
        }

        // 同步移除后的缓存到文件中
        KvStore::serialize_to_kv_file(&self.path, &cache)?;

        Ok(Some(value))
    }

    pub fn set(&mut self, k: &String, v: &String) -> Result<()> {
        let key = String::from(k);
        let value = String::from(v);

        let cache = &mut self.entity.store;
        // 1. 数据写入缓存
        cache.insert(key, value);

        // 2. 缓存写入文件
        KvStore::serialize_to_kv_file(&self.path, &cache)?;
        Ok(())
    }
}
