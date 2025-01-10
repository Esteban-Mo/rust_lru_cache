use thiserror::Error;

#[derive(Debug)]
pub enum CacheError {
    CapacityError(String),
    StorageError(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::CapacityError(msg) => write!(f, "Erreur de capacité: {}", msg),
            CacheError::StorageError(msg) => write!(f, "Erreur de stockage: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {} 