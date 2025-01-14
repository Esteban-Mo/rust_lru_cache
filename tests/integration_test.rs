use lru_cache::lru::{Cache, traits::CacheTrait};

///////////////////////////////////////////////////////////////////////////////
// Test d'intégration de base
///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_cache_integration() {
    let mut cache = Cache::new(3);
    
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&1), Some(&"one"));
    
    cache.put(4, "four");
    assert_eq!(cache.get(&3), None);
    
    let mut complex_cache: Cache<String, Vec<i32>> = Cache::new(2);
    complex_cache.put("numbers".to_string(), vec![1, 2, 3]);
    complex_cache.put("more_numbers".to_string(), vec![4, 5, 6]);
    
    assert_eq!(complex_cache.get(&"numbers".to_string()), Some(&vec![1, 2, 3]));
}

///////////////////////////////////////////////////////////////////////////////
// Test d'intégration du trait
///////////////////////////////////////////////////////////////////////////////

#[test]
fn test_trait_implementation() {
    fn use_cache_trait<C: CacheTrait<i32, String>>(cache: &mut C) {
        cache.put(1, "one".to_string());
        assert_eq!(cache.get(&1), Some(&"one".to_string()));
    }
    
    let mut cache = Cache::new(3);
    use_cache_trait(&mut cache);
}

#[test]
fn test_persistent_cache() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let cache_path = "test_cache.txt";

    // Nettoyage préalable
    let _ = fs::remove_file(cache_path);

    {
        // Test d'écriture
        let mut cache = Cache::new_persistent(3, cache_path)?;
        cache.put(1, 100);
        cache.put(2, 200);
        cache.put(3, 300);
        cache.persist(cache_path)?;
    }

    {
        // Test de lecture
        let mut cache = Cache::new_persistent(3, cache_path)?;
        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.get(&2), Some(&200));
        assert_eq!(cache.get(&3), Some(&300));
        
        // Test de l'ordre LRU
        cache.put(4, 400);
        assert_eq!(cache.get(&1), None); // Le plus ancien devrait être évincé
    }

    // Nettoyage
    fs::remove_file(cache_path)?;
    Ok(())
} 