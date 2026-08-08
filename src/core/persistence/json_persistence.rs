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

        let content = match fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read {}: {}", self.path.display(), e);
                return Ok(None);
            }
        };

        match serde_json::from_str(&content) {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                eprintln!("Failed to parse {}: {}", self.path.display(), e);
                Ok(None)
            }
        }
    }

    pub fn save(&self, value: &T) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;
        }

        let content = serde_json::to_string_pretty(value)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        let tmp_path = self.path.with_extension(format!(
            "{}.tmp",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));

        fs::write(&tmp_path, content).map_err(|e| format!("Failed to write {}: {}", tmp_path.display(), e))?;
        fs::rename(&tmp_path, &self.path).map_err(|e| {
            let _ = fs::remove_file(&tmp_path);
            format!("Failed to rename {} to {}: {}", tmp_path.display(), self.path.display(), e)
        })?;

        Ok(())
    }
}
