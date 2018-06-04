#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin13;

fn bench_check(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "gtin13 check",
        |b, &code| b.iter(|| gtin13::check(code)),
        &["0000000000000"],
    );

    c.bench_function_over_inputs(
        "gtin13 check - too long",
        |b, &code| b.iter(|| gtin13::check(code)),
        &["01234567890123456789012345678901234567890123456789"],
    );
}

criterion_group!(gtin13, bench_check);
criterion_main!(gtin13);
