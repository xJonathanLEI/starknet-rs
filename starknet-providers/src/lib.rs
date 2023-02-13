#![doc = include_str!("../README.md")]
#![no_std]

extern crate no_std_compat as std;

mod provider;
pub use provider::{Provider, ProviderError};

mod sequencer_gateway;
pub use sequencer_gateway::{
    GatewayClientError as SequencerGatewayProviderError, SequencerGatewayProvider,
};

pub mod jsonrpc;
