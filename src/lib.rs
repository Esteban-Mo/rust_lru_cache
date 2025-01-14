//! Bibliothèque implémentant un cache LRU (Least Recently Used).
//! 
//! Cette bibliothèque fournit une implémentation d'un cache avec politique
//! d'éviction LRU, permettant de stocker un nombre limité d'éléments et
//! de supprimer automatiquement les éléments les moins récemment utilisés
//! lorsque la capacité est atteinte.
//! 
//! # Fonctionnalités principales
//! 
//! - Cache générique supportant différents types de clés et valeurs
//! - Politique d'éviction LRU
//! - Persistance optionnelle sur disque
//! - Interface trait pour l'extensibilité
//! 
//! # Exemple d'utilisation
//! 
//! ```
//! use lru_cache::lru::Cache;
//! use lru_cache::lru::traits::CacheTrait;
//! 
//! // Création d'un cache de taille 2
//! let mut cache = Cache::new(2);
//! 
//! // Ajout d'éléments
//! cache.put("clé1", "valeur1");
//! cache.put("clé2", "valeur2");
//! 
//! // Accès aux éléments (met à jour l'ordre LRU)
//! assert_eq!(cache.get(&"clé1"), Some(&"valeur1"));
//! 
//! // L'ajout d'un nouvel élément évince l'élément le moins récemment utilisé
//! cache.put("clé3", "valeur3");
//! assert_eq!(cache.get(&"clé2"), None); // clé2 a été évincée
//! ```

pub mod error;
pub mod lru;