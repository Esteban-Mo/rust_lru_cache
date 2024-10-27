use std::collections::HashMap;

pub struct Cache {
    capacity: usize,
    data: HashMap<String, String>,
}

impl Cache {
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cache() {
        let mut cache = Cache::new(3);
        cache.put(String::from("test"), String::from("value"));
    }

    #[test]
    fn test_get_value() {
        let mut cache = Cache::new(3);
        cache.put(String::from("key"), String::from("value"));
        assert_eq!(cache.data.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut cache = Cache::new(3);
        cache.put(String::from("key"), String::from("value"));
        assert_eq!(cache.get("key"), Some(&String::from("value")));
        assert_eq!(cache.get("unknown"), None);
    }
}
