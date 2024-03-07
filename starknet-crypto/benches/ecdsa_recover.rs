use criterion::{black_box, criterion_group, criterion_main, Criterion};
use starknet_crypto::{recover, sign};
use starknet_types_core::felt::Felt;

pub fn criterion_benchmark(c: &mut Criterion) {
    let private_key =
        Felt::from_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    let message =
        Felt::from_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    let k =
        Felt::from_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    let signature = sign(&private_key, &message, &k).unwrap();

    c.bench_function("ecdsa_recover", |b| {
        b.iter(|| {
            _ = black_box(recover(&message, &signature.r, &signature.s, &signature.v).unwrap());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
