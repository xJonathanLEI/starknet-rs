# Starknet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 22.04.1 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
class_hash              time:   [21.680 ms 21.684 ms 21.688 ms]
```

### WebAssembly

_(With its excellent wasm performance, results are only provided for Node.js here. Check out the [benchmark page](../BENCHMARK.md) for running the benchmark on other runtimes)._

Runtime version:

```console
$ node --version
v18.16.0
```

Node.js results:

```log
class_hash              time:   [124.48 ms 124.58 ms 124.69 ms]
```
