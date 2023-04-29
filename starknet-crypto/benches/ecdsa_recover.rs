use criterion::{black_box, criterion_group, criterion_main, Criterion};
use starknet_crypto::{recover, sign, FieldElement};

pub fn criterion_benchmark(c: &mut Criterion) {
    let private_key = FieldElement::from_hex_be(
        "0000000000000000000000000000000000000000000000000000000000000001",
    )
    .unwrap();
    let message = FieldElement::from_hex_be(
        "0000000000000000000000000000000000000000000000000000000000000001",
    )
    .unwrap();
    let k = FieldElement::from_hex_be(
        "0000000000000000000000000000000000000000000000000000000000000001",
    )
    .unwrap();

    let signature = sign(&private_key, &message, &k).unwrap();

    c.bench_function("ecdsa_recover", |b| {
        b.iter(|| {
            _ = black_box(recover(&message, &signature.r, &signature.s, &signature.v).unwrap());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
