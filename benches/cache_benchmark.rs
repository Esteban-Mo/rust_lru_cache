use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::lru::{Cache, traits::CacheTrait};

fn cache_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cache Operations");
    
    group.bench_function("put operation", |b| {
        let mut cache = Cache::new(1000);
        let mut i = 0;
        b.iter(|| {
            cache.put(black_box(i), black_box(format!("value_{}", i)));
            i += 1;
        });
    });
    
    group.bench_function("get hit", |b| {
        let mut cache = Cache::new(1000);
        for i in 0..1000 {
            cache.put(i, format!("value_{}", i));
        }
        let mut i = 0;
        b.iter(|| {
            black_box(cache.get(&black_box(i % 1000)));
            i += 1;
        });
    });
    
    group.bench_function("get miss", |b| {
        let mut cache = Cache::new(1000);
        for i in 0..1000 {
            cache.put(i, format!("value_{}", i));
        }
        let mut i = 1000;
        b.iter(|| {
            black_box(cache.get(&black_box(i)));
            i += 1;
        });
    });
    
    group.bench_function("mixed operations", |b| {
        let mut cache = Cache::new(1000);
        let mut i = 0;
        b.iter(|| {
            if i % 2 == 0 {
                cache.put(black_box(i), black_box(format!("value_{}", i)));
            } else {
                black_box(cache.get(&black_box(i - 1)));
            }
            i += 1;
        });
    });
    
    group.finish();
}

criterion_group!(benches, cache_benchmark);
criterion_main!(benches); 