use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use crate::lru::traits::CacheTrait;
use crate::error::CacheError;

pub struct PersistentCache<K, V>
where
    K: Hash + Eq + Serialize + for<'de> Deserialize<'de>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    cache: super::Cache<K, V>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V>
where
    K: Hash + Eq + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(capacity: usize, file_path: &str) -> Result<Self, CacheError> {
        let cache = super::Cache::new(capacity);
        let persistent_cache = PersistentCache {
            cache,
            file_path: file_path.to_string(),
        };
        persistent_cache.load_from_file()?;
        Ok(persistent_cache)
    }

    fn load_from_file(&self) -> Result<(), CacheError> {
        if !Path::new(&self.file_path).exists() {
            return Ok(());
        }

        let mut file = File::open(&self.file_path).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors de l'ouverture du fichier: {}", e)))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors de la lecture du fichier: {}", e)))?;

        if contents.is_empty() {
            return Ok(());
        }

        let data: Vec<(K, V)> = serde_json::from_str(&contents).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors de la désérialisation: {}", e)))?;

        for (key, value) in data {
            self.cache.put(key, value);
        }

        Ok(())
    }

    fn save_to_file(&self) -> Result<(), CacheError> {
        let data: Vec<(&K, &V)> = self.cache.map.iter().collect();
        let serialized = serde_json::to_string(&data).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors de la sérialisation: {}", e)))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| CacheError::StorageError(format!("Erreur lors de l'écriture du fichier: {}", e)))?;

        file.seek(SeekFrom::Start(0)).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors du positionnement dans le fichier: {}", e)))?;
        
        file.write_all(serialized.as_bytes()).map_err(|e| 
            CacheError::StorageError(format!("Erreur lors de l'écriture dans le fichier: {}", e)))?;

        Ok(())
    }
}

impl<K, V> CacheTrait<K, V> for PersistentCache<K, V>
where
    K: Hash + Eq + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Serialize + for<'de> Deserialize<'de>,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn put(&mut self, key: K, value: V) {
        self.cache.put(key, value);
        if let Err(e) = self.save_to_file() {
            eprintln!("Erreur lors de la sauvegarde du cache: {}", e);
        }
    }
} 