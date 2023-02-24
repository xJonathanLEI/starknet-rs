# Starknet data types

// TODO: add `starknet-core` documentation

## Benchmark

These results were generated on the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 22.04.1 LTS_.

For instructions on running the benchmarks yourself, check out [this page](../BENCHMARK.md).

### Native

```log
class_hash              time:   [17.633 ms 17.644 ms 17.656 ms]
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
class_hash              time:   [66.064 ms 66.120 ms 66.188 ms]
```
