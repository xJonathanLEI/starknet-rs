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
ecdsa_get_public_key    time:   [1.4756 ms 1.4761 ms 1.4765 ms]
ecdsa_sign              time:   [1.4728 ms 1.4735 ms 1.4743 ms]
ecdsa_verify            time:   [3.1314 ms 3.1331 ms 3.1348 ms]
pedersen_hash           time:   [223.93 µs 224.41 µs 225.04 µs]
rfc6979_generate_k      time:   [2.2909 µs 2.2922 µs 2.2935 µs]
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
ecdsa_get_public_key    time:   [3.0703 ms 3.0806 ms 3.0914 ms]
ecdsa_sign              time:   [3.1632 ms 3.1665 ms 3.1717 ms]
ecdsa_verify            time:   [6.9936 ms 7.0168 ms 7.0426 ms]
pedersen_hash           time:   [2.0007 ms 2.0076 ms 2.0145 ms]
rfc6979_generate_k      time:   [12.197 µs 12.203 µs 12.210 µs]
```

`wasmtime` results:

```log
ecdsa_get_public_key    time:   [2.8495 ms 2.8541 ms 2.8618 ms]
ecdsa_sign              time:   [2.8797 ms 2.8814 ms 2.8832 ms]
ecdsa_verify            time:   [6.7528 ms 6.7570 ms 6.7613 ms]
pedersen_hash           time:   [1.6589 ms 1.6618 ms 1.6651 ms]
rfc6979_generate_k      time:   [10.833 µs 10.839 µs 10.845 µs]
```

Node.js results:

```log
ecdsa_get_public_key    time:   [2.6020 ms 2.6138 ms 2.6304 ms]
ecdsa_sign              time:   [2.5616 ms 2.5654 ms 2.5697 ms]
ecdsa_verify            time:   [6.2385 ms 6.2399 ms 6.2412 ms]
pedersen_hash           time:   [1.7788 ms 1.7899 ms 1.8013 ms]
rfc6979_generate_k      time:   [9.6454 µs 9.6483 µs 9.6514 µs]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesone [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
