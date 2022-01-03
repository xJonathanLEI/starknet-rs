use crate::provider::Provider;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use serde_json::Error as SerdeJsonError;
use starknet_core::types::Block;
use thiserror::Error;
use url::Url;

pub struct SequencerGatewayProvider {
    client: Client,
    #[allow(unused)]
    gateway_url: Url,
    feeder_gateway_url: Url,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error("Deserialization error: {err}, Response: {text}")]
    SerdeJson { err: SerdeJsonError, text: String },
}

impl SequencerGatewayProvider {
    pub fn new(gateway_url: impl Into<Url>, feeder_gateway_url: impl Into<Url>) -> Self {
        Self {
            client: Client::new(),
            gateway_url: gateway_url.into(),
            feeder_gateway_url: feeder_gateway_url.into(),
        }
    }

    pub fn starknet_alpha_mainnet() -> Self {
        Self::new(
            Url::parse("https://alpha-mainnet.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha-mainnet.starknet.io/feeder_gateway").unwrap(),
        )
    }

    pub fn starknet_alpha_goerli() -> Self {
        Self::new(
            Url::parse("https://alpha4.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha4.starknet.io/feeder_gateway").unwrap(),
        )
    }
}

#[async_trait]
impl Provider for SequencerGatewayProvider {
    type Error = ProviderError;

    async fn get_block(&self) -> Result<Block, Self::Error> {
        let mut request_url = self.feeder_gateway_url.clone();
        request_url
            .path_segments_mut()
            .expect("Invalid base URL")
            .extend(&["get_block"]);

        let res = self.client.get(request_url).send().await?;
        let body = res.text().await?;
        serde_json::from_str(&body).map_err(|err| ProviderError::SerdeJson { err, text: body })
    }
}
