use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::{verify, FieldElement};

pub fn criterion_benchmark(c: &mut Criterion) {
    let stark_key = hex!("0565ee8f4203a04fbd5de77c678bc3738538f35c0871e377cdc45fcfa79e6bd9");
    let msg_hash = hex!("010aaf60f545a5b9a55463fbb56f35dfdfe8010ff1d95283afe1b14e07cb8f61");
    let r_bytes = hex!("03879bf25e6919880960131bb3b614c40d942791f83dac999d28028824c2d712");
    let s_bytes = hex!("01f2a4527241c802e0885cf3aeac5bdfdbb559c09a45e1b745addae358f6c03b");

    let stark_key = FieldElement::from_bytes_be(&stark_key).unwrap();
    let msg_hash = FieldElement::from_bytes_be(&msg_hash).unwrap();
    let r_bytes = FieldElement::from_bytes_be(&r_bytes).unwrap();
    let s_bytes = FieldElement::from_bytes_be(&s_bytes).unwrap();

    c.bench_function("ecdsa_verify", |b| {
        b.iter(|| {
            black_box(verify(&stark_key, &msg_hash, &r_bytes, &s_bytes).unwrap());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
