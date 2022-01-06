use crate::provider::Provider;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use starknet_core::{
    serde::deserialize_h256_from_hex,
    types::{
        AddTransactionResult, Block, BlockId, CallContractResult, ContractAddresses, ContractCode,
        InvokeFunction, StarknetError, TransactionId, TransactionReceipt, TransactionRequest,
        TransactionStatus, TransactionWithStatus, H256, U256,
    },
};
use thiserror::Error;
use url::Url;

pub struct SequencerGatewayProvider {
    client: Client,
    gateway_url: Url,
    feeder_gateway_url: Url,
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error(transparent)]
    Serialization(SerdeJsonError),
    #[error("Deserialization error: {err}, Response: {text}")]
    Deserialization { err: SerdeJsonError, text: String },
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
enum GatewayResponse<D> {
    Data(D),
    StarknetError(StarknetError),
}

// Work around gateway sending `abi` as `{}` instead of `[]` when the code doesn't exist
#[derive(Deserialize)]
#[serde(untagged)]
enum GetCodeResponse {
    ContractCode(ContractCode),
    EmptyContractCode(EmptyContractCode),
    StarknetError(StarknetError),
}

// Work H256 deserialization
#[derive(Deserialize)]
#[serde(untagged)]
enum RawH256Response {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    Data(H256),
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
    fn extend_gateway_url(&self, segment: &str) -> Url {
        let mut url = self.gateway_url.clone();
        extend_url(&mut url, segment);
        url
    }

    fn extend_feeder_gateway_url(&self, segment: &str) -> Url {
        let mut url = self.feeder_gateway_url.clone();
        extend_url(&mut url, segment);
        url
    }

    async fn send_get_request<T>(&self, url: Url) -> Result<T, ProviderError>
    where
        T: DeserializeOwned,
    {
        let res = self.client.get(url).send().await?;
        let body = res.text().await?;
        serde_json::from_str(&body)
            .map_err(|err| ProviderError::Deserialization { err, text: body })
    }

    async fn send_post_request<Q, S>(&self, url: Url, body: &Q) -> Result<S, ProviderError>
    where
        Q: Serialize,
        S: DeserializeOwned,
    {
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(body).map_err(ProviderError::Serialization)?)
            .send()
            .await?;
        let body = res.text().await?;
        serde_json::from_str(&body)
            .map_err(|err| ProviderError::Deserialization { err, text: body })
    }
}

#[async_trait]
impl Provider for SequencerGatewayProvider {
    type Error = ProviderError;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
        token: Option<String>,
    ) -> Result<AddTransactionResult, Self::Error> {
        let mut request_url = self.extend_gateway_url("add_transaction");
        if let Some(token) = token {
            request_url.query_pairs_mut().append_pair("token", &token);
        }

        match self.send_post_request(request_url, &tx).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_contract_addresses(&self) -> Result<ContractAddresses, Self::Error> {
        let request_url = self.extend_feeder_gateway_url("get_contract_addresses");

        match self
            .send_get_request::<GatewayResponse<ContractAddresses>>(request_url)
            .await?
        {
            GatewayResponse::Data(addrs) => Ok(addrs),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn call_contract(
        &self,
        invoke_tx: InvokeFunction,
        block_hash_or_number: Option<BlockId>,
    ) -> Result<CallContractResult, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("call_contract");
        append_block_id(&mut request_url, block_hash_or_number);

        match self.send_post_request(request_url, &invoke_tx).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block(&self, block_hash_or_number: Option<BlockId>) -> Result<Block, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block");
        append_block_id(&mut request_url, block_hash_or_number);

        match self
            .send_get_request::<GatewayResponse<Block>>(request_url)
            .await?
        {
            GatewayResponse::Data(block) => Ok(block),
            GatewayResponse::StarknetError(starknet_err) => {
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

        match self
            .send_get_request::<GetCodeResponse>(request_url)
            .await?
        {
            GetCodeResponse::ContractCode(code) => Ok(code),
            GetCodeResponse::EmptyContractCode(_) => Ok(ContractCode {
                bytecode: vec![],
                abi: Some(vec![]),
            }),
            GetCodeResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_storage_at(
        &self,
        contract_address: H256,
        key: U256,
        block_hash_or_number: Option<BlockId>,
    ) -> Result<U256, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_storage_at");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address))
            .append_pair("key", &key.to_string());
        append_block_id(&mut request_url, block_hash_or_number);

        match self
            .send_get_request::<GatewayResponse<U256>>(request_url)
            .await?
        {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_status(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionStatus, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_status");
        append_transaction_id(&mut request_url, transaction_hash_or_number);

        match self
            .send_get_request::<GatewayResponse<TransactionStatus>>(request_url)
            .await?
        {
            GatewayResponse::Data(tx_status) => Ok(tx_status),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionWithStatus, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction");
        append_transaction_id(&mut request_url, transaction_hash_or_number);

        match self
            .send_get_request::<GatewayResponse<TransactionWithStatus>>(request_url)
            .await?
        {
            GatewayResponse::Data(tx) => Ok(tx),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionReceipt, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_receipt");
        append_transaction_id(&mut request_url, transaction_hash_or_number);

        match self
            .send_get_request::<GatewayResponse<TransactionReceipt>>(request_url)
            .await?
        {
            GatewayResponse::Data(receipt) => Ok(receipt),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block_hash_by_id(&self, block_number: u64) -> Result<H256, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("blockId", &block_number.to_string());

        match self
            .send_get_request::<RawH256Response>(request_url)
            .await?
        {
            RawH256Response::Data(hash) => Ok(hash),
            RawH256Response::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block_id_by_hash(&self, block_hash: H256) -> Result<u64, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_id_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("blockHash", &format!("{:#x}", block_hash));

        match self
            .send_get_request::<GatewayResponse<u64>>(request_url)
            .await?
        {
            GatewayResponse::Data(number) => Ok(number),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<H256, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("transactionId", &transaction_number.to_string());

        match self
            .send_get_request::<RawH256Response>(request_url)
            .await?
        {
            RawH256Response::Data(hash) => Ok(hash),
            RawH256Response::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_id_by_hash(&self, transaction_hash: H256) -> Result<u64, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_id_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{:#x}", transaction_hash));

        match self
            .send_get_request::<GatewayResponse<u64>>(request_url)
            .await?
        {
            GatewayResponse::Data(number) => Ok(number),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }
}

fn extend_url(url: &mut Url, segment: &str) {
    url.path_segments_mut()
        .expect("Invalid base URL")
        .extend(&[segment]);
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

fn append_transaction_id(url: &mut Url, block_hash_or_number: TransactionId) {
    match block_hash_or_number {
        TransactionId::Hash(tx_hash) => {
            url.query_pairs_mut()
                .append_pair("transactionHash", &format!("{:#x}", tx_hash));
        }
        TransactionId::Number(tx_number) => {
            url.query_pairs_mut()
                .append_pair("transactionId", &tx_number.to_string());
        }
    };
}
