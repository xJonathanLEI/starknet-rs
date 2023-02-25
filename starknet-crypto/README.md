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
ecdsa_get_public_key    time:   [1.3717 ms 1.3722 ms 1.3727 ms]
ecdsa_sign              time:   [1.3782 ms 1.3788 ms 1.3794 ms]
ecdsa_verify            time:   [2.9048 ms 2.9061 ms 2.9075 ms]
pedersen_hash           time:   [29.558 µs 29.594 µs 29.635 µs]
poseidon_hash           time:   [11.650 µs 11.657 µs 11.664 µs]
poseidon_hash_single    time:   [11.609 µs 11.615 µs 11.620 µs]
poseidon_hash_many      time:   [23.320 µs 23.344 µs 23.375 µs]
rfc6979_generate_k      time:   [1.3761 µs 1.3769 µs 1.3779 µs]
```

### WebAssembly

_(With its excellent wasm performance, results are only provided for Node.js here. Check out the [benchmark page](../BENCHMARK.md) for running the benchmark on other runtimes)._

Runtime version:

```console
$ node --version
v18.14.2
```

Node.js results:

```log
ecdsa_get_public_key    time:   [2.3087 ms 2.3096 ms 2.3105 ms]
ecdsa_sign              time:   [2.3152 ms 2.3163 ms 2.3177 ms]
ecdsa_verify            time:   [5.2511 ms 5.2533 ms 5.2556 ms]
pedersen_hash           time:   [124.09 µs 124.20 µs 124.36 µs]
poseidon_hash           time:   [43.965 µs 44.022 µs 44.084 µs]
poseidon_hash_single    time:   [43.942 µs 43.992 µs 44.055 µs]
poseidon_hash_many      time:   [87.895 µs 87.976 µs 88.063 µs]
rfc6979_generate_k      time:   [8.6301 µs 8.6353 µs 8.6414 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesome [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

The Poseidon hash implementation was also ported from [`pathfinder`](https://github.com/eqlabs/pathfinder/blob/ab3f2e849cd94d5dc3c7c02040adff4ad7d0597b/crates/stark_poseidon/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
