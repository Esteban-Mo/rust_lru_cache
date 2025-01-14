//! Module implémentant un cache LRU (Least Recently Used).
//! 
//! Ce module fournit une implémentation d'un cache avec politique d'éviction LRU.
//! Le cache maintient un nombre limité d'éléments et supprime automatiquement
//! les éléments les moins récemment utilisés lorsque sa capacité est atteinte.
//! 
//! # Exemple simple
//! ```
//! use lru_cache::lru::Cache;
//! use lru_cache::lru::traits::CacheTrait;
//! 
//! let mut cache = Cache::new(2);
//! 
//! // Ajout d'éléments
//! cache.put("clé1".to_string(), "valeur1".to_string());
//! cache.put("clé2".to_string(), "valeur2".to_string());
//! 
//! // Accès aux éléments
//! assert_eq!(cache.get(&"clé1".to_string()), Some(&"valeur1".to_string()));
//! 
//! // L'ajout d'un troisième élément évince le moins récemment utilisé
//! cache.put("clé3".to_string(), "valeur3".to_string());
//! assert_eq!(cache.get(&"clé2".to_string()), None); // clé2 a été évincée
//! ```
//! 
//! # Exemple avec persistance
//! ```no_run
//! use lru_cache::lru::Cache;
//! use lru_cache::lru::traits::CacheTrait;
//! 
//! // Création d'un cache persistant
//! let mut cache = Cache::<String, String>::new_persistent(2, "mon_cache.txt").unwrap();
//! 
//! // Utilisation normale du cache
//! cache.put("clé1".to_string(), "valeur1".to_string());
//! 
//! // Sauvegarde de l'état du cache
//! cache.persist("mon_cache.txt").unwrap();
//! ```

use std::collections::HashMap;
use std::hash::Hash;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;
use std::fmt::Display;
use std::str::FromStr;
use crate::error::CacheError;
use crate::lru::traits::CacheTrait;

pub mod traits;

/// Structure principale du cache LRU.
/// 
/// Le cache utilise une `HashMap` pour stocker les paires clé-valeur et un `Vec`
/// pour maintenir l'ordre d'utilisation des éléments.
/// 
/// # Type Parameters
/// 
/// * `K` - Le type de la clé, qui doit implémenter `Hash` et `Eq`
/// * `V` - Le type de la valeur
/// 
/// # Exemples
/// 
/// ```
/// use lru_cache::lru::Cache;
/// use lru_cache::lru::traits::CacheTrait;
/// 
/// // Cache avec des types simples
/// let mut cache: Cache<i32, String> = Cache::new(2);
/// cache.put(1, "un".to_string());
/// 
/// // Cache avec des types plus complexes
/// let mut cache: Cache<String, Vec<i32>> = Cache::new(2);
/// cache.put("nombres".to_string(), vec![1, 2, 3]);
/// ```
#[derive(Debug)]
pub struct Cache<K, V> 
where 
    K: Hash + Eq,
{
    pub(crate) capacity: usize,
    pub(crate) elements: HashMap<K, V>,
    pub(crate) usage_order: Vec<K>,
}

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
    /// Crée un nouveau cache avec la capacité spécifiée.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - La capacité maximale du cache
    /// 
    /// # Panics
    /// 
    /// Panique si la capacité est 0.
    /// 
    /// # Exemples
    /// 
    /// ```
    /// use lru_cache::lru::Cache;
    /// 
    /// let cache: Cache<String, i32> = Cache::new(3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("La capacité du cache doit être supérieure à 0");
        }
        
        Cache {
            capacity,
            elements: HashMap::with_capacity(capacity),
            usage_order: Vec::with_capacity(capacity),
        }
    }

    /// Met à jour l'ordre d'utilisation en déplaçant la clé spécifiée
    /// à la fin de la liste (élément le plus récemment utilisé).
    fn move_to_recently_used(&mut self, key: &K) {
        if let Some(pos) = self.usage_order.iter().position(|k| k == key) {
            let key = self.usage_order.remove(pos);
            self.usage_order.push(key);
        }
    }

    /// Retourne le nombre d'éléments actuellement dans le cache.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Vérifie si le cache est vide.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Vide le cache de tous ses éléments.
    pub fn clear(&mut self) {
        self.elements.clear();
        self.usage_order.clear();
    }

    /// Retourne un itérateur sur les paires clé-valeur du cache.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.usage_order.iter().filter_map(|key| {
            self.elements.get(key).map(|value| (key, value))
        })
    }
}

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone + Display + FromStr,
    V: Display + FromStr,
{
    /// Crée un nouveau cache persistant avec la capacité spécifiée.
    /// 
    /// Si le fichier existe déjà, le cache est initialisé avec son contenu.
    /// Sinon, un nouveau cache vide est créé.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - La capacité maximale du cache
    /// * `path` - Le chemin du fichier de persistance
    /// 
    /// # Errors
    /// 
    /// Retourne une erreur si :
    /// * Le fichier existe mais ne peut pas être lu
    /// * Le contenu du fichier ne peut pas être parsé
    /// 
    /// # Exemples
    /// 
    /// ```no_run
    /// use lru_cache::lru::Cache;
    /// 
    /// let cache = Cache::<String, String>::new_persistent(3, "cache.txt").unwrap();
    /// ```
    pub fn new_persistent<P: AsRef<Path>>(capacity: usize, path: P) -> Result<Self, CacheError> {
        let cache = match File::open(path.as_ref()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                Self::load_from_reader(reader, capacity)?
            },
            Err(_) => Self::new(capacity),
        };
        Ok(cache)
    }

    fn load_from_reader<R: Read>(mut reader: R, capacity: usize) -> Result<Self, CacheError> {
        let mut content = String::new();
        reader.read_to_string(&mut content)
            .map_err(|e| CacheError::IoError(e))?;

        let mut cache = Self::new(capacity);

        for line in content.lines() {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() != 2 {
                return Err(CacheError::ParseError("Format de ligne invalide".to_string()));
            }

            let key = K::from_str(parts[0])
                .map_err(|_| CacheError::ParseError(format!("Impossible de parser la clé: {}", parts[0])))?;
            let value = V::from_str(parts[1])
                .map_err(|_| CacheError::ParseError(format!("Impossible de parser la valeur: {}", parts[1])))?;

            cache.put(key, value);
        }

        Ok(cache)
    }

    /// Sauvegarde l'état actuel du cache dans un fichier.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Le chemin du fichier où sauvegarder le cache
    /// 
    /// # Errors
    /// 
    /// Retourne une erreur si :
    /// * Le fichier ne peut pas être créé ou écrit
    /// * Une erreur survient lors de l'écriture
    /// 
    /// # Exemples
    /// 
    /// ```no_run
    /// use lru_cache::lru::Cache;
    /// 
    /// let cache = Cache::<String, String>::new(3);
    /// cache.persist("cache.txt").unwrap();
    /// ```
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), CacheError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| CacheError::IoError(e))?;

        let mut writer = BufWriter::new(file);

        for key in &self.usage_order {
            if let Some(value) = self.elements.get(key) {
                writeln!(writer, "{}\t{}", key, value)
                    .map_err(|e| CacheError::IoError(e))?;
            }
        }

        writer.flush().map_err(|e| CacheError::IoError(e))?;
        Ok(())
    }
}

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.elements.contains_key(key) {
            self.move_to_recently_used(key);
            self.elements.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.elements.len() >= self.capacity && !self.elements.contains_key(&key) {
            // Supprimer l'élément le moins récemment utilisé
            if let Some(lru_key) = self.usage_order.first().cloned() {
                self.elements.remove(&lru_key);
                self.usage_order.remove(0);
            }
        }

        // Si la clé existe déjà, la mettre à jour
        if self.elements.contains_key(&key) {
            self.elements.insert(key.clone(), value);
            self.move_to_recently_used(&key);
        } else {
            // Sinon, ajouter le nouvel élément
            self.elements.insert(key.clone(), value);
            self.usage_order.push(key);
        }
    }
}