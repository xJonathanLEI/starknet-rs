<p align="center">
  <img src="https://github.com/xJonathanLEI/starknet-rs/blob/master/images/starknet-rs-logo.png?raw=true" alt="Logo"/>
  <h1 align="center">starknet-rs</h1>
</p>

**Complete StarkNet library in Rust**

[![linting-badge](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml)

> _Note that `starknet-rs` is still experimental. Breaking changes will be made before the first stable release. Use at your own risk._

## Adding starknet-rs to your project

This crate hasn't been published to [crates.io](https://crates.io/) yet, so you'll need to add the following to your `Cargo.toml` to use it:

```toml
[dependencies]
starknet = { git = "https://github.com/xJonathanLEI/starknet-rs" }
```

## Features

- [x] Sequencer gateway / feeder gateway client
- [ ] Smart contract deployment
- [ ] Signer for using [IAccount](https://github.com/OpenZeppelin/cairo-contracts/blob/main/contracts/IAccount.cairo) account contracts
- [ ] Strongly-typed smart contract binding code generation from ABI

## Example

Using `SequencerGatewayProvider` to get the latest block from the `alpha-goerli` testnet:

```rust
use starknet::providers::{Provider, SequencerGatewayProvider};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block = provider.get_block(None).await;
    println!("{:#?}", latest_block);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
