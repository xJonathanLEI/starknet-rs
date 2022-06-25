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
    #[serde(rename = "starknet_getTransactionByBlockHashAndIndex")]
    GetTransactionByBlockHashAndIndex,
    #[serde(rename = "starknet_getTransactionByBlockNumberAndIndex")]
    GetTransactionByBlockNumberAndIndex,
    #[serde(rename = "starknet_getTransactionReceipt")]
    GetTransactionReceipt,
    #[serde(rename = "starknet_getClass")]
    GetClass,
    #[serde(rename = "starknet_getClassHashAt")]
    GetClassHashAt,
    #[serde(rename = "starknet_getClassAt")]
    GetClassAt,
    #[serde(rename = "starknet_getBlockTransactionCountByHash")]
    GetBlockTransactionCountByHash,
    #[serde(rename = "starknet_getBlockTransactionCountByNumber")]
    GetBlockTransactionCountByNumber,
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
    #[serde(rename = "starknet_chainId")]
    ChainId,
    #[serde(rename = "starknet_syncing")]
    Syncing,
    #[serde(rename = "starknet_getEvents")]
    GetEvents,
    #[serde(rename = "starknet_call")]
    Call,
    #[serde(rename = "starknet_addInvokeTransaction")]
    AddInvokeTransaction,
    #[serde(rename = "starknet_addDeclareTransaction")]
    AddDeclareTransaction,
    #[serde(rename = "starknet_addDeployTransaction")]
    AddDeployTransaction,
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

#[derive(Serialize)]
struct EventFilterWithPage {
    #[serde(flatten)]
    filter: EventFilter,
    page_size: u64,
    page_number: u64,
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

    /// Get the details of a transaction by a given block hash and index
    pub async fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: &BlockHashOrTag,
        index: u64,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionByBlockHashAndIndex,
            [
                serde_json::to_value(block_hash)?,
                serde_json::to_value(index)?,
            ],
        )
        .await
    }

    /// Get the details of a transaction by a given block number and index
    pub async fn get_transaction_by_block_number_and_index(
        &self,
        block_number: &BlockNumOrTag,
        index: u64,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionByBlockNumberAndIndex,
            [
                serde_json::to_value(block_number)?,
                serde_json::to_value(index)?,
            ],
        )
        .await
    }

    /// Get the details of a transaction by a given block number and index
    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetTransactionReceipt,
            [serde_json::to_value(Felt(transaction_hash))?],
        )
        .await
    }

    /// Get the contract class definition associated with the given hash
    pub async fn get_class(
        &self,
        class_hash: FieldElement,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetClass,
            [serde_json::to_value(Felt(class_hash))?],
        )
        .await
    }

    /// Get the contract class hash for the contract deployed at the given address
    pub async fn get_class_hash_at(
        &self,
        contract_address: FieldElement,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetClassHashAt,
                [serde_json::to_value(Felt(contract_address))?],
            )
            .await?
            .0)
    }

    /// Get the contract class definition at the given address
    pub async fn get_class_at(
        &self,
        contract_address: FieldElement,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetClassAt,
            [serde_json::to_value(Felt(contract_address))?],
        )
        .await
    }

    /// Get the number of transactions in a block given a block hash
    pub async fn get_block_transaction_count_by_hash(
        &self,
        block_hash: &BlockHashOrTag,
    ) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockTransactionCountByHash,
            [serde_json::to_value(block_hash)?],
        )
        .await
    }

    /// Get the number of transactions in a block given a block number (height)
    pub async fn get_block_transaction_count_by_number(
        &self,
        block_number: &BlockNumOrTag,
    ) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetBlockTransactionCountByNumber,
            [serde_json::to_value(block_number)?],
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

    /// Returns all events matching the given filter
    pub async fn get_events(
        &self,
        filter: EventFilter,
        page_size: u64,
        page_number: u64,
    ) -> Result<EventsPage, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::GetEvents,
            [serde_json::to_value(EventFilterWithPage {
                filter,
                page_size,
                page_number,
            })?],
        )
        .await
    }

    /// Call a starknet function without creating a StarkNet transaction
    pub async fn call<R>(
        &self,
        request: R,
        block_hash: &BlockHashOrTag,
    ) -> Result<Vec<FieldElement>, JsonRpcClientError<T::Error>>
    where
        R: AsRef<FunctionCall>,
    {
        Ok(self
            .send_request::<_, FeltArray>(
                JsonRpcMethod::Call,
                [
                    serde_json::to_value(request.as_ref())?,
                    serde_json::to_value(block_hash)?,
                ],
            )
            .await?
            .0)
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_invoke_transaction(
        &self,
        function_invocation: &FunctionCall,
        signature: Vec<FieldElement>,
        max_fee: FieldElement,
        version: FieldElement,
    ) -> Result<InvokeTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddInvokeTransaction,
            [
                serde_json::to_value(function_invocation)?,
                serde_json::to_value(FeltArray(signature))?,
                serde_json::to_value(Felt(max_fee))?,
                serde_json::to_value(Felt(version))?,
            ],
        )
        .await
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_declare_transaction(
        &self,
        contract_class: &CompressedContractClass,
        version: FieldElement,
    ) -> Result<DeclareTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddDeclareTransaction,
            [
                serde_json::to_value(contract_class)?,
                serde_json::to_value(Felt(version))?,
            ],
        )
        .await
    }

    /// Submit a new deploy contract transaction
    pub async fn add_deploy_transaction(
        &self,
        contract_address_salt: FieldElement,
        constructor_calldata: Vec<FieldElement>,
        contract_definition: &CompressedContractClass,
    ) -> Result<DeployTransactionResult, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::AddDeployTransaction,
            [
                serde_json::to_value(Felt(contract_address_salt))?,
                serde_json::to_value(FeltArray(constructor_calldata))?,
                serde_json::to_value(contract_definition)?,
            ],
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
            JsonRpcResponse::Error { error, .. } => Err(JsonRpcClientError::RpcError(error)),
        }
    }
}

impl<T> From<serde_json::Error> for JsonRpcClientError<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
