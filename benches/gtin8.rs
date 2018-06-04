#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate gtin_validate;
use gtin_validate::gtin8;

fn bench_check(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "gtin8 check",
        |b, &code| b.iter(|| gtin8::check(code)),
        &["00000000"],
    );

    c.bench_function_over_inputs(
        "gtin8 check - too long",
        |b, &code| b.iter(|| gtin8::check(code)),
        &["01234567890123456789012345678901234567890123456789"],
    );
}

criterion_group!(gtin8, bench_check);
criterion_main!(gtin8);
