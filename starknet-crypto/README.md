# Low-level cryptography utilities for StarkNet

`starknet-crypto` contains utilities for performing **low-level** cryptographic operations in StarkNet.

> _You're advised to use high-level crypto utilities implemented by the `starknet-core` crate (or use it through the `starknet::core` re-export) if you're not familiar with cryptographic primitives. Using these low-level functions incorrectly could result in leaking your private key, for example._

> _This library does not provide constant-time guarantees._

## **WARNING**

While it has been tested against data randomly generated from [`cairo-lang`](https://github.com/starkware-libs/cairo-lang), this crate is _NOT_ audited or reviewed for security. **Use at your own risk**.

If you're a cryptographer, you're welcome to contribute by reviewing the implementation :)

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
ecdsa_get_public_key    time:   [1.5350 ms 1.5461 ms 1.5629 ms]
ecdsa_sign              time:   [1.5379 ms 1.5420 ms 1.5474 ms]
ecdsa_verify            time:   [3.2405 ms 3.2443 ms 3.2487 ms]
pedersen_hash           time:   [31.775 µs 31.988 µs 32.273 µs]
rfc6979_generate_k      time:   [2.3819 µs 2.3904 µs 2.4020 µs]
```

### WebAssembly

Runtime versions:

```console
$ wasmer --version
wasmer 2.3.0
$ wasmtime --version
wasmtime-cli 2.0.2
$ node --version
v18.12.1
$ wasmer-js --version
wasmer-js 0.4.1
```

`wasmer` results:

```log
ecdsa_get_public_key    time:   [3.1780 ms 3.2041 ms 3.2374 ms]
ecdsa_sign              time:   [3.2371 ms 3.2554 ms 3.2785 ms]
ecdsa_verify            time:   [7.5052 ms 7.5168 ms 7.5297 ms]
pedersen_hash           time:   [267.19 µs 268.74 µs 270.89 µs]
rfc6979_generate_k      time:   [12.501 µs 12.512 µs 12.525 µs]
```

`wasmtime` results:

```log
ecdsa_get_public_key    time:   [2.9626 ms 2.9677 ms 2.9734 ms]
ecdsa_sign              time:   [2.9489 ms 2.9603 ms 2.9730 ms]
ecdsa_verify            time:   [6.8464 ms 6.8792 ms 6.9221 ms]
pedersen_hash           time:   [220.62 µs 221.77 µs 223.72 µs]
rfc6979_generate_k      time:   [11.263 µs 11.281 µs 11.304 µs]
```

Node.js results:

```log
ecdsa_get_public_key    time:   [2.7033 ms 2.7220 ms 2.7461 ms]
ecdsa_sign              time:   [2.7405 ms 2.7431 ms 2.7461 ms]
ecdsa_verify            time:   [6.5923 ms 6.6322 ms 6.6816 ms]
pedersen_hash           time:   [230.24 µs 230.84 µs 231.74 µs]
rfc6979_generate_k      time:   [9.9566 µs 9.9891 µs 10.032 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesone [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
