use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::{splish, sploosh};



pub fn benchy(c: &mut Criterion) {
    c.bench_function("a benchmark", |b| b.iter(|| splish(black_box(5), black_box(5))));
}

criterion_group!(benches, benchy);
criterion_main!(benches);
