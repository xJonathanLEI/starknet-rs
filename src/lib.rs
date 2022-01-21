//! # Complete StarkNet library in Rust
//!
//! > _Note that `starknet-rs` is still experimental. Breaking changes will be made before the first
//! stable release. Use at your own risk._
//!
//! > _The underlying cryptography library `starknet-crypto` does NOT provide constant-time
//! guarantees._
//!
//! `starknet-rs` is a Rust client library for StarkNet. The current version offers full API
//! coverage of the sequencer gateway and feeder gateway.
//!
//! Future versions of `starknet-rs` will support all common features required for buildling client
//! software for StarkNet:
//!
//! - full JSON-RPC API coverage as full node implementations become available
//! - contract deployment
//! - generating strongly-typed binding code for contracts from ABI
//! - invoking contracts through the standard account interface
//!
//! ## `core`
//!
//! Contains all the [necessary data structures](core::types) for interacting with StarkNet.
//!
//! ## `providers`
//!
//! The [`Provider`](providers::Provider) trait provides abstraction over StarkNet data providers.
//! Currently the only implementation is [`SequencerGatewayProvider`](providers::SequencerGatewayProvider).
//!
//! ## `contract`
//!
//! Contains all the types for deploying and interacting with StarkNet smart contracts.

#[doc = include_str!("../assets/CORE_README.md")]
pub mod core {
    pub use starknet_core::*;
}

#[doc = include_str!("../assets/PROVIDERS_README.md")]
pub mod providers {
    pub use starknet_providers::*;
}

#[doc = include_str!("../assets/CONTRACT_README.md")]
pub mod contract {
    pub use starknet_contract::*;
}
