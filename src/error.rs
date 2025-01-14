//! Module de gestion des erreurs pour le cache LRU.

use std::io;

/// Énumération des erreurs possibles lors de l'utilisation du cache.
#[derive(Debug)]
pub enum CacheError {
    /// Erreur liée à la capacité du cache
    CapacityError(String),
    /// Erreur d'entrée/sortie lors des opérations de persistance
    IoError(io::Error),
    /// Erreur de parsing lors du chargement du cache
    ParseError(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheError::CapacityError(msg) => write!(f, "Erreur de capacité: {}", msg),
            CacheError::IoError(err) => write!(f, "Erreur I/O: {}", err),
            CacheError::ParseError(msg) => write!(f, "Erreur de parsing: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CacheError::IoError(err) => Some(err),
            _ => None,
        }
    }
} 