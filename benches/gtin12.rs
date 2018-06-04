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

    c.bench_function_over_inputs(
        "gtin12 check - too long",
        |b, &code| b.iter(|| gtin12::check(code)),
        &["01234567890123456789012345678901234567890123456789"],
    );
}

criterion_group!(gtin12, bench_check);
criterion_main!(gtin12);
