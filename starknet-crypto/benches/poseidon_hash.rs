use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::{poseidon_hash, poseidon_hash_many, poseidon_hash_single, FieldElement};

pub fn criterion_benchmark(c: &mut Criterion) {
    let e0 = hex!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let e1 = hex!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
    let e2 = hex!("013126bb6143939a9da42c90ce6f4a48f8430f9955a62367571a348e9c65177c");

    let e0 = FieldElement::from_bytes_be(&e0).unwrap();
    let e1 = FieldElement::from_bytes_be(&e1).unwrap();
    let e2 = FieldElement::from_bytes_be(&e2).unwrap();

    c.bench_function("poseidon_hash", |b| {
        b.iter(|| {
            black_box(poseidon_hash(e0, e1));
        });
    });

    c.bench_function("poseidon_hash_single", |b| {
        b.iter(|| {
            black_box(poseidon_hash_single(e0));
        });
    });

    c.bench_function("poseidon_hash_many", |b| {
        b.iter(|| {
            black_box(poseidon_hash_many(&[e0, e1, e2]));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
