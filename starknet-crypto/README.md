# Low-level cryptography utilities for StarkNet

`starknet-crypto` contains utilities for performing **low-level** cryptographic operations in StarkNet.

> _You're advised to use high-level crypto utilities implemented by the `starknet-core` crate (or use it through the `starknet::core` re-export) if you're not familiar with cryptographic primitives. Using these low-level functions incorrectly could result in leaking your private key, for example._

> _This library does not provide constant-time guarantees._

## **WARNING**

While it has been tested against data randomly generated from [`cairo-lang`](https://github.com/starkware-libs/cairo-lang), this crate is _NOT_ audited or reviewed for security. **Use at your own risk**.

If you're a cryptographer, you're welcome to contribute by reviewing the implementation :)

## Benchmark

On the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_:

```log
ecdsa_get_public_key    time:   [1.4792 ms 1.4795 ms 1.4799 ms]
ecdsa_sign              time:   [1.4861 ms 1.4865 ms 1.4870 ms]
ecdsa_verify            time:   [3.1352 ms 3.1405 ms 3.1470 ms]
pedersen_hash           time:   [237.49 us 237.88 us 238.71 us]
rfc6979_generate_k      time:   [2.2266 us 2.2300 us 2.2348 us]
```

## Credits

Most of the code in this crate for the Pedersen hash implementation was inspired and modified from the awesone [`pathfinder` from Equilibrium](https://github.com/eqlabs/pathfinder/blob/b091cb889e624897dbb0cbec3c1df9a9e411eb1e/crates/pedersen/src/lib.rs).

Based on this solid foundation, ECDSA functionalities were inspired and ported from the [`crypto-cpp` implementation from StarkWare](https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ecdsa.cc).
