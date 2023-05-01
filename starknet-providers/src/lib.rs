#![doc = include_str!("../README.md")]

mod provider;
pub use provider::{Provider, ProviderError};

mod sequencer_gateway;
pub use sequencer_gateway::{
    GatewayClientError as SequencerGatewayProviderError, SequencerGatewayProvider,
};

pub mod jsonrpc;

mod any;
pub use any::{AnyProvider, AnyProviderError};
