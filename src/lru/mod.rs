use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::traits::CacheTrait;

pub mod traits;

///////////////////////////////////////////////////////////////////////////////
// Structure principale du cache
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Cache<K, V> 
where 
    K: Hash + Eq,
{
    pub(crate) capacity: usize,
    pub(crate) elements: HashMap<K, V>,
    pub(crate) usage_order: Vec<K>,
}

///////////////////////////////////////////////////////////////////////////////
// Implémentation de base du cache
///////////////////////////////////////////////////////////////////////////////

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
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

    fn move_to_recently_used(&mut self, key: &K) {
        if let Some(pos) = self.usage_order.iter().position(|k| k == key) {
            self.usage_order.remove(pos);
            self.usage_order.push(key.clone());
        }
    }

    fn remove_oldest(&mut self) {
        if let Some(oldest_key) = self.usage_order.first().cloned() {
            self.elements.remove(&oldest_key);
            self.usage_order.remove(0);
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.usage_order.clear();
    }
}

///////////////////////////////////////////////////////////////////////////////
// Implémentation du trait CacheTrait
///////////////////////////////////////////////////////////////////////////////

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where 
    K: Hash + Eq + Clone,
{
    fn put(&mut self, key: K, value: V) {
        if self.elements.contains_key(&key) {
            self.update_existing_key(&key, value);
        } else {
            self.insert_new_key(key, value);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.elements.contains_key(key) {
            self.move_to_recently_used(key);
            self.elements.get(key)
        } else {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Méthodes internes du cache
///////////////////////////////////////////////////////////////////////////////

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
    fn update_existing_key(&mut self, key: &K, value: V) {
        self.elements.insert(key.clone(), value);
        self.move_to_recently_used(key);
    }

    fn insert_new_key(&mut self, key: K, value: V) {
        if self.elements.len() >= self.capacity {
            self.remove_oldest();
        }
        self.elements.insert(key.clone(), value);
        self.usage_order.push(key);
    }
}

///////////////////////////////////////////////////////////////////////////////
// Implémentation de Default
///////////////////////////////////////////////////////////////////////////////

impl<K, V> Default for Cache<K, V>
where 
    K: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new(16)
    }
}