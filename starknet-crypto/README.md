# Low-level cryptography utilities for Starknet

`starknet-crypto` contains utilities for performing **low-level** cryptographic operations in Starknet.

> _You're advised to use high-level crypto utilities implemented by the `starknet-core` crate (or use it through the `starknet::core` re-export) if you're not familiar with cryptographic primitives. Using these low-level functions incorrectly could result in leaking your private key, for example._

> _This library does not provide constant-time guarantees._

## **WARNING**

While it has been tested against data randomly generated from [`cairo-lang`](https://github.com/starkware-libs/cairo-lang), this crate is _NOT_ audited or reviewed for security. **Use at your own risk**.

If you're a cryptographer, you're welcome to contribute by reviewing the implementation :)

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 22.04.1 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
ecdsa_get_public_key    time:   [1.4787 ms 1.4878 ms 1.4979 ms]
ecdsa_sign              time:   [1.4732 ms 1.4856 ms 1.4998 ms]
ecdsa_verify            time:   [433.81 µs 435.56 µs 437.35 µs]
pedersen_hash           time:   [30.661 µs 30.954 µs 31.298 µs]
poseidon_hash           time:   [12.209 µs 12.252 µs 12.297 µs]
poseidon_hash_single    time:   [12.159 µs 12.256 µs 12.362 µs]
poseidon_hash_many      time:   [23.839 µs 23.945 µs 24.056 µs]
rfc6979_generate_k      time:   [1.4203 µs 1.4244 µs 1.4290 µs]
```

### WebAssembly

_(With its excellent wasm performance, results are only provided for Node.js here. Check out the [benchmark page](../BENCHMARK.md) for running the benchmark on other runtimes)._

Runtime version:

```console
$ node --version
v18.16.0
```

Node.js results:

```log
ecdsa_get_public_key    time:   [2.6912 ms 2.6933 ms 2.6958 ms]
ecdsa_sign              time:   [2.7180 ms 2.7341 ms 2.7516 ms]
ecdsa_verify            time:   [3.0274 ms 3.0375 ms 3.0480 ms]
pedersen_hash           time:   [228.55 µs 229.09 µs 229.68 µs]
poseidon_hash           time:   [84.220 µs 84.399 µs 84.655 µs]
poseidon_hash_single    time:   [84.330 µs 84.477 µs 84.640 µs]
poseidon_hash_many      time:   [167.52 µs 167.86 µs 168.24 µs]
rfc6979_generate_k      time:   [9.6608 µs 9.7225 µs 9.7981 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesome [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

The Poseidon hash implementation was also ported from [`pathfinder`](https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
