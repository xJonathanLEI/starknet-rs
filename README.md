<p align="center">
  <img src="https://github.com/xJonathanLEI/starknet-rs/blob/master/images/starknet-rs-logo.png?raw=true" alt="Logo"/>
  <h1 align="center">starknet-rs</h1>
</p>

**Complete StarkNet library in Rust**

[![linting-badge](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml)
[![crates-badge](https://img.shields.io/crates/v/starknet.svg)](https://crates.io/crates/starknet)

> _Note that `starknet-rs` is still experimental. Breaking changes will be made before the first stable release. The library is also NOT audited or reviewed for security at the moment. Use at your own risk._

> _The underlying cryptography library [`starknet-crypto`](./starknet-crypto) does NOT provide constant-time guarantees._

## Adding starknet-rs to your project

To use the crate from [crates.io](https://crates.io/crates/starknet), add the following to your `Cargo.toml` file:

```toml
[dependencies]
starknet = "0.1.0"
```

To use from GitHub directly, use this line instead:

```toml
[dependencies]
starknet = { git = "https://github.com/xJonathanLEI/starknet-rs" }
```

## Features

- [x] Sequencer gateway / feeder gateway client
- [x] Smart contract deployment
- [ ] Signer for using [IAccount](https://github.com/OpenZeppelin/cairo-contracts/blob/main/contracts/IAccount.cairo) account contracts
- [ ] Strongly-typed smart contract binding code generation from ABI

## Crates

This workspace contains the following crates:

- `starknet`: Re-export of other sub-crates (recommended)
- `starknet-core`: Core data structures for interacting with StarkNet
- `starknet-providers`: Abstraction and implementation of clients for interacting with StarkNet nodes and sequencers
- `starknet-contract`: Types for deploying and interacting with StarkNet smart contracts
- `starknet-crypto`: **Low-level** cryptography utilities for StarkNet
- `starknet-signers`: StarkNet signer implementations

## Example

### Get the latest block from `alpha-goerli` testnet

```rust
use starknet::providers::{Provider, SequencerGatewayProvider};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block = provider.get_block(None).await;
    println!("{:#?}", latest_block);
}
```

### Deploy contract to `alpha-goerli` testnet

```rust
use std::str::FromStr;

use starknet::{
    contract::{ContractArtifact, ContractFactory},
    core::types::UnsignedFieldElement,
    providers::SequencerGatewayProvider,
};

#[tokio::main]
async fn main() {
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let contract_factory = ContractFactory::new(contract_artifact, provider).unwrap();
    contract_factory
        .deploy(
            vec![UnsignedFieldElement::from_str("123456").unwrap()],
            None,
        )
        .await
        .expect("Unable to deploy contract");
}
```

### Mint yourself 1,000 TST tokens on `alpha-goerli`

```rust
use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{types::UnsignedFieldElement, utils::get_selector_from_name},
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        UnsignedFieldElement::from_hex_str("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address =
        UnsignedFieldElement::from_hex_str("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let tst_token_address = UnsignedFieldElement::from_hex_str(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address);
    let nonce = account.get_nonce(None).await.unwrap();

    let result = account
        .execute(
            tst_token_address,
            get_selector_from_name("mint").unwrap(),
            &[
                address,
                UnsignedFieldElement::from_str("1000000000000000000000").unwrap(),
                UnsignedFieldElement::ZERO,
            ],
            nonce,
        )
        .await
        .unwrap();

    dbg!(result);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
