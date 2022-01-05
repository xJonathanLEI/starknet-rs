use crate::provider::Provider;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use starknet_core::types::{Block, BlockId, ContractCode, StarknetError, H256};
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
    #[error(transparent)]
    StarknetError(StarknetError),
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

#[derive(Deserialize)]
#[serde(untagged)]
enum GetBlockResponse {
    Block(Block),
    StarknetError(StarknetError),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GetCodeResponse {
    ContractCode(ContractCode),
    EmptyContractCode(EmptyContractCode),
    StarknetError(StarknetError),
}

// Work around gateway sending `abi` as `{}` instead of `[]` when the code doesn't exist
#[allow(unused)]
#[derive(Deserialize)]
struct EmptyContractCode {
    pub bytecode: Vec<EmptyObject>,
    pub abi: EmptyObject,
}

#[derive(Deserialize)]
struct EmptyObject {}

impl SequencerGatewayProvider {
    fn extend_feeder_gateway_url(&self, segment: &str) -> Url {
        let mut url = self.feeder_gateway_url.clone();
        url.path_segments_mut()
            .expect("Invalid base URL")
            .extend(&[segment]);
        url
    }
}

#[async_trait]
impl Provider for SequencerGatewayProvider {
    type Error = ProviderError;

    async fn get_block(&self, block_hash_or_number: Option<BlockId>) -> Result<Block, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block");
        append_block_id(&mut request_url, block_hash_or_number);

        let res = self.client.get(request_url).send().await?;
        let body = res.text().await?;
        let res: GetBlockResponse = serde_json::from_str(&body)
            .map_err(|err| ProviderError::SerdeJson { err, text: body })?;

        match res {
            GetBlockResponse::Block(block) => Ok(block),
            GetBlockResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_code(
        &self,
        contract_address: H256,
        block_hash_or_number: Option<BlockId>,
    ) -> Result<ContractCode, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_code");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address));
        append_block_id(&mut request_url, block_hash_or_number);

        let res = self.client.get(request_url).send().await?;
        let body = res.text().await?;
        let res: GetCodeResponse = serde_json::from_str(&body)
            .map_err(|err| ProviderError::SerdeJson { err, text: body })?;

        match res {
            GetCodeResponse::ContractCode(code) => Ok(code),
            GetCodeResponse::EmptyContractCode(_) => Ok(ContractCode {
                bytecode: vec![],
                abi: vec![],
            }),
            GetCodeResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }
}

fn append_block_id(url: &mut Url, block_hash_or_number: Option<BlockId>) {
    match block_hash_or_number {
        Some(BlockId::Hash(block_hash)) => {
            url.query_pairs_mut()
                .append_pair("blockHash", &format!("{:#x}", block_hash));
        }
        Some(BlockId::Number(block_number)) => {
            url.query_pairs_mut()
                .append_pair("blockNumber", &block_number.to_string());
        }
        _ => (),
    };
}
