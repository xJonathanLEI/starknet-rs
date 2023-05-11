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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct JsonRpcRequest {
    pub id: u64,
    pub data: JsonRpcRequestData,
}

#[derive(Debug, Clone)]
pub enum JsonRpcRequestData {
    GetBlockWithTxHashes(GetBlockWithTxHashesRequest),
    GetBlockWithTxs(GetBlockWithTxsRequest),
    GetStateUpdate(GetStateUpdateRequest),
    GetStorageAt(GetStorageAtRequest),
    GetTransactionByHash(GetTransactionByHashRequest),
    GetTransactionByBlockIdAndIndex(GetTransactionByBlockIdAndIndexRequest),
    GetTransactionReceipt(GetTransactionReceiptRequest),
    GetClass(GetClassRequest),
    GetClassHashAt(GetClassHashAtRequest),
    GetClassAt(GetClassAtRequest),
    GetBlockTransactionCount(GetBlockTransactionCountRequest),
    Call(CallRequest),
    EstimateFee(EstimateFeeRequest),
    BlockNumber(BlockNumberRequest),
    BlockHashAndNumber(BlockHashAndNumberRequest),
    ChainId(ChainIdRequest),
    PendingTransactions(PendingTransactionsRequest),
    Syncing(SyncingRequest),
    GetEvents(GetEventsRequest),
    GetNonce(GetNonceRequest),
    AddInvokeTransaction(AddInvokeTransactionRequest),
    AddDeclareTransaction(AddDeclareTransactionRequest),
    AddDeployTransaction(AddDeployTransactionRequest),
    AddDeployAccountTransaction(AddDeployAccountTransactionRequest),
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
    /// Get block information with transaction hashes given the block id
    pub async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::GetBlockWithTxHashes,
            GetBlockWithTxHashesRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get block information with full transactions given the block id
    pub async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::GetBlockWithTxs,
            GetBlockWithTxsRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get the information about the result of executing the requested block
    pub async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<StateUpdate, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::GetStateUpdate,
            GetStateUpdateRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get the value of the storage at the given address and key
    pub async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>>
    where
        A: AsRef<FieldElement>,
        K: AsRef<FieldElement>,
        B: AsRef<BlockId>,
    {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetStorageAt,
                GetStorageAtRequestRef {
                    contract_address: contract_address.as_ref(),
                    key: key.as_ref(),
                    block_id: block_id.as_ref(),
                },
            )
            .await?
            .0)
    }

    /// Get the details and status of a submitted transaction
    pub async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>>
    where
        H: AsRef<FieldElement>,
    {
        self.send_request(
            JsonRpcMethod::GetTransactionByHash,
            GetTransactionByHashRequestRef {
                transaction_hash: transaction_hash.as_ref(),
            },
        )
        .await
    }

    /// Get the details of a transaction by a given block id and index
    pub async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::GetTransactionByBlockIdAndIndex,
            GetTransactionByBlockIdAndIndexRequestRef {
                block_id: block_id.as_ref(),
                index: &index,
            },
        )
        .await
    }

    /// Get the details of a transaction by a given block number and index
    pub async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, JsonRpcClientError<T::Error>>
    where
        H: AsRef<FieldElement>,
    {
        self.send_request(
            JsonRpcMethod::GetTransactionReceipt,
            GetTransactionReceiptRequestRef {
                transaction_hash: transaction_hash.as_ref(),
            },
        )
        .await
    }

    /// Get the contract class definition in the given block associated with the given hash
    pub async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
        H: AsRef<FieldElement>,
    {
        self.send_request(
            JsonRpcMethod::GetClass,
            GetClassRequestRef {
                block_id: block_id.as_ref(),
                class_hash: class_hash.as_ref(),
            },
        )
        .await
    }

    /// Get the contract class hash in the given block for the contract deployed at the given address
    pub async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
        A: AsRef<FieldElement>,
    {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetClassHashAt,
                GetClassHashAtRequestRef {
                    block_id: block_id.as_ref(),
                    contract_address: contract_address.as_ref(),
                },
            )
            .await?
            .0)
    }

    /// Get the contract class definition in the given block at the given address
    pub async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
        A: AsRef<FieldElement>,
    {
        self.send_request(
            JsonRpcMethod::GetClassAt,
            GetClassAtRequestRef {
                block_id: block_id.as_ref(),
                contract_address: contract_address.as_ref(),
            },
        )
        .await
    }

    /// Get the number of transactions in a block given a block id
    pub async fn get_block_transaction_count<B>(
        &self,
        block_id: B,
    ) -> Result<u64, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::GetBlockTransactionCount,
            GetBlockTransactionCountRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Call a starknet function without creating a Starknet transaction
    pub async fn call<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<Vec<FieldElement>, JsonRpcClientError<T::Error>>
    where
        R: AsRef<FunctionCall>,
        B: AsRef<BlockId>,
    {
        Ok(self
            .send_request::<_, FeltArray>(
                JsonRpcMethod::Call,
                CallRequestRef {
                    request: request.as_ref(),
                    block_id: block_id.as_ref(),
                },
            )
            .await?
            .0)
    }

    /// Estimate the fee for a given Starknet transaction
    pub async fn estimate_fee<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<FeeEstimate, JsonRpcClientError<T::Error>>
    where
        R: AsRef<BroadcastedTransaction>,
        B: AsRef<BlockId>,
    {
        self.send_request(
            JsonRpcMethod::EstimateFee,
            EstimateFeeRequestRef {
                request: request.as_ref(),
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get the most recent accepted block number
    pub async fn block_number(&self) -> Result<u64, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::BlockNumber, BlockNumberRequest)
            .await
    }

    /// Get the most recent accepted block hash and number
    pub async fn block_hash_and_number(
        &self,
    ) -> Result<BlockHashAndNumber, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::BlockHashAndNumber, BlockHashAndNumberRequest)
            .await
    }

    /// Return the currently configured Starknet chain id
    pub async fn chain_id(&self) -> Result<FieldElement, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, Felt>(JsonRpcMethod::ChainId, ChainIdRequest)
            .await?
            .0)
    }

    /// Returns the transactions in the transaction pool, recognized by this sequencer
    pub async fn pending_transactions(
        &self,
    ) -> Result<Vec<Transaction>, JsonRpcClientError<T::Error>> {
        self.send_request(
            JsonRpcMethod::PendingTransactions,
            PendingTransactionsRequest,
        )
        .await
    }

    /// Returns an object about the sync status, or false if the node is not synching
    pub async fn syncing(&self) -> Result<SyncStatusType, JsonRpcClientError<T::Error>> {
        self.send_request(JsonRpcMethod::Syncing, SyncingRequest)
            .await
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
            GetEventsRequestRef {
                filter: &EventFilterWithPage {
                    event_filter: filter,
                    result_page_request: ResultPageRequest {
                        continuation_token,
                        chunk_size,
                    },
                },
            },
        )
        .await
    }

    /// Get the nonce associated with the given address in the given block
    pub async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, JsonRpcClientError<T::Error>>
    where
        B: AsRef<BlockId>,
        A: AsRef<FieldElement>,
    {
        Ok(self
            .send_request::<_, Felt>(
                JsonRpcMethod::GetNonce,
                GetNonceRequestRef {
                    block_id: block_id.as_ref(),
                    contract_address: contract_address.as_ref(),
                },
            )
            .await?
            .0)
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, JsonRpcClientError<T::Error>>
    where
        I: AsRef<BroadcastedInvokeTransaction>,
    {
        self.send_request(
            JsonRpcMethod::AddInvokeTransaction,
            AddInvokeTransactionRequestRef {
                invoke_transaction: invoke_transaction.as_ref(),
            },
        )
        .await
    }

    /// Submit a new transaction to be added to the chain
    pub async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, JsonRpcClientError<T::Error>>
    where
        D: AsRef<BroadcastedDeclareTransaction>,
    {
        self.send_request(
            JsonRpcMethod::AddDeclareTransaction,
            AddDeclareTransactionRequestRef {
                declare_transaction: declare_transaction.as_ref(),
            },
        )
        .await
    }

    /// Submit a new deploy contract transaction
    pub async fn add_deploy_transaction<D>(
        &self,
        deploy_transaction: D,
    ) -> Result<DeployTransactionResult, JsonRpcClientError<T::Error>>
    where
        D: AsRef<BroadcastedDeployTransaction>,
    {
        self.send_request(
            JsonRpcMethod::AddDeployTransaction,
            AddDeployTransactionRequestRef {
                deploy_transaction: deploy_transaction.as_ref(),
            },
        )
        .await
    }

    /// Submit a new deploy account transaction
    pub async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, JsonRpcClientError<T::Error>>
    where
        D: AsRef<BroadcastedDeployAccountTransaction>,
    {
        self.send_request(
            JsonRpcMethod::AddDeployAccountTransaction,
            AddDeployAccountTransactionRequestRef {
                deploy_account_transaction: deploy_account_transaction.as_ref(),
            },
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

impl<'de> Deserialize<'de> for JsonRpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawRequest {
            id: u64,
            method: JsonRpcMethod,
            params: serde_json::Value,
        }

        let error_mapper =
            |err| serde::de::Error::custom(format!("unable to decode params: {}", err));

        let raw_request = RawRequest::deserialize(deserializer)?;
        let request_data = match raw_request.method {
            JsonRpcMethod::GetBlockWithTxHashes => JsonRpcRequestData::GetBlockWithTxHashes(
                serde_json::from_value::<GetBlockWithTxHashesRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockWithTxs => JsonRpcRequestData::GetBlockWithTxs(
                serde_json::from_value::<GetBlockWithTxsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetStateUpdate => JsonRpcRequestData::GetStateUpdate(
                serde_json::from_value::<GetStateUpdateRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetStorageAt => JsonRpcRequestData::GetStorageAt(
                serde_json::from_value::<GetStorageAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetTransactionByHash => JsonRpcRequestData::GetTransactionByHash(
                serde_json::from_value::<GetTransactionByHashRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetTransactionByBlockIdAndIndex => {
                JsonRpcRequestData::GetTransactionByBlockIdAndIndex(
                    serde_json::from_value::<GetTransactionByBlockIdAndIndexRequest>(
                        raw_request.params,
                    )
                    .map_err(error_mapper)?,
                )
            }
            JsonRpcMethod::GetTransactionReceipt => JsonRpcRequestData::GetTransactionReceipt(
                serde_json::from_value::<GetTransactionReceiptRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClass => JsonRpcRequestData::GetClass(
                serde_json::from_value::<GetClassRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClassHashAt => JsonRpcRequestData::GetClassHashAt(
                serde_json::from_value::<GetClassHashAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClassAt => JsonRpcRequestData::GetClassAt(
                serde_json::from_value::<GetClassAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockTransactionCount => {
                JsonRpcRequestData::GetBlockTransactionCount(
                    serde_json::from_value::<GetBlockTransactionCountRequest>(raw_request.params)
                        .map_err(error_mapper)?,
                )
            }
            JsonRpcMethod::Call => JsonRpcRequestData::Call(
                serde_json::from_value::<CallRequest>(raw_request.params).map_err(error_mapper)?,
            ),
            JsonRpcMethod::EstimateFee => JsonRpcRequestData::EstimateFee(
                serde_json::from_value::<EstimateFeeRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::BlockNumber => JsonRpcRequestData::BlockNumber(
                serde_json::from_value::<BlockNumberRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::BlockHashAndNumber => JsonRpcRequestData::BlockHashAndNumber(
                serde_json::from_value::<BlockHashAndNumberRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::ChainId => JsonRpcRequestData::ChainId(
                serde_json::from_value::<ChainIdRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::PendingTransactions => JsonRpcRequestData::PendingTransactions(
                serde_json::from_value::<PendingTransactionsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::Syncing => JsonRpcRequestData::Syncing(
                serde_json::from_value::<SyncingRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetEvents => JsonRpcRequestData::GetEvents(
                serde_json::from_value::<GetEventsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetNonce => JsonRpcRequestData::GetNonce(
                serde_json::from_value::<GetNonceRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddInvokeTransaction => JsonRpcRequestData::AddInvokeTransaction(
                serde_json::from_value::<AddInvokeTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddDeclareTransaction => JsonRpcRequestData::AddDeclareTransaction(
                serde_json::from_value::<AddDeclareTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddDeployTransaction => JsonRpcRequestData::AddDeployTransaction(
                serde_json::from_value::<AddDeployTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddDeployAccountTransaction => {
                JsonRpcRequestData::AddDeployAccountTransaction(
                    serde_json::from_value::<AddDeployAccountTransactionRequest>(
                        raw_request.params,
                    )
                    .map_err(error_mapper)?,
                )
            }
        };

        Ok(Self {
            id: raw_request.id,
            data: request_data,
        })
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
            50 => ErrorCode::InvalidContractClass,
            _ => return Err(()),
        })
    }
}
