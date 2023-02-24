use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod ecdsa;
use crate::ecdsa::generate_sign_verify;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ecdsa function", move |b| {
        b.iter(|| {
            let message = String::from("Hello world!");
            generate_sign_verify(message.clone());
        })
    });
}

criterion_group! {
    name = ecdsa;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(ecdsa);
