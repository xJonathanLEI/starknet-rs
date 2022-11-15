# Benchmark

`starknet-rs` uses [`criterion`](https://github.com/bheisler/criterion.rs) for performance benchmarking.

## Native

For native performance benchmark, simply run:

```console
$ cargo bench --all
```

Or just `cargo bench` inside any crate with benchmark code.

## WebAssembly

As a portable format, WebAssembly has many runtimes, including `wasmer`, `wasmtime`, Node.js, browsers, and more. Results are only provided for `wamer`, `wasmtime`, and Node.js. For other runtimes, check out [this guide](https://github.com/bheisler/criterion.rs/blob/version-0.4/book/src/user_guide/wasi.md) from `criterion`.
