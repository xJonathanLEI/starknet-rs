use crate::provider::ProviderError;

use log::trace;
use reqwest::{Client, Error as ReqwestError, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use serde_with::serde_as;
use starknet_core::{
    chain_id,
    serde::unsigned_field_element::UfeHex,
    types::{contract::CompiledClass, Felt, StarknetError},
};
use url::Url;

// Sequencer specific model types. Not exposed by design to discourage sequencer usage.
#[allow(unused)]
pub mod models;
use models::{conversions::ConversionError, *};

// Allows sequencer gateway to be used as if it's jsonrpc.
mod provider;

#[derive(Debug, Clone)]
pub struct SequencerGatewayProvider {
    client: Client,
    gateway_url: Url,
    feeder_gateway_url: Url,
    chain_id: Felt,
    headers: Vec<(String, String)>,
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
    /// Method is not supported (only when using as [Provider])
    #[error("method not supported")]
    MethodNotSupported,
    /// Model conversion error (only when using as [Provider])
    #[error("unable to convert gateway models to jsonrpc types")]
    ModelConversionError,
    /// Simulating multiple transactions is not supported (only when using as [Provider])
    #[error("simulating multiple transactions not supported")]
    BulkSimulationNotSupported,
    /// At least one of the simulation flags is not supported (only when using as [Provider])
    #[error("unsupported simulation flag")]
    UnsupportedSimulationFlag,
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
    #[serde(rename = "StarknetErrorCode.VALIDATE_FAILURE")]
    ValidateFailure,
    #[serde(rename = "StarknetErrorCode.CLASS_ALREADY_DECLARED")]
    ClassAlreadyDeclared,
    #[serde(rename = "StarknetErrorCode.COMPILATION_FAILED")]
    CompilationFailed,
    #[serde(rename = "StarknetErrorCode.INVALID_COMPILED_CLASS_HASH")]
    InvalidCompiledClassHash,
    #[serde(rename = "StarknetErrorCode.DUPLICATED_TRANSACTION")]
    DuplicatedTransaction,
    #[serde(rename = "StarknetErrorCode.INVALID_CONTRACT_CLASS")]
    InvalidContractClass,
    #[serde(rename = "StarknetErrorCode.DEPRECATED_ENDPOINT")]
    DeprecatedEndpoint,
}

impl SequencerGatewayProvider {
    pub fn new(
        gateway_url: impl Into<Url>,
        feeder_gateway_url: impl Into<Url>,
        chain_id: Felt,
    ) -> Self {
        Self::new_with_client(gateway_url, feeder_gateway_url, chain_id, Client::new())
    }

    pub fn new_with_client(
        gateway_url: impl Into<Url>,
        feeder_gateway_url: impl Into<Url>,
        chain_id: Felt,
        client: Client,
    ) -> Self {
        Self {
            client,
            gateway_url: gateway_url.into(),
            feeder_gateway_url: feeder_gateway_url.into(),
            chain_id,
            headers: vec![],
        }
    }

    pub fn starknet_alpha_mainnet() -> Self {
        Self::new(
            Url::parse("https://alpha-mainnet.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha-mainnet.starknet.io/feeder_gateway").unwrap(),
            chain_id::MAINNET,
        )
    }

    pub fn starknet_alpha_sepolia() -> Self {
        Self::new(
            Url::parse("https://alpha-sepolia.starknet.io/gateway").unwrap(),
            Url::parse("https://alpha-sepolia.starknet.io/feeder_gateway").unwrap(),
            chain_id::SEPOLIA,
        )
    }

    /// Consumes the current [SequencerGatewayProvider] instance and returns a new one with the
    /// header appended. Same as calling [add_header].
    pub fn with_header(self, name: String, value: String) -> Self {
        let mut headers = self.headers;
        headers.push((name, value));

        Self {
            client: self.client,
            gateway_url: self.gateway_url,
            feeder_gateway_url: self.feeder_gateway_url,
            chain_id: self.chain_id,
            headers,
        }
    }

    /// Adds a custom HTTP header to be sent for requests to the sequencer.
    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value))
    }
}

enum GatewayResponse<D> {
    Data(D),
    SequencerError(SequencerError),
}

// Work Felt deserialization
#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
enum RawFieldElementResponse {
    Data(#[serde_as(as = "UfeHex")] Felt),
    SequencerError(SequencerError),
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
        trace!("Sending GET request to sequencer API ({})", url);

        let mut request = self.client.get(url);
        for (name, value) in self.headers.iter() {
            request = request.header(name, value);
        }

        let res = request.send().await.map_err(GatewayClientError::Network)?;
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            Err(ProviderError::RateLimited)
        } else {
            let body = res.text().await.map_err(GatewayClientError::Network)?;

            trace!("Response from sequencer API: {}", body);

            Ok(serde_json::from_str(&body).map_err(GatewayClientError::Serde)?)
        }
    }

    async fn send_post_request<Q, S>(&self, url: Url, body: &Q) -> Result<S, ProviderError>
    where
        Q: Serialize,
        S: DeserializeOwned,
    {
        let request_body = serde_json::to_string(body).map_err(GatewayClientError::Serde)?;

        trace!(
            "Sending POST request to sequencer API ({}): {}",
            url,
            request_body
        );

        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(request_body);
        for (name, value) in self.headers.iter() {
            request = request.header(name, value);
        }

        let res = request.send().await.map_err(GatewayClientError::Network)?;
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            Err(ProviderError::RateLimited)
        } else {
            let body = res.text().await.map_err(GatewayClientError::Network)?;

            trace!("Response from sequencer API: {}", body);

            Ok(serde_json::from_str(&body).map_err(GatewayClientError::Serde)?)
        }
    }
}

impl SequencerGatewayProvider {
    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, ProviderError> {
        let request_url = self.extend_gateway_url("add_transaction");

        self.send_post_request::<_, GatewayResponse<_>>(request_url, &tx)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_contract_addresses(&self) -> Result<ContractAddresses, ProviderError> {
        let request_url = self.extend_feeder_gateway_url("get_contract_addresses");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_block(&self, block_identifier: BlockId) -> Result<Block, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_block");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_block_traces(
        &self,
        block_identifier: BlockId,
    ) -> Result<BlockTraces, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_traces");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_state_update(
        &self,
        block_identifier: BlockId,
    ) -> Result<StateUpdate, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_state_update");
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_compiled_class_by_class_hash(
        &self,
        class_hash: Felt,
        block_identifier: BlockId,
    ) -> Result<CompiledClass, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_compiled_class_by_class_hash");
        request_url
            .query_pairs_mut()
            .append_pair("classHash", &format!("{class_hash:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_class_by_hash(
        &self,
        class_hash: Felt,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_class_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("classHash", &format!("{class_hash:#x}"));
        append_block_id(&mut request_url, block_identifier);

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_transaction_status(
        &self,
        transaction_hash: Felt,
    ) -> Result<TransactionStatusInfo, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_status");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_transaction(
        &self,
        transaction_hash: Felt,
    ) -> Result<TransactionInfo, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_transaction_trace(
        &self,
        transaction_hash: Felt,
    ) -> Result<TransactionTrace, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_transaction_trace");
        request_url
            .query_pairs_mut()
            .append_pair("transactionHash", &format!("{transaction_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_block_hash_by_id(&self, block_number: u64) -> Result<Felt, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_hash_by_id");
        request_url
            .query_pairs_mut()
            .append_pair("blockId", &block_number.to_string());

        self.send_get_request::<RawFieldElementResponse>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_block_id_by_hash(&self, block_hash: Felt) -> Result<u64, ProviderError> {
        let mut request_url = self.extend_feeder_gateway_url("get_block_id_by_hash");
        request_url
            .query_pairs_mut()
            .append_pair("blockHash", &format!("{block_hash:#x}"));

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_last_batch_id(&self) -> Result<u64, ProviderError> {
        let request_url = self.extend_feeder_gateway_url("get_last_batch_id");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }

    #[deprecated(
        note = "Sequencer-specific functions are deprecated. Use it via the Provider trait instead."
    )]
    pub async fn get_l1_blockchain_id(&self) -> Result<u64, ProviderError> {
        let request_url = self.extend_feeder_gateway_url("get_l1_blockchain_id");

        self.send_get_request::<GatewayResponse<_>>(request_url)
            .await?
            .into()
    }
}

impl From<SequencerError> for ProviderError {
    fn from(value: SequencerError) -> Self {
        let matching_code = match value.code {
            ErrorCode::BlockNotFound => Some(StarknetError::BlockNotFound),
            ErrorCode::EntryPointNotFoundInContract => None,
            ErrorCode::InvalidProgram => None,
            ErrorCode::TransactionFailed => {
                Some(StarknetError::ValidationFailure(value.message.clone()))
            }
            ErrorCode::TransactionNotFound => Some(StarknetError::ContractNotFound),
            ErrorCode::UninitializedContract => Some(StarknetError::ContractNotFound),
            ErrorCode::MalformedRequest => None,
            ErrorCode::UndeclaredClass => Some(StarknetError::ClassHashNotFound),
            ErrorCode::InvalidTransactionNonce => Some(StarknetError::InvalidTransactionNonce),
            ErrorCode::ValidateFailure => {
                Some(StarknetError::ValidationFailure(value.message.clone()))
            }
            ErrorCode::ClassAlreadyDeclared => Some(StarknetError::ClassAlreadyDeclared),
            ErrorCode::CompilationFailed => Some(StarknetError::CompilationFailed),
            ErrorCode::InvalidCompiledClassHash => Some(StarknetError::CompiledClassHashMismatch),
            ErrorCode::DuplicatedTransaction => Some(StarknetError::DuplicateTx),
            ErrorCode::InvalidContractClass => None,
            ErrorCode::DeprecatedEndpoint => None,
        };

        match matching_code {
            Some(code) => ProviderError::StarknetError(code),
            None => GatewayClientError::SequencerError(value).into(),
        }
    }
}

impl From<ConversionError> for ProviderError {
    fn from(_value: ConversionError) -> Self {
        Self::Other(Box::new(GatewayClientError::ModelConversionError))
    }
}

impl<D> From<GatewayResponse<D>> for Result<D, ProviderError> {
    fn from(value: GatewayResponse<D>) -> Self {
        match value {
            GatewayResponse::Data(data) => Ok(data),
            GatewayResponse::SequencerError(err) => Err(err.into()),
        }
    }
}

impl From<RawFieldElementResponse> for Result<Felt, ProviderError> {
    fn from(value: RawFieldElementResponse) -> Self {
        match value {
            RawFieldElementResponse::Data(data) => Ok(data),
            RawFieldElementResponse::SequencerError(err) => Err(err.into()),
        }
    }
}

// We need to manually implement this because `raw_value` doesn't work with `untagged`:
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
    fn test_get_class_by_hash_deser_success() {
        for raw in [
            include_str!("../../test-data/raw_gateway_responses/get_class_by_hash/1_cairo_0.txt"),
            include_str!("../../test-data/raw_gateway_responses/get_class_by_hash/3_cairo_1.txt"),
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
            "../../test-data/raw_gateway_responses/get_class_by_hash/2_not_declared.txt"
        ))
        .unwrap()
        {
            GatewayResponse::SequencerError(err) => {
                assert_eq!(err.code, ErrorCode::UndeclaredClass);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_error_deser_invalid_contract_class() {
        let error: SequencerError = serde_json::from_str(include_str!(
            "../../test-data/serde/sequencer_error_invalid_contract_class.json"
        ))
        .unwrap();

        assert_eq!(error.code, ErrorCode::InvalidContractClass);
    }
}
