use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};

use crate::jsonrpc::models::*;

mod transports;
pub use transports::{HttpTransport, JsonRpcTransport};

/// Temporary module for holding JSON-RPC data models until the provider switch:
///
/// https://github.com/xJonathanLEI/starknet-rs/issues/77#issuecomment-1150184364
pub mod models;

#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

#[derive(Debug, Serialize)]
pub enum JsonRpcMethod {
    #[serde(rename = "starknet_getBlockByHash")]
    GetBlockByHash,
    #[serde(rename = "starknet_getBlockByNumber")]
    GetBlockByNumber,
    #[serde(rename = "starknet_getStorageAt")]
    GetStorageAt,
    #[serde(rename = "starknet_getTransactionByHash")]
    GetTransactionByHash,
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
    #[serde(rename = "starknet_chainId")]
    ChainId,
    #[serde(rename = "starknet_syncing")]
    Syncing,
    #[serde(rename = "starknet_call")]
    Call,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError<T> {
    #[error(transparent)]
    JsonError(serde_json::Error),
    #[error(transparent)]
    TransportError(T),
    #[error(transparent)]
    RpcError(JsonRpcError),
}

#[derive(Debug, thiserror::Error, Deserialize)]
#[error("JSON-RPC error: code={code}, message=\"{message}\"")]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse<T> {
    Success { id: u64, result: T },
    Error { id: u64, error: JsonRpcError },
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum BlockResponseScopeOptions {
    TxnHash,
    FullTxns,
    FullTxnAndReceipts,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Felt(#[serde_as(as = "UfeHex")] pub FieldElement);

#[serde_as]
#[derive(Serialize, Deserialize)]
struct FeltArray(#[serde_as(as = "Vec<UfeHex>")] pub Vec<FieldElement>);

impl<T> JsonRpcClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> JsonRpcClient<T>
where
    T: JsonRpcTransport,
{
    /// Get block information given the block id
    pub async fn get_block_by_hash(
        &self,
        block_hash: &BlockHashOrTag,
    ) -> Result<Block, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByHash,
            [
                serde_json::to_value(block_hash)?,
                serde_json::to_value(BlockResponseScopeOptions::TxnHash)?,
            ],
        )
        .await
    }

    /// Get block information given the block id
    pub async fn get_block_by_hash_with_txns(
        &self,
        block_hash: &BlockHashOrTag,
    ) -> Result<BlockWithTxns, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByHash,
            [
                serde_json::to_value(block_hash)?,
                serde_json::to_value(BlockResponseScopeOptions::FullTxns)?,
            ],
        )
        .await
    }

    /// Get block information given the block id
    pub async fn get_block_by_hash_with_receipts(
        &self,
        block_hash: &BlockHashOrTag,
    ) -> Result<BlockWithReceipts, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByHash,
            [
                serde_json::to_value(block_hash)?,
                serde_json::to_value(BlockResponseScopeOptions::FullTxnAndReceipts)?,
            ],
        )
        .await
    }

    /// Get block information given the block number (its height)
    pub async fn get_block_by_number(
        &self,
        block_number: &BlockNumOrTag,
    ) -> Result<Block, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByNumber,
            [
                serde_json::to_value(block_number)?,
                serde_json::to_value(BlockResponseScopeOptions::TxnHash)?,
            ],
        )
        .await
    }

    /// Get block information given the block number (its height)
    pub async fn get_block_by_number_with_txns(
        &self,
        block_number: &BlockNumOrTag,
    ) -> Result<BlockWithTxns, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByNumber,
            [
                serde_json::to_value(block_number)?,
                serde_json::to_value(BlockResponseScopeOptions::FullTxns)?,
            ],
        )
        .await
    }

    /// Get block information given the block number (its height)
    pub async fn get_block_by_number_with_receipts(
        &self,
        block_number: &BlockNumOrTag,
    ) -> Result<BlockWithReceipts, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockByNumber,
            [
                serde_json::to_value(block_number)?,
                serde_json::to_value(BlockResponseScopeOptions::FullTxnAndReceipts)?,
            ],
        )
        .await
    }

    /// Get the value of the storage at the given address and key
    pub async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_hash: &BlockHashOrTag,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetStorageAt,
                [
                    serde_json::to_value(Felt(contract_address))?,
                    serde_json::to_value(Felt(key))?,
                    serde_json::to_value(block_hash)?,
                ],
            )
            .await?
            .0)
    }

    /// Get the details and status of a submitted transaction
    pub async fn get_transaction_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionByHash,
            [serde_json::to_value(Felt(transaction_hash))?],
        )
        .await
    }

    /// Get the most recent accepted block number
    pub async fn block_number(&self) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::BlockNumber, ()).await
    }

    /// Return the currently configured StarkNet chain id
    pub async fn chain_id(&self) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(JsonRpcMethod::ChainId, ())
            .await?
            .0)
    }

    /// Returns an object about the sync status, or false if the node is not synching
    pub async fn syncing(&self) -> Result<SyncStatusType, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::Syncing, ()).await
    }

    /// Call a starknet function without creating a StarkNet transaction
    pub async fn call(
        &self,
        request: &FunctionCall,
        block_hash: &BlockHashOrTag,
    ) -> Result<Vec<FieldElement>, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, FeltArray>(
                JsonRpcMethod::Call,
                [
                    serde_json::to_value(request)?,
                    serde_json::to_value(block_hash)?,
                ],
            )
            .await?
            .0)
    }

    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<R, JsonRpcClientError<T::Error>>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        match self
            .transport
            .send_request(method, params)
            .await
            .map_err(JsonRpcClientError::TransportError)?
        {
            JsonRpcResponse::Success { result, .. } => Ok(result),
            JsonRpcResponse::Error { error, .. } => Err(JsonRpcClientError::RpcError(error)),
        }
    }
}

impl<T> From<serde_json::Error> for JsonRpcClientError<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
