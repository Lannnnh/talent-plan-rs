use crate::error::{KvError, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
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
        let hash_map = KvStore::deserialize_from_kv_file(&pathbuf)?;

        Ok(Self {
            path: pathbuf,
            entity: Entity { store: hash_map },
        })
    }

    pub fn get(&mut self, k: &String) -> Result<String> {
        let cache = &mut self.entity.store;
        if let Some(v) = cache.get(k) {
            // 1. 在缓存中寻找
            Ok(v.clone())
        } else {
            // 2. 缓存中找不到，同步文件信息到缓存中，继续寻找
            let new_hash_map = KvStore::deserialize_from_kv_file(self.path.as_path())?;
            cache.clone_from(&new_hash_map);
            if let Some(v) = cache.get(k) {
                Ok(v.clone())
            } else {
                // 3. 都找不到，返回 Key not found
                Err(KvError::KeyNotFound)
            }
        }
    }

    // pub fn rm(&mut self, k: &String) -> Result<String> {
    //     match self.KvCache.remove(k) {
    //         Some(v) => Ok(v.clone()),
    //         None => Err(KvError::KeyNotFound),
    //     }
    // }

    pub fn set(&mut self, k: &String, v: &String) -> Result<()> {
        let key = String::from(k);
        let value = String::from(v);

        let cache = &mut self.entity.store;
        // 1. 数据写入缓存
        cache.insert(key, value);

        // 2. 缓存写入文件
        let serialized = serde_json::to_string(&cache).unwrap();

        fs::write(self.path.as_path(), serialized.as_bytes())?;
        Ok(())
    }
}
