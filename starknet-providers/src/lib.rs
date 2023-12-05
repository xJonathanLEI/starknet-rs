#![doc = include_str!("../README.md")]

mod provider;
pub use provider::{Provider, ProviderError};

pub mod sequencer;
pub use sequencer::{
    GatewayClientError as SequencerGatewayProviderError, SequencerGatewayProvider,
};

pub mod jsonrpc;
pub use jsonrpc::JsonRpcClient;

mod any;
pub use any::AnyProvider;
