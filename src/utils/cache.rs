use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Cache<T> {
    data: HashMap<String, T>,
    cache_dir: PathBuf,
}

impl<T> Cache<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            data: HashMap::new(),
            cache_dir,
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: String, value: T) {
        self.data.insert(key, value);
    }

    pub fn save(&self, filename: &str) -> Result<()> {
        let path = self.cache_dir.join(filename);
        let json = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        let path = self.cache_dir.join(filename);
        if path.exists() {
            let json = std::fs::read_to_string(path)?;
            self.data = serde_json::from_str(&json)?;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
