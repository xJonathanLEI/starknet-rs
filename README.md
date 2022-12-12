<p align="center">
  <img src="https://github.com/xJonathanLEI/starknet-rs/blob/master/images/starknet-rs-logo.png?raw=true" alt="Logo"/>
  <h1 align="center">starknet-rs</h1>
</p>

**Complete StarkNet library in Rust**

![starknet-version-v0.10.3](https://img.shields.io/badge/StarkNet_Version-v0.10.3-2ea44f?logo=ethereum)
[![jsonrpc-spec-v0.2.1](https://img.shields.io/badge/JSON--RPC-v0.2.1-2ea44f?logo=ethereum)](https://github.com/starkware-libs/starknet-specs/tree/v0.2.1)
[![linting-badge](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml)
[![crates-badge](https://img.shields.io/crates/v/starknet.svg)](https://crates.io/crates/starknet)

> _Note that `starknet-rs` is still experimental. Breaking changes will be made before the first stable release. The library is also NOT audited or reviewed for security at the moment. Use at your own risk._

> _The underlying cryptography library [`starknet-crypto`](./starknet-crypto) does NOT provide constant-time guarantees._

## Adding starknet-rs to your project

To use the crate from [crates.io](https://crates.io/crates/starknet), add the following to your `Cargo.toml` file:

```toml
[dependencies]
starknet = "0.2.0"
```

Note that the [crates.io version](https://crates.io/crates/starknet) might be outdated. You may want to use the library directly from GitHub for all the latest features and fixes:

```toml
[dependencies]
starknet = { git = "https://github.com/xJonathanLEI/starknet-rs" }
```

## Features

- [x] Sequencer gateway / feeder gateway client
- [x] Full node JSON-RPC API client
- [x] Smart contract deployment
- [x] Signer for using [IAccount](https://github.com/OpenZeppelin/cairo-contracts/blob/main/src/openzeppelin/account/IAccount.cairo) account contracts
- [ ] Strongly-typed smart contract binding code generation from ABI

## Crates

This workspace contains the following crates:

- `starknet`: Re-export of other sub-crates (recommended)
- `starknet-core`: Core data structures for interacting with StarkNet
- `starknet-providers`: Abstraction and implementation of clients for interacting with StarkNet nodes and sequencers
- `starknet-contract`: Types for deploying and interacting with StarkNet smart contracts
- `starknet-crypto`: **Low-level** cryptography utilities for StarkNet
- `starknet-signers`: StarkNet signer implementations
- `starknet-accounts`: Types for handling StarkNet account abstraction
- `starknet-ff`: StarkNet field element type
- `starknet-curve`: StarkNet curve operations
- `starknet-macros`: Useful macros for using the `starknet` crates

## WebAssembly

`starknet-rs` can be used as a WebAssembly module. Check out [this example](./examples/starknet-wasm/).

## Performance

Benchmark results for native and WebAssembly targets are available for these crates:

- [starknet-core](./starknet-core/)
- [starknet-crypto](./starknet-crypto/)

For instructions on running the benchmark yourself, check [here](./BENCHMARK.md).

## Example

Examples can be found in the [examples folder](./examples):

1. [Get the latest block from `alpha-goerli` testnet](./examples/get_block.rs)

2. [Deploy contract to `alpha-goerli` testnet](./examples/deploy_contract.rs)

3. [Mint yourself 1,000 TST tokens on `alpha-goerli`](./examples/mint_tokens.rs)

   Make sure your account has some L2 Goerli ETH to pay for the transaction fee. You can use [this faucet](https://faucet.goerli.starknet.io/) to fund your account.

4. [Declare contract on `alpha-goerli` testnet](./examples/declare_contract.rs)

   Declaring contracts without going through an account (and thus not paying fees) has been deprecated, so please make sure your account has enough L2 Goerli ETH for the fees.

5. [Query the latest block number with JSON-RPC](./examples/jsonrpc.rs)

6. [Call a contract view function via sequencer gateway](./examples/sequencer_erc20_balance.rs)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
