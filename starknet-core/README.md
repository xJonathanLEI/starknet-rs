# StarkNet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
class_hash              time:   [18.931 ms 18.943 ms 18.958 ms]
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
class_hash              time:   [126.30 ms 126.47 ms 126.65 ms]
```

`wasmtime` results:

```log
class_hash              time:   [108.80 ms 109.02 ms 109.24 ms]
```

Node.js results:

```log
class_hash              time:   [113.77 ms 114.36 ms 115.07 ms]
```
