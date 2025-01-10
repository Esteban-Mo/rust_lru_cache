use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));

        assert_eq!(cache.get(&"A"), Some(&String::from("value_a")));
        assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
        assert_eq!(cache.get(&"C"), Some(&String::from("value_c")));
        assert_eq!(cache.get(&"D"), None);
    }

    #[test]
    fn test_eviction() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));

        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"D"), Some(&String::from("value_d")));
    }

    #[test]
    fn test_update_order() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));

        cache.get(&"B");
        cache.put("D", String::from("value_d"));

        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
    }
}