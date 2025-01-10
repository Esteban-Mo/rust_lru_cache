use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::traits::CacheTrait;

pub mod traits;

/// Structure représentant un cache LRU (Least Recently Used)
pub struct Cache<K, V> 
where 
    K: Hash + Eq,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    fn update_order(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
            self.order.push(key.clone());
        }
    }
}

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where 
    K: Hash + Eq + Clone,
{
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.update_order(&key);
        } else {
            if self.map.len() >= self.capacity {
                if let Some(lru_key) = self.order.first().cloned() {
                    self.map.remove(&lru_key);
                    self.order.remove(0);
                }
            }
            self.map.insert(key.clone(), value);
            self.order.push(key);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.update_order(key);
            self.map.get(key)
        } else {
            None
        }
    }
}