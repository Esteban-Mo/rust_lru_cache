use lru_cache::lru::{Cache, traits::CacheTrait};
use std::fs;

#[test]
fn test_cache_integration() {
    let mut cache = Cache::new(3);
    
    // Test de base avec différents types
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&1), Some(&"one"));
    
    // Test d'éviction
    cache.put(4, "four");
    assert_eq!(cache.get(&3), None);
    
    // Test avec des types plus complexes
    let mut complex_cache: Cache<String, Vec<i32>> = Cache::new(2);
    complex_cache.put("numbers".to_string(), vec![1, 2, 3]);
    complex_cache.put("more_numbers".to_string(), vec![4, 5, 6]);
    
    assert_eq!(complex_cache.get(&"numbers".to_string()), Some(&vec![1, 2, 3]));
}

#[test]
fn test_persistent_cache() {
    use lru_cache::lru::persistent::PersistentCache;
    
    let test_file = "test_cache.json";
    
    // Nettoyage avant le test
    let _ = fs::remove_file(test_file);
    
    {
        let mut cache = PersistentCache::new(3, test_file).unwrap();
        cache.put("test1", 1);
        cache.put("test2", 2);
        cache.put("test3", 3);
        
        assert_eq!(cache.get(&"test2"), Some(&2));
    }
    
    // Création d'un nouveau cache pour vérifier la persistance
    {
        let mut cache = PersistentCache::new(3, test_file).unwrap();
        assert_eq!(cache.get(&"test1"), Some(&1));
        assert_eq!(cache.get(&"test2"), Some(&2));
        assert_eq!(cache.get(&"test3"), Some(&3));
    }
    
    // Nettoyage après le test
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_trait_implementation() {
    fn use_cache_trait<C: CacheTrait<i32, String>>(cache: &mut C) {
        cache.put(1, "one".to_string());
        assert_eq!(cache.get(&1), Some(&"one".to_string()));
    }
    
    let mut cache = Cache::new(3);
    use_cache_trait(&mut cache);
} 