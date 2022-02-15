use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::{get_public_key, FieldElement};

pub fn criterion_benchmark(c: &mut Criterion) {
    let private_key = hex!("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d");

    let private_key = FieldElement::from_bytes_be(&private_key).unwrap();

    c.bench_function("ecdsa_get_public_key", |b| {
        b.iter(|| {
            black_box(get_public_key(&private_key));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
