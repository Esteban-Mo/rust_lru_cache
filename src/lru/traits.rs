//! Module définissant les traits pour le cache LRU.

/// Trait définissant les opérations de base d'un cache.
/// 
/// Ce trait fournit les méthodes essentielles pour interagir avec un cache :
/// - Récupérer une valeur (`get`)
/// - Ajouter ou mettre à jour une valeur (`put`)
/// 
/// # Type Parameters
/// 
/// * `K` - Le type de la clé
/// * `V` - Le type de la valeur
/// 
/// # Exemples
/// 
/// ```
/// use lru_cache::lru::traits::CacheTrait;
/// use lru_cache::lru::Cache;
/// 
/// fn utiliser_cache<C: CacheTrait<String, i32>>(cache: &mut C) {
///     cache.put("un".to_string(), 1);
///     assert_eq!(cache.get(&"un".to_string()), Some(&1));
/// }
/// 
/// let mut cache = Cache::new(2);
/// utiliser_cache(&mut cache);
/// ```
pub trait CacheTrait<K, V> {
    /// Récupère une référence à la valeur associée à la clé.
    /// 
    /// Met également à jour l'ordre d'utilisation du cache.
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Ajoute ou met à jour une paire clé-valeur dans le cache.
    /// 
    /// Si le cache est plein, l'élément le moins récemment utilisé est supprimé.
    fn put(&mut self, key: K, value: V);
}