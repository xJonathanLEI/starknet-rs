#![doc = include_str!("../README.md")]

mod provider;
pub use provider::Provider;

mod sequencer_gateway;
pub use sequencer_gateway::{
    ProviderError as SequencerGatewayProviderError, SequencerGatewayProvider,
};

pub mod jsonrpc;
