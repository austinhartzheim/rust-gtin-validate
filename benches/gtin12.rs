#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin12;

fn bench_check(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "gtin12 check",
        |b, &code| b.iter(|| gtin12::check(code)),
        &["000000000000"],
    );
}

criterion_group!(gtin12, bench_check);
criterion_main!(gtin12);
