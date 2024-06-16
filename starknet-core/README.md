# Starknet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _Apple M3 Max_ running _macOS 14.5_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
cairo0_class_hash       time:   [10.808 ms 10.813 ms 10.819 ms]
sierra_class_hash       time:   [6.2774 ms 6.2802 ms 6.2832 ms]
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
cairo0_class_hash       time:   [39.192 ms 39.207 ms 39.224 ms]
sierra_class_hash       time:   [20.514 ms 20.521 ms 20.529 ms]
```
