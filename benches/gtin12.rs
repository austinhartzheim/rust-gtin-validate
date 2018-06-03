#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin12;

fn bench_check(c: &mut Criterion) {
    c.bench_function("gtin12 check", |b| {
        b.iter(|| gtin12::check(criterion::black_box("000000000000")))
    });
}

criterion_group!(gtin12, bench_check);
criterion_main!(gtin12);
