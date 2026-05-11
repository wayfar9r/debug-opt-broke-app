use broken_app::{algo, sum_even};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).collect();
    c.bench_function("sum_even_broken", |b| b.iter(|| sum_even(black_box(&data))));
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("slow_fib_broken", |b| {
        b.iter(|| algo::slow_fib(black_box(32)))
    });
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("slow_dedup_broken", |b| {
        b.iter_batched(
            || data.clone(),
            |v| {
                let _ = algo::slow_dedup(black_box(&v));
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_sum_even, bench_fib, bench_dedup);
criterion_main!(benches);
