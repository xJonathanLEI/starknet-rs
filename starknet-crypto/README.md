# Low-level cryptography utilities for Starknet

`starknet-crypto` contains utilities for performing **low-level** cryptographic operations in Starknet.

> _You're advised to use high-level crypto utilities implemented by the `starknet-core` crate (or use it through the `starknet::core` re-export) if you're not familiar with cryptographic primitives. Using these low-level functions incorrectly could result in leaking your private key, for example._

> _This library does not provide constant-time guarantees._

## **WARNING**

While it has been tested against data randomly generated from [`cairo-lang`](https://github.com/starkware-libs/cairo-lang), this crate is _NOT_ audited or reviewed for security. **Use at your own risk**.

If you're a cryptographer, you're welcome to contribute by reviewing the implementation :)

## Benchmark

These results were generated on the author's machine with _Apple M3 Max_ running _macOS 14.5_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
ecdsa_get_public_key    time:   [52.416 µs 52.456 µs 52.506 µs]
ecdsa_recover           time:   [233.25 µs 234.29 µs 235.43 µs]
ecdsa_sign              time:   [87.730 µs 87.967 µs 88.211 µs]
ecdsa_verify            time:   [239.97 µs 240.65 µs 241.46 µs]
pedersen_hash           time:   [15.635 µs 15.668 µs 15.695 µs]
poseidon_hash           time:   [4.6606 µs 4.6828 µs 4.7052 µs]
poseidon_hash_single    time:   [4.7146 µs 4.7243 µs 4.7341 µs]
poseidon_hash_many      time:   [10.101 µs 10.419 µs 10.713 µs]
rfc6979_generate_k      time:   [4.7230 µs 4.7346 µs 4.7469 µs]
```

### WebAssembly

_(Results are only provided for `wasmtime` here. Check out the [benchmark page](../BENCHMARK.md) for running the benchmark on other runtimes)._

Runtime version:

```console
$ wasmtime --version
wasmtime-cli 21.0.1 (cedf9aa0f 2024-05-22)
```

`wasmtime` results:

```log
ecdsa_get_public_key    time:   [264.92 µs 265.06 µs 265.21 µs]
ecdsa_recover           time:   [921.34 µs 922.06 µs 922.88 µs]
ecdsa_sign              time:   [311.44 µs 311.58 µs 311.72 µs]
ecdsa_verify            time:   [916.04 µs 917.13 µs 918.73 µs]
pedersen_hash           time:   [71.713 µs 71.751 µs 71.795 µs]
poseidon_hash           time:   [19.333 µs 19.359 µs 19.381 µs]
poseidon_hash_single    time:   [19.223 µs 19.234 µs 19.245 µs]
poseidon_hash_many      time:   [39.004 µs 39.048 µs 39.089 µs]
rfc6979_generate_k      time:   [11.798 µs 11.807 µs 11.817 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesome [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

The Poseidon hash implementation was also ported from [`pathfinder`](https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
