use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ff::PrimeField;
use hex_literal::hex;
use starknet_crypto::{rfc6979_generate_k, FieldElement, FieldElementRepr};

pub fn criterion_benchmark(c: &mut Criterion) {
    let message_hash = hex!("010b559a3b4dc1b7137d90521cb413b397ff07963214d128a92d65aec7182f68");
    let private_key = hex!("07e3184f4bef18f371bc53fc412dff1b30dbc94f758490fb8e2349bae647a642");
    let seed = hex!("03fe27199aaad4e700559e2436a919f4de70def585a6deb2f4c087fdf6a27c1b");

    let message_hash = FieldElement::from_repr(FieldElementRepr(message_hash)).unwrap();
    let private_key = FieldElement::from_repr(FieldElementRepr(private_key)).unwrap();
    let seed = FieldElement::from_repr(FieldElementRepr(seed)).unwrap();

    c.bench_function("rfc6979_generate_k", |b| {
        b.iter(|| {
            black_box(rfc6979_generate_k(&message_hash, &private_key, &seed));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
