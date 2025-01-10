use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::traits::CacheTrait;

pub mod traits;

#[derive(Debug)]
pub struct Cache<K, V> 
where 
    K: Hash + Eq,
{
    pub(crate) capacity: usize,
    pub(crate) map: HashMap<K, V>,
    pub(crate) order: Vec<K>,
}

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("La capacité du cache ne peut pas être nulle");
        }
        
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

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
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

impl<K, V> Default for Cache<K, V>
where 
    K: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new(16)
    }
}