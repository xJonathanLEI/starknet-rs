use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::{pedersen_hash, FieldElement};

// Benchmark taken from pathfinder for performance comparison:
// https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/benches/pedersen.rs

pub fn criterion_benchmark(c: &mut Criterion) {
    let e0 = hex!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let e1 = hex!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");

    let e0 = FieldElement::from_bytes_be(&e0).unwrap();
    let e1 = FieldElement::from_bytes_be(&e1).unwrap();

    c.bench_function("pedersen_hash", |b| {
        b.iter(|| {
            black_box(pedersen_hash(&e0, &e1));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
