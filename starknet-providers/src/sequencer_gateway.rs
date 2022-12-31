use crate::provider::Provider;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::UfeHex,
    types::{
        AccountTransaction, AddTransactionResult, Block, BlockId, BlockTraces, CallContractResult,
        CallFunction, CallL1Handler, ContractAddresses, ContractArtifact, ContractCode,
        FeeEstimate, FieldElement, StarknetError, StateUpdate, TransactionInfo, TransactionReceipt,
        TransactionRequest, TransactionSimulationInfo, TransactionStatusInfo, TransactionTrace,
    },
};
use thiserror::Error;
use url::Url;

#[derive(Clone)]
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

    pub fn starknet_alpha_goerli_2() -> Self {
        Self::new(
            Url::parse("https://alpha4-2.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha4-2.starknet.io/feeder_gateway").unwrap(),
        )
    }

    pub fn starknet_nile_localhost() -> Self {
        Self::new(
            Url::parse("http://127.0.0.1:5050/gateway").unwrap(),
            Url::parse("http://127.0.0.1:5050/feeder_gateway").unwrap(),
        )
    }
}

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

// Work FieldElement deserialization
#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
enum RawFieldElementResponse {
    Data(#[serde_as(as = "UfeHex")] FieldElement),
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

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for SequencerGatewayProvider {
    type Error = ProviderError;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, Self::Error> {
        let request_url = self.extend_gateway_url("add_transaction");

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
        call_function: CallFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("call_contract");
        append_block_id(&mut request_url, block_identifier);

        match self.send_post_request(request_url, &call_function).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn estimate_fee(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
    ) -> Result<FeeEstimate, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_fee");
        append_block_id(&mut request_url, block_identifier);

        match self.send_post_request(request_url, &tx).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn estimate_fee_bulk(
        &self,
        txs: &[AccountTransaction],
        block_identifier: BlockId,
    ) -> Result<Vec<FeeEstimate>, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_fee_bulk");
        append_block_id(&mut request_url, block_identifier);

        match self.send_post_request(request_url, &txs).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn estimate_message_fee(
        &self,
        call_l1_handler: CallL1Handler,
        block_identifier: BlockId,
    ) -> Result<FeeEstimate, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_message_fee");
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_post_request(request_url, &call_l1_handler)
            .await?
        {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn simulate_transaction(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
    ) -> Result<TransactionSimulationInfo, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("simulate_transaction");
        append_block_id(&mut request_url, block_identifier);

        match self.send_post_request(request_url, &tx).await? {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block(&self, block_identifier: BlockId) -> Result<Block, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block");
        append_block_id(&mut request_url, block_identifier);

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

    async fn get_block_traces(
        &self,
        block_identifier: BlockId,
    ) -> Result<BlockTraces, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_traces");
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<GatewayResponse<BlockTraces>>(request_url)
            .await?
        {
            GatewayResponse::Data(update) => Ok(update),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_state_update(
        &self,
        block_identifier: BlockId,
    ) -> Result<StateUpdate, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_state_update");
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<GatewayResponse<StateUpdate>>(request_url)
            .await?
        {
            GatewayResponse::Data(update) => Ok(update),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_code(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<ContractCode, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_code");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address));
        append_block_id(&mut request_url, block_identifier);

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

    async fn get_full_contract(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<ContractArtifact, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_full_contract");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address));
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<GatewayResponse<ContractArtifact>>(request_url)
            .await?
        {
            GatewayResponse::Data(artifact) => Ok(artifact),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_class_hash_at(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_class_hash_at");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address));
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<RawFieldElementResponse>(request_url)
            .await?
        {
            RawFieldElementResponse::Data(hash) => Ok(hash),
            RawFieldElementResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_class_by_hash(
        &self,
        class_hash: FieldElement,
    ) -> Result<ContractArtifact, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_class_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("classHash", &format!("{:#x}", class_hash));

        match self
            .send_get_request::<GatewayResponse<ContractArtifact>>(request_url)
            .await?
        {
            GatewayResponse::Data(artifact) => Ok(artifact),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_storage_at");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address))
            .append_pair("key", &key.to_string());
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<RawFieldElementResponse>(request_url)
            .await?
        {
            RawFieldElementResponse::Data(data) => Ok(data),
            RawFieldElementResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_nonce");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{:#x}", contract_address));
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<RawFieldElementResponse>(request_url)
            .await?
        {
            RawFieldElementResponse::Data(data) => Ok(data),
            RawFieldElementResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_status(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionStatusInfo, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_status");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{:#x}", transaction_hash));

        match self
            .send_get_request::<GatewayResponse<TransactionStatusInfo>>(request_url)
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
        transaction_hash: FieldElement,
    ) -> Result<TransactionInfo, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{:#x}", transaction_hash));

        match self
            .send_get_request::<GatewayResponse<TransactionInfo>>(request_url)
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
        transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_receipt");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{:#x}", transaction_hash));

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

    async fn get_transaction_trace(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionTrace, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_trace");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{:#x}", transaction_hash));

        match self.send_get_request(request_url).await? {
            GatewayResponse::Data(trace) => Ok(trace),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block_hash_by_id(&self, block_number: u64) -> Result<FieldElement, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("blockId", &block_number.to_string());

        match self
            .send_get_request::<RawFieldElementResponse>(request_url)
            .await?
        {
            RawFieldElementResponse::Data(hash) => Ok(hash),
            RawFieldElementResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_block_id_by_hash(&self, block_hash: FieldElement) -> Result<u64, Self::Error> {
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
    ) -> Result<FieldElement, Self::Error> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("transactionId", &transaction_number.to_string());

        match self
            .send_get_request::<RawFieldElementResponse>(request_url)
            .await?
        {
            RawFieldElementResponse::Data(hash) => Ok(hash),
            RawFieldElementResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<u64, Self::Error> {
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

    async fn get_last_batch_id(&self) -> Result<u64, Self::Error> {
        let request_url = self.extend_feeder_gateway_url("get_last_batch_id");

        match self
            .send_get_request::<GatewayResponse<u64>>(request_url)
            .await?
        {
            GatewayResponse::Data(batch_id) => Ok(batch_id),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }

    async fn get_l1_blockchain_id(&self) -> Result<u64, Self::Error> {
        let request_url = self.extend_feeder_gateway_url("get_l1_blockchain_id");

        match self
            .send_get_request::<GatewayResponse<u64>>(request_url)
            .await?
        {
            GatewayResponse::Data(network_id) => Ok(network_id),
            GatewayResponse::StarknetError(starknet_err) => {
                Err(ProviderError::StarknetError(starknet_err))
            }
        }
    }
}

// We need to manually implement this because `arbitrary_precision` doesn't work with `untagged`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de, T> Deserialize<'de> for GatewayResponse<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        if let Ok(value) = T::deserialize(&temp_value) {
            return Ok(GatewayResponse::Data(value));
        }
        if let Ok(value) = StarknetError::deserialize(&temp_value) {
            return Ok(GatewayResponse::StarknetError(value));
        }
        Err(serde::de::Error::custom(
            "data did not match any variant of enum GatewayResponse",
        ))
    }
}

fn extend_url(url: &mut Url, segment: &str) {
    url.path_segments_mut()
        .expect("Invalid base URL")
        .extend(&[segment]);
}

fn append_block_id(url: &mut Url, block_identifier: BlockId) {
    match block_identifier {
        BlockId::Hash(block_hash) => {
            url.query_pairs_mut()
                .append_pair("blockHash", &format!("{:#x}", block_hash));
        }
        BlockId::Number(block_number) => {
            url.query_pairs_mut()
                .append_pair("blockNumber", &block_number.to_string());
        }
        BlockId::Pending => {
            url.query_pairs_mut().append_pair("blockNumber", "pending");
        }
        BlockId::Latest => (), // latest block is implicit
    };
}

#[cfg(test)]
mod tests {
    use starknet_core::types::StarknetErrorCode;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_estimate_fee_deser() {
        serde_json::from_str::<GatewayResponse<FeeEstimate>>(include_str!(
            "../test-data/estimate_fee/1_success.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_estimate_fee_bulk_deser() {
        serde_json::from_str::<GatewayResponse<Vec<FeeEstimate>>>(include_str!(
            "../test-data/estimate_fee_bulk/1_success.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_storage_at_deser() {
        serde_json::from_str::<RawFieldElementResponse>(include_str!(
            "../test-data/get_storage_at/1_empty.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_full_contract_deser() {
        serde_json::from_str::<GatewayResponse<ContractArtifact>>(include_str!(
            "../test-data/get_full_contract/1_code.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_success() {
        match serde_json::from_str::<GatewayResponse<ContractArtifact>>(include_str!(
            "../test-data/get_class_by_hash/1_success.txt"
        ))
        .unwrap()
        {
            GatewayResponse::Data(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_not_declared() {
        match serde_json::from_str::<GatewayResponse<ContractArtifact>>(include_str!(
            "../test-data/get_class_by_hash/2_not_declared.txt"
        ))
        .unwrap()
        {
            GatewayResponse::StarknetError(err) => {
                assert_eq!(err.code, StarknetErrorCode::UndeclaredClass);
            }
            _ => panic!("Unexpected result"),
        }
    }
}
