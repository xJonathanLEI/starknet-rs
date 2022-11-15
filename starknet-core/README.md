# StarkNet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
class_hash              time:   [117.32 ms 117.55 ms 117.78 ms]
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
class_hash              time:   [1.0280 s 1.0283 s 1.0287 s]
```

`wasmtime` results:

```log
class_hash              time:   [836.69 ms 839.33 ms 841.92 ms]
```

Node.js results:

```log
class_hash              time:   [900.73 ms 901.09 ms 901.47 ms]
```
