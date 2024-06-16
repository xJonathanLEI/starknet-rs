use criterion::{black_box, criterion_group, criterion_main, Criterion};
use starknet_core::types::contract::SierraClass;

pub fn criterion_benchmark(c: &mut Criterion) {
    let contract_artifact: SierraClass = serde_json::from_str(include_str!(
        "../test-data/contracts/cairo1/artifacts/erc20_sierra.txt"
    ))
    .unwrap();

    c.bench_function("sierra_class_hash", |b| {
        b.iter(|| {
            black_box(&contract_artifact).class_hash().unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
