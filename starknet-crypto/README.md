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
ecdsa_get_public_key    time:   [125.53 µs 125.63 µs 125.75 µs]
ecdsa_recover           time:   [421.74 µs 421.93 µs 422.16 µs]
ecdsa_sign              time:   [170.30 µs 170.50 µs 170.71 µs]
ecdsa_verify            time:   [428.34 µs 428.73 µs 429.17 µs]
pedersen_hash           time:   [33.379 µs 33.435 µs 33.521 µs]
poseidon_hash           time:   [12.552 µs 12.571 µs 12.595 µs]
poseidon_hash_single    time:   [12.572 µs 12.587 µs 12.601 µs]
poseidon_hash_many      time:   [25.048 µs 25.089 µs 25.137 µs]
rfc6979_generate_k      time:   [1.4810 µs 1.4817 µs 1.4827 µs]
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
ecdsa_get_public_key    time:   [1.0093 ms 1.0118 ms 1.0147 ms]
ecdsa_recover           time:   [3.0610 ms 3.0627 ms 3.0646 ms]
ecdsa_sign              time:   [1.0584 ms 1.0600 ms 1.0615 ms]
ecdsa_verify            time:   [3.0273 ms 3.0309 ms 3.0345 ms]
pedersen_hash           time:   [234.12 µs 234.30 µs 234.49 µs]
poseidon_hash           time:   [90.892 µs 91.032 µs 91.166 µs]
poseidon_hash_single    time:   [90.358 µs 90.404 µs 90.451 µs]
poseidon_hash_many      time:   [180.93 µs 181.13 µs 181.35 µs]
rfc6979_generate_k      time:   [9.2623 µs 9.2793 µs 9.2979 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesome [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

The Poseidon hash implementation was also ported from [`pathfinder`](https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
