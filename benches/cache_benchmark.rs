use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::lru::{Cache, traits::CacheTrait};

fn cache_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cache Operations");
    
    // Test de remplissage du cache
    group.bench_function("cache fill", |b| {
        let mut cache = Cache::new(1000);
        b.iter(|| {
            for i in 0..1000 {
                cache.put(black_box(i), black_box(format!("value_{}", i)));
            }
        });
    });

    // Test d'accès séquentiel avec rotation
    group.bench_function("sequential access with rotation", |b| {
        let mut cache = Cache::new(1000);
        // Remplissage initial
        for i in 0..1000 {
            cache.put(i, format!("value_{}", i));
        }
        b.iter(|| {
            for i in 0..2000 {
                let key = i % 1500;
                black_box(cache.get(&black_box(key)));
            }
        });
    });

    // Test de remplacement LRU
    group.bench_function("lru replacement", |b| {
        let mut cache = Cache::new(1000);
        b.iter(|| {
            for i in 0..2000 {
                cache.put(black_box(i), black_box(format!("value_{}", i)));
                if i % 2 == 0 {
                    black_box(cache.get(&black_box(i - 1)));
                }
            }
        });
    });

    // Test de pattern d'accès réaliste
    group.bench_function("realistic access pattern", |b| {
        let mut cache = Cache::new(1000);
        let mut i = 0;
        b.iter(|| {
            let key = if i % 5 == 0 {
                black_box(i % 2000)
            } else {
                black_box(i % 200)
            };
            if i % 3 == 0 {
                cache.put(black_box(key), black_box(format!("value_{}", key)));
            } else {
                black_box(cache.get(&black_box(key)));
            }
            i += 1;
        });
    });

    group.finish();
}

criterion_group!(benches, cache_benchmark);
criterion_main!(benches);