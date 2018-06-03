#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin14;

fn bench_check(c: &mut Criterion) {
    c.bench_function("gtin14 check", |b| {
        b.iter(|| gtin14::check(criterion::black_box("00000000000000")))
    });
}

criterion_group!(gtin14, bench_check);
criterion_main!(gtin14);
