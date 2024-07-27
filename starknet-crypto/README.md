# Low-level cryptography utilities for Starknet

`starknet-crypto` contains utilities for performing **low-level** cryptographic operations in Starknet:

- ECDSA operations
  - Signing hashes
  - Verifying signatures
  - Recovering public keys from signatures
- Pedersen hash
- Poseidon hash
- RFC-6979

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
ecdsa_get_public_key    time:   [62.223 µs 62.231 µs 62.240 µs]
ecdsa_recover           time:   [253.15 µs 253.47 µs 254.13 µs]
ecdsa_sign              time:   [95.633 µs 95.649 µs 95.668 µs]
ecdsa_verify            time:   [255.70 µs 255.77 µs 255.84 µs]
pedersen_hash           time:   [13.021 µs 13.023 µs 13.024 µs]
poseidon_hash           time:   [5.0139 µs 5.0148 µs 5.0155 µs]
poseidon_hash_single    time:   [5.0239 µs 5.0381 µs 5.0543 µs]
poseidon_hash_many      time:   [10.077 µs 10.087 µs 10.100 µs]
rfc6979_generate_k      time:   [4.5806 µs 4.5821 µs 4.5836 µs]
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
ecdsa_get_public_key    time:   [333.64 µs 334.07 µs 334.48 µs]
ecdsa_recover           time:   [1.1177 ms 1.1207 ms 1.1248 ms]
ecdsa_sign              time:   [386.33 µs 387.42 µs 388.68 µs]
ecdsa_verify            time:   [1.1246 ms 1.1280 ms 1.1320 ms]
pedersen_hash           time:   [64.934 µs 64.962 µs 64.993 µs]
poseidon_hash           time:   [20.745 µs 20.772 µs 20.825 µs]
poseidon_hash_single    time:   [20.790 µs 20.813 µs 20.837 µs]
poseidon_hash_many      time:   [41.878 µs 41.911 µs 41.945 µs]
rfc6979_generate_k      time:   [11.564 µs 11.566 µs 11.569 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesome [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

The Poseidon hash implementation was also ported from [`pathfinder`](https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
