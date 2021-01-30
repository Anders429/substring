extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate substring;
use substring::Substring;

fn bench_substring(c: &mut Criterion) {
    c.bench_function("substring", |b| {
        b.iter(|| black_box("Hello, world!".substring(2, 9)));
    });
}

criterion_group!(benches, bench_substring);
criterion_main!(benches);
