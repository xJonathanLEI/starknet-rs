//! Clients for interacting with Starknet nodes and sequencers.
//!
//! This crate provides the [`Provider`] trait for abstraction over means of accessing the Starknet
//! network. The most commonly used implementation is [`JsonRpcClient`] with
//! [`HttpTransport`](jsonrpc::HttpTransport).

#![deny(missing_docs)]

mod provider;
pub use provider::{Provider, ProviderError, ProviderRequestData, ProviderResponseData};

// Sequencer-related functionalities are mostly deprecated so we skip the docs.
/// Module containing types related to the (now deprecated) sequencer gateway client.
#[allow(missing_docs)]
pub mod sequencer;
pub use sequencer::{
    GatewayClientError as SequencerGatewayProviderError, SequencerGatewayProvider,
};

/// Module containing types related to JSON-RPC clients and servers.
pub mod jsonrpc;
pub use jsonrpc::JsonRpcClient;

mod any;
pub use any::AnyProvider;

// Re-export
pub use url::Url;
