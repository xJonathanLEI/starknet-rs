use criterion::{black_box, criterion_group, criterion_main, Criterion};
use starknet_core::types::contract::legacy::LegacyContractClass;

pub fn criterion_benchmark(c: &mut Criterion) {
    // Using the latest OZ account contract for the benchmark
    let contract_artifact: LegacyContractClass = serde_json::from_str(include_str!(
        "../test-data/contracts/cairo0/artifacts/oz_account.txt"
    ))
    .unwrap();

    c.bench_function("class_hash", |b| {
        b.iter(|| {
            black_box(&contract_artifact).class_hash().unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
