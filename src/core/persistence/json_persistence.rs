use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct JsonStore<T: Serialize + DeserializeOwned> {
    path: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> JsonStore<T> {
    pub fn new(path: PathBuf) -> Self {
        Self { path, _phantom: PhantomData }
    }

    pub fn load(&self) -> Result<Option<T>, String> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path).map_err(|e| format!("Failed to read {}: {}", self.path.display(), e))?;
        serde_json::from_str(&content)
            .map(Some)
            .map_err(|e| format!("Failed to parse {}: {}", self.path.display(), e))
    }

    pub fn save(&self, value: &T) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;
        }

        let content = serde_json::to_string_pretty(value)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        fs::write(&self.path, content).map_err(|e| format!("Failed to write {}: {}", self.path.display(), e))?;
        Ok(())
    }
}