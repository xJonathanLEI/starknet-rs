# Starknet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _Apple M3 Max_ running _macOS 14.5_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
cairo0_class_hash       time:   [9.1665 ms 9.1690 ms 9.1718 ms]
sierra_class_hash       time:   [6.6931 ms 6.6944 ms 6.6958 ms]
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
cairo0_class_hash       time:   [36.515 ms 36.526 ms 36.538 ms]
sierra_class_hash       time:   [22.550 ms 22.557 ms 22.567 ms]
```
