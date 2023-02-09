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
    kv_file: File,
    entity: Entity,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        fs::create_dir_all(path)?;

        let mut pathbuf = path.to_path_buf();
        pathbuf.push("log-1.json");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&pathbuf)?;
        Ok(Self {
            path: pathbuf,
            kv_file: file,
            entity: Entity {
                store: HashMap::new(),
            },
        })
    }

    pub fn get(&mut self, k: &String) -> Result<String> {
        let cache = &mut self.entity.store;
        if let Some(v) = cache.get(k) {
            // 1. 在缓存中寻找
            Ok(v.clone())
        } else {
            // 2. 缓存中找不到，在文件中寻找
            let content = fs::read_to_string(self.path.as_path())?;
            let dec: HashMap<String, String> = serde_json::from_str(&content).unwrap();
            if let Some(v) = dec.get(k) {
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

        self.kv_file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
