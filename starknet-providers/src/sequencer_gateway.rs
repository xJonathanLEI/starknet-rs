use crate::provider::{Provider, ProviderError};

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::UfeHex,
    types::{
        contract::{legacy::LegacyContractCode, CompiledClass, DeployedClass},
        AccountTransaction, AddTransactionResult, Block, BlockId, BlockTraces, CallContractResult,
        CallFunction, CallL1Handler, ContractAddresses, FeeEstimate, FieldElement, StarknetError,
        StateUpdate, TransactionInfo, TransactionReceipt, TransactionRequest,
        TransactionSimulationInfo, TransactionStatusInfo, TransactionTrace,
    },
};
use url::Url;

#[derive(Debug, Clone)]
pub struct SequencerGatewayProvider {
    client: Client,
    gateway_url: Url,
    feeder_gateway_url: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum GatewayClientError {
    /// Network related error
    #[error(transparent)]
    Network(#[from] ReqwestError),
    /// JSON serialization/deserialization error
    #[error(transparent)]
    Serde(SerdeJsonError),
    /// Sequencer error responses not parsable into [StarknetError]
    #[error(transparent)]
    SequencerError(SequencerError),
}

#[derive(Debug, thiserror::Error, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
#[error("{message} ({code:?})")]
pub struct SequencerError {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum ErrorCode {
    #[serde(rename = "StarknetErrorCode.BLOCK_NOT_FOUND")]
    BlockNotFound,
    #[serde(rename = "StarknetErrorCode.ENTRY_POINT_NOT_FOUND_IN_CONTRACT")]
    EntryPointNotFoundInContract,
    #[serde(rename = "StarknetErrorCode.INVALID_PROGRAM")]
    InvalidProgram,
    #[serde(rename = "StarknetErrorCode.TRANSACTION_FAILED")]
    TransactionFailed,
    #[serde(rename = "StarknetErrorCode.TRANSACTION_NOT_FOUND")]
    TransactionNotFound,
    #[serde(rename = "StarknetErrorCode.UNINITIALIZED_CONTRACT")]
    UninitializedContract,
    #[serde(rename = "StarkErrorCode.MALFORMED_REQUEST")]
    MalformedRequest,
    #[serde(rename = "StarknetErrorCode.UNDECLARED_CLASS")]
    UndeclaredClass,
    #[serde(rename = "StarknetErrorCode.INVALID_TRANSACTION_NONCE")]
    InvalidTransactionNonce,
    #[serde(rename = "StarknetErrorCode.CLASS_ALREADY_DECLARED")]
    ClassAlreadyDeclared,
}

impl SequencerGatewayProvider {
    pub fn new(gateway_url: impl Into<Url>, feeder_gateway_url: impl Into<Url>) -> Self {
        Self::new_with_client(gateway_url, feeder_gateway_url, Client::new())
    }

    pub fn new_with_client(
        gateway_url: impl Into<Url>,
        feeder_gateway_url: impl Into<Url>,
        client: Client,
    ) -> Self {
        Self {
            client,
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
    SequencerError(SequencerError),
}

// Work around gateway sending `abi` as `{}` instead of `[]` when the code doesn't exist
#[derive(Deserialize)]
#[serde(untagged)]
enum GetCodeResponse {
    ContractCode(LegacyContractCode),
    EmptyContractCode(EmptyContractCode),
    SequencerError(SequencerError),
}

// Work FieldElement deserialization
#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
enum RawFieldElementResponse {
    Data(#[serde_as(as = "UfeHex")] FieldElement),
    SequencerError(SequencerError),
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

    async fn send_get_request<T>(&self, url: Url) -> Result<T, ProviderError<GatewayClientError>>
    where
        T: DeserializeOwned,
    {
        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| ProviderError::Other(GatewayClientError::Network(err)))?;
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            Err(ProviderError::RateLimited)
        } else {
            let body = res
                .text()
                .await
                .map_err(|err| ProviderError::Other(GatewayClientError::Network(err)))?;
            serde_json::from_str(&body)
                .map_err(|err| ProviderError::Other(GatewayClientError::Serde(err)))
        }
    }

    async fn send_post_request<Q, S>(
        &self,
        url: Url,
        body: &Q,
    ) -> Result<S, ProviderError<GatewayClientError>>
    where
        Q: Serialize,
        S: DeserializeOwned,
    {
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(body)
                    .map_err(|err| ProviderError::Other(GatewayClientError::Serde(err)))?,
            )
            .send()
            .await
            .map_err(|err| ProviderError::Other(GatewayClientError::Network(err)))?;
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            Err(ProviderError::RateLimited)
        } else {
            let body = res
                .text()
                .await
                .map_err(|err| ProviderError::Other(GatewayClientError::Network(err)))?;
            serde_json::from_str(&body)
                .map_err(|err| ProviderError::Other(GatewayClientError::Serde(err)))
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Provider for SequencerGatewayProvider {
    type Error = GatewayClientError;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, ProviderError<Self::Error>> {
        let request_url = self.extend_gateway_url("add_transaction");

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &tx)
            .await?
            .into()
    }

    async fn get_contract_addresses(
        &self,
    ) -> Result<ContractAddresses, ProviderError<Self::Error>> {
        let request_url = self.extend_feeder_gateway_url("get_contract_addresses");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn call_contract(
        &self,
        call_function: CallFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("call_contract");
        append_block_id(&mut request_url, block_identifier);

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &call_function)
            .await?
            .into()
    }

    async fn estimate_fee(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_fee");
        request_url
            .query_pairs_mut()
            .append_pair("skipValidate", if skip_validate { "true" } else { "false" });
        append_block_id(&mut request_url, block_identifier);

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &tx)
            .await?
            .into()
    }

    async fn estimate_fee_bulk(
        &self,
        txs: &[AccountTransaction],
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_fee_bulk");
        request_url
            .query_pairs_mut()
            .append_pair("skipValidate", if skip_validate { "true" } else { "false" });
        append_block_id(&mut request_url, block_identifier);

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &txs)
            .await?
            .into()
    }

    async fn estimate_message_fee(
        &self,
        call_l1_handler: CallL1Handler,
        block_identifier: BlockId,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("estimate_message_fee");
        append_block_id(&mut request_url, block_identifier);

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &call_l1_handler)
            .await?
            .into()
    }

    async fn simulate_transaction(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<TransactionSimulationInfo, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("simulate_transaction");
        request_url
            .query_pairs_mut()
            .append_pair("skipValidate", if skip_validate { "true" } else { "false" });
        append_block_id(&mut request_url, block_identifier);

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &tx)
            .await?
            .into()
    }

    async fn get_block(
        &self,
        block_identifier: BlockId,
    ) -> Result<Block, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_block");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_block_traces(
        &self,
        block_identifier: BlockId,
    ) -> Result<BlockTraces, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_traces");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_state_update(
        &self,
        block_identifier: BlockId,
    ) -> Result<StateUpdate, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_state_update");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_code(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<LegacyContractCode, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_code");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{contract_address:#x}"));
        append_block_id(&mut request_url, block_identifier);

        match self
            .send_get_request::<GetCodeResponse>(request_url)
            .await?
        {
            GetCodeResponse::ContractCode(code) => Ok(code),
            GetCodeResponse::EmptyContractCode(_) => Ok(LegacyContractCode {
                bytecode: vec![],
                abi: Some(vec![]),
            }),
            GetCodeResponse::SequencerError(err) => match err.code.try_into() {
                Ok(sn_err) => Err(ProviderError::StarknetError(sn_err)),
                Err(_) => Err(ProviderError::Other(GatewayClientError::SequencerError(
                    err,
                ))),
            },
        }
    }

    async fn get_full_contract(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_full_contract");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{contract_address:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_compiled_class_by_class_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<CompiledClass, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_compiled_class_by_class_hash");
        request_url
            .query_pairs_mut()
            .append_pair("classHash", &format!("{class_hash:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_class_hash_at(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_class_hash_at");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{contract_address:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    async fn get_class_by_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_class_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("classHash", &format!("{class_hash:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_storage_at");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{contract_address:#x}"))
            .append_pair("key", &key.to_string());
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_nonce");
        request_url
            .query_pairs_mut()
            .append_pair("contractAddress", &format!("{contract_address:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    async fn get_transaction_status(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionStatusInfo, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_status");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_transaction(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionInfo, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_receipt");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_transaction_trace(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionTrace, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_trace");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_block_hash_by_id(
        &self,
        block_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("blockId", &block_number.to_string());

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    async fn get_block_id_by_hash(
        &self,
        block_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_id_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("blockHash", &format!("{block_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("transactionId", &transaction_number.to_string());

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_id_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_last_batch_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        let request_url = self.extend_feeder_gateway_url("get_last_batch_id");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    async fn get_l1_blockchain_id(&self) -> Result<u64, ProviderError<Self::Error>> {
        let request_url = self.extend_feeder_gateway_url("get_l1_blockchain_id");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }
}

impl TryFrom<ErrorCode> for StarknetError {
    type Error = ();

    fn try_from(value: ErrorCode) -> Result<Self, Self::Error> {
        match value {
            ErrorCode::BlockNotFound => Ok(Self::BlockNotFound),
            ErrorCode::EntryPointNotFoundInContract => Err(()),
            ErrorCode::InvalidProgram => Ok(Self::InvalidContractClass),
            ErrorCode::TransactionFailed => Err(()),
            ErrorCode::TransactionNotFound => Ok(Self::ContractNotFound),
            ErrorCode::UninitializedContract => Ok(Self::ContractNotFound),
            ErrorCode::MalformedRequest => Err(()),
            ErrorCode::UndeclaredClass => Ok(Self::ClassHashNotFound),
            ErrorCode::InvalidTransactionNonce => Err(()),
            ErrorCode::ClassAlreadyDeclared => Err(()),
        }
    }
}

impl<D> From<GatewayResponse<D>> for Result<D, ProviderError<GatewayClientError>> {
    fn from(value: GatewayResponse<D>) -> Self {
        match value {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::SequencerError(err) => match err.code.try_into() {
                Ok(sn_err) => Err(ProviderError::StarknetError(sn_err)),
                Err(_) => Err(ProviderError::Other(GatewayClientError::SequencerError(
                    err,
                ))),
            },
        }
    }
}

impl From<RawFieldElementResponse> for Result<FieldElement, ProviderError<GatewayClientError>> {
    fn from(value: RawFieldElementResponse) -> Self {
        match value {
            RawFieldElementResponse::Data(data) => Ok(data),
            RawFieldElementResponse::SequencerError(err) => match err.code.try_into() {
                Ok(sn_err) => Err(ProviderError::StarknetError(sn_err)),
                Err(_) => Err(ProviderError::Other(GatewayClientError::SequencerError(
                    err,
                ))),
            },
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
        if let Ok(value) = SequencerError::deserialize(&temp_value) {
            return Ok(GatewayResponse::SequencerError(value));
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
                .append_pair("blockHash", &format!("{block_hash:#x}"));
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
        for raw in [
            include_str!("../test-data/get_full_contract/1_cairo_0.txt"),
            include_str!("../test-data/get_full_contract/2_cairo_1.txt"),
        ]
        .into_iter()
        {
            serde_json::from_str::<GatewayResponse<DeployedClass>>(raw).unwrap();
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_success() {
        for raw in [
            include_str!("../test-data/get_class_by_hash/1_cairo_0.txt"),
            include_str!("../test-data/get_class_by_hash/3_cairo_1.txt"),
        ]
        .into_iter()
        {
            serde_json::from_str::<GatewayResponse<DeployedClass>>(raw).unwrap();
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_not_declared() {
        match serde_json::from_str::<GatewayResponse<DeployedClass>>(include_str!(
            "../test-data/get_class_by_hash/2_not_declared.txt"
        ))
        .unwrap()
        {
            GatewayResponse::SequencerError(err) => {
                assert_eq!(err.code, ErrorCode::UndeclaredClass);
            }
            _ => panic!("Unexpected result"),
        }
    }
}
