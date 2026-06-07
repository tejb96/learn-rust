use benchmarks_lesson::fib_iter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib_iter 20", |b| b.iter(|| fib_iter(black_box(20))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
