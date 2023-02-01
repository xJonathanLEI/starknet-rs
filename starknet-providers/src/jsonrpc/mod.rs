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

/// Temporary module for bridging the client and the `Provider` trait until:
///
/// https://github.com/xJonathanLEI/starknet-rs/issues/77#issuecomment-1150184364
mod provider;
pub use provider::JsonRpcProviderError;

#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

#[derive(Debug, Serialize)]
pub enum JsonRpcMethod {
    #[serde(rename = "starknet_getBlockWithTxHashes")]
    GetBlockWithTxHashes,
    #[serde(rename = "starknet_getBlockWithTxs")]
    GetBlockWithTxs,
    #[serde(rename = "starknet_getStateUpdate")]
    GetStateUpdate,
    #[serde(rename = "starknet_getStorageAt")]
    GetStorageAt,
    #[serde(rename = "starknet_getTransactionByHash")]
    GetTransactionByHash,
    #[serde(rename = "starknet_getTransactionByBlockIdAndIndex")]
    GetTransactionByBlockIdAndIndex,
    #[serde(rename = "starknet_getTransactionReceipt")]
    GetTransactionReceipt,
    #[serde(rename = "starknet_getClass")]
    GetClass,
    #[serde(rename = "starknet_getClassHashAt")]
    GetClassHashAt,
    #[serde(rename = "starknet_getClassAt")]
    GetClassAt,
    #[serde(rename = "starknet_getBlockTransactionCount")]
    GetBlockTransactionCount,
    #[serde(rename = "starknet_call")]
    Call,
    #[serde(rename = "starknet_estimateFee")]
    EstimateFee,
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
    #[serde(rename = "starknet_blockHashAndNumber")]
    BlockHashAndNumber,
    #[serde(rename = "starknet_chainId")]
    ChainId,
    #[serde(rename = "starknet_pendingTransactions")]
    PendingTransactions,
    #[serde(rename = "starknet_syncing")]
    Syncing,
    #[serde(rename = "starknet_getEvents")]
    GetEvents,
    #[serde(rename = "starknet_getNonce")]
    GetNonce,
    #[serde(rename = "starknet_addInvokeTransaction")]
    AddInvokeTransaction,
    #[serde(rename = "starknet_addDeclareTransaction")]
    AddDeclareTransaction,
    #[serde(rename = "starknet_addDeployTransaction")]
    AddDeployTransaction,
    #[serde(rename = "starknet_addDeployAccountTransaction")]
    AddDeployAccountTransaction,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError<T> {
    #[error(transparent)]
    JsonError(serde_json::Error),
    #[error(transparent)]
    TransportError(T),
    #[error(transparent)]
    RpcError(RpcError),
}

#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error(transparent)]
    Code(ErrorCode),
    #[error(transparent)]
    Unknown(JsonRpcError),
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

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Felt(#[serde_as(as = "UfeHex")] pub FieldElement);

#[serde_as]
#[derive(Serialize, Deserialize)]
struct FeltArray(#[serde_as(as = "Vec<UfeHex>")] pub Vec<FieldElement>);

#[derive(Serialize)]
struct EventFilterWithPage {
    #[serde(flatten)]
    filter: EventFilter,
    #[serde(flatten)]
    page: ResultPageRequest,
}

impl<T> JsonRpcClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> JsonRpcClient<T>
where
    T: JsonRpcTransport,
{
    /// Get block information with transaction hashes given the block id
    pub async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockWithTxHashes,
            [serde_json::to_value(block_id)?],
        )
        .await
    }

    /// Get block information with full transactions given the block id
    pub async fn get_block_with_txs(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxs, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockWithTxs,
            [serde_json::to_value(block_id)?],
        )
        .await
    }

    /// Get the information about the result of executing the requested block
    pub async fn get_state_update(
        &self,
        block_id: &BlockId,
    ) -> Result<StateUpdate, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetStateUpdate,
            [serde_json::to_value(block_id)?],
        )
        .await
    }

    /// Get the value of the storage at the given address and key
    pub async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: &BlockId,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetStorageAt,
                [
                    serde_json::to_value(Felt(contract_address))?,
                    serde_json::to_value(Felt(key))?,
                    serde_json::to_value(block_id)?,
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

    /// Get the details of a transaction by a given block id and index
    pub async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionByBlockIdAndIndex,
            [
                serde_json::to_value(block_id)?,
                serde_json::to_value(index)?,
            ],
        )
        .await
    }

    /// Get the details of a transaction by a given block number and index
    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionReceipt,
            [serde_json::to_value(Felt(transaction_hash))?],
        )
        .await
    }

    /// Get the contract class definition in the given block associated with the given hash
    pub async fn get_class(
        &self,
        block_id: &BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetClass,
            [
                serde_json::to_value(block_id)?,
                serde_json::to_value(Felt(class_hash))?,
            ],
        )
        .await
    }

    /// Get the contract class hash in the given block for the contract deployed at the given address
    pub async fn get_class_hash_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetClassHashAt,
                [
                    serde_json::to_value(block_id)?,
                    serde_json::to_value(Felt(contract_address))?,
                ],
            )
            .await?
            .0)
    }

    /// Get the contract class definition in the given block at the given address
    pub async fn get_class_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetClassAt,
            [
                serde_json::to_value(block_id)?,
                serde_json::to_value(Felt(contract_address))?,
            ],
        )
        .await
    }

    /// Get the number of transactions in a block given a block id
    pub async fn get_block_transaction_count(
        &self,
        block_id: &BlockId,
    ) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockTransactionCount,
            [serde_json::to_value(block_id)?],
        )
        .await
    }

    /// Call a starknet function without creating a Starknet transaction
    pub async fn call<R>(
        &self,
        request: R,
        block_id: &BlockId,
    ) -> Result<Vec<FieldElement>, JsonRpcClientError<T::Error>>
    where
        R: AsRef<FunctionCall>,
    {
        Ok(self
            .send_request::<_, FeltArray>(
                JsonRpcMethod::Call,
                [
                    serde_json::to_value(request.as_ref())?,
                    serde_json::to_value(block_id)?,
                ],
            )
            .await?
            .0)
    }

    /// Estimate the fee for a given Starknet transaction
    pub async fn estimate_fee<R>(
        &self,
        request: R,
        block_id: &BlockId,
    ) -> Result<FeeEstimate, JsonRpcClientError<T::Error>>
    where
        R: AsRef<BroadcastedTransaction>,
    {
        self.send_request(
            JsonRpcMethod::EstimateFee,
            [
                serde_json::to_value(request.as_ref())?,
                serde_json::to_value(block_id)?,
            ],
        )
        .await
    }

    /// Get the most recent accepted block number
    pub async fn block_number(&self) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::BlockNumber, ()).await
    }

    /// Get the most recent accepted block hash and number
    pub async fn block_hash_and_number(
        &self,
    ) -> Result<BlockHashAndNumber, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::BlockHashAndNumber, ())
            .await
    }

    /// Return the currently configured Starknet chain id
    pub async fn chain_id(&self) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(JsonRpcMethod::ChainId, ())
            .await?
            .0)
    }

    /// Returns the transactions in the transaction pool, recognized by this sequencer
    pub async fn pending_transactions(
        &self,
    ) -> Result<Vec<Transaction>, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::PendingTransactions, ())
            .await
    }

    /// Returns an object about the sync status, or false if the node is not synching
    pub async fn syncing(&self) -> Result<SyncStatusType, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::Syncing, ()).await
    }

    /// Returns all events matching the given filter
    pub async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetEvents,
            [serde_json::to_value(EventFilterWithPage {
                filter,
                page: ResultPageRequest {
                    continuation_token,
                    chunk_size,
                },
            })?],
        )
        .await
    }

    /// Get the nonce associated with the given address in the given block
    pub async fn get_nonce(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetNonce,
                [
                    serde_json::to_value(block_id)?,
                    serde_json::to_value(Felt(contract_address))?,
                ],
            )
            .await?
            .0)
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_invoke_transaction(
        &self,
        invoke_transaction: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddInvokeTransaction,
            [serde_json::to_value(invoke_transaction)?],
        )
        .await
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_declare_transaction(
        &self,
        declare_transaction: &BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddDeclareTransaction,
            [serde_json::to_value(declare_transaction)?],
        )
        .await
    }

    /// Submit a new deploy contract transaction
    pub async fn add_deploy_transaction(
        &self,
        deploy_transaction: &BroadcastedDeployTransaction,
    ) -> Result<DeployTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddDeployTransaction,
            [serde_json::to_value(deploy_transaction)?],
        )
        .await
    }

    /// Submit a new deploy account transaction
    pub async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: &BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddDeployAccountTransaction,
            [serde_json::to_value(deploy_account_transaction)?],
        )
        .await
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
            JsonRpcResponse::Error { error, .. } => {
                Err(JsonRpcClientError::RpcError(match error.code.try_into() {
                    Ok(code) => RpcError::Code(code),
                    Err(_) => RpcError::Unknown(error),
                }))
            }
        }
    }
}

impl<T> From<serde_json::Error> for JsonRpcClientError<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl TryFrom<i64> for ErrorCode {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => ErrorCode::FailedToReceiveTransaction,
            20 => ErrorCode::ContractNotFound,
            21 => ErrorCode::InvalidMessageSelector,
            22 => ErrorCode::InvalidCallData,
            24 => ErrorCode::BlockNotFound,
            25 => ErrorCode::TransactionHashNotFound,
            27 => ErrorCode::InvalidTransactionIndex,
            28 => ErrorCode::ClassHashNotFound,
            31 => ErrorCode::PageSizeTooBig,
            32 => ErrorCode::NoBlocks,
            33 => ErrorCode::InvalidContinuationToken,
            40 => ErrorCode::ContractError,
            _ => return Err(()),
        })
    }
}
