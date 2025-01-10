#[derive(Debug)]
pub enum CacheError {
    CapacityError(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::CapacityError(msg) => write!(f, "Erreur de capacité: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {} 