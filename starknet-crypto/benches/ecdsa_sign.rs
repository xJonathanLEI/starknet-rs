use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::{sign, FieldElement};

pub fn criterion_benchmark(c: &mut Criterion) {
    let private_key = hex!("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d");
    let message = hex!("010aaf60f545a5b9a55463fbb56f35dfdfe8010ff1d95283afe1b14e07cb8f61");
    let k = hex!("075414c392c57a61417fc1702ad6fa83d12541690963915646617b59451972b3");

    let private_key = FieldElement::from_bytes_be(&private_key).unwrap();
    let message = FieldElement::from_bytes_be(&message).unwrap();
    let k = FieldElement::from_bytes_be(&k).unwrap();

    c.bench_function("ecdsa_sign", |b| {
        b.iter(|| {
            black_box(sign(&private_key, &message, &k).unwrap());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
