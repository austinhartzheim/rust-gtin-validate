#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin8;

fn bench_check(c: &mut Criterion) {
    c.bench_function("gtin8 check", |b| {
        b.iter(|| gtin8::check(criterion::black_box("00000000")))
    });
}

criterion_group!(gtin8, bench_check);
criterion_main!(gtin8);
