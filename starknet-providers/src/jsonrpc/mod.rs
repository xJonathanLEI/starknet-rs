use std::{any::Any, error::Error, fmt::Display};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::UfeHex,
    types::{
        requests::*, BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
        BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
        ContractClass, ContractErrorData, DeclareTransactionResult, DeployAccountTransactionResult,
        EventFilter, EventFilterWithPage, EventsPage, FeeEstimate, Felt as FeltPrimitive,
        FunctionCall, InvokeTransactionResult, MaybePendingBlockWithReceipts,
        MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
        MsgFromL1, NoTraceAvailableErrorData, ResultPageRequest, SimulatedTransaction,
        SimulationFlag, SimulationFlagForEstimateFee, StarknetError, SyncStatusType, Transaction,
        TransactionExecutionErrorData, TransactionReceiptWithBlockInfo, TransactionStatus,
        TransactionTrace, TransactionTraceWithHash,
    },
};

use crate::{
    provider::ProviderImplError, Provider, ProviderError, ProviderRequestData, ProviderResponseData,
};

mod transports;
pub use transports::{HttpTransport, HttpTransportError, JsonRpcTransport};

/// A generic JSON-RPC client with any transport.
///
/// A "transport" is any implementation that can send JSON-RPC requests and receive responses. This
/// most commonly happens over a network via HTTP connections, as with [`HttpTransport`].
#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

/// All JSON-RPC methods as listed by the official specification.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JsonRpcMethod {
    /// The `starknet_specVersion` method.
    #[serde(rename = "starknet_specVersion")]
    SpecVersion,
    /// The `starknet_getBlockWithTxHashes` method.
    #[serde(rename = "starknet_getBlockWithTxHashes")]
    GetBlockWithTxHashes,
    /// The `starknet_getBlockWithTxs` method.
    #[serde(rename = "starknet_getBlockWithTxs")]
    GetBlockWithTxs,
    /// The `starknet_getBlockWithReceipts` method.
    #[serde(rename = "starknet_getBlockWithReceipts")]
    GetBlockWithReceipts,
    /// The `starknet_getStateUpdate` method.
    #[serde(rename = "starknet_getStateUpdate")]
    GetStateUpdate,
    /// The `starknet_getStorageAt` method.
    #[serde(rename = "starknet_getStorageAt")]
    GetStorageAt,
    /// The `starknet_getTransactionStatus` method.
    #[serde(rename = "starknet_getTransactionStatus")]
    GetTransactionStatus,
    /// The `starknet_getTransactionByHash` method.
    #[serde(rename = "starknet_getTransactionByHash")]
    GetTransactionByHash,
    /// The `starknet_getTransactionByBlockIdAndIndex` method.
    #[serde(rename = "starknet_getTransactionByBlockIdAndIndex")]
    GetTransactionByBlockIdAndIndex,
    /// The `starknet_getTransactionReceipt` method.
    #[serde(rename = "starknet_getTransactionReceipt")]
    GetTransactionReceipt,
    /// The `starknet_getClass` method.
    #[serde(rename = "starknet_getClass")]
    GetClass,
    /// The `starknet_getClassHashAt` method.
    #[serde(rename = "starknet_getClassHashAt")]
    GetClassHashAt,
    /// The `starknet_getClassAt` method.
    #[serde(rename = "starknet_getClassAt")]
    GetClassAt,
    /// The `starknet_getBlockTransactionCount` method.
    #[serde(rename = "starknet_getBlockTransactionCount")]
    GetBlockTransactionCount,
    /// The `starknet_call` method.
    #[serde(rename = "starknet_call")]
    Call,
    /// The `starknet_estimateFee` method.
    #[serde(rename = "starknet_estimateFee")]
    EstimateFee,
    /// The `starknet_estimateMessageFee` method.
    #[serde(rename = "starknet_estimateMessageFee")]
    EstimateMessageFee,
    /// The `starknet_blockNumber` method.
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
    /// The `starknet_blockHashAndNumber` method.
    #[serde(rename = "starknet_blockHashAndNumber")]
    BlockHashAndNumber,
    /// The `starknet_chainId` method.
    #[serde(rename = "starknet_chainId")]
    ChainId,
    /// The `starknet_syncing` method.
    #[serde(rename = "starknet_syncing")]
    Syncing,
    /// The `starknet_getEvents` method.
    #[serde(rename = "starknet_getEvents")]
    GetEvents,
    /// The `starknet_getNonce` method.
    #[serde(rename = "starknet_getNonce")]
    GetNonce,
    /// The `starknet_addInvokeTransaction` method.
    #[serde(rename = "starknet_addInvokeTransaction")]
    AddInvokeTransaction,
    /// The `starknet_addDeclareTransaction` method.
    #[serde(rename = "starknet_addDeclareTransaction")]
    AddDeclareTransaction,
    /// The `starknet_addDeployAccountTransaction` method.
    #[serde(rename = "starknet_addDeployAccountTransaction")]
    AddDeployAccountTransaction,
    /// The `starknet_traceTransaction` method.
    #[serde(rename = "starknet_traceTransaction")]
    TraceTransaction,
    /// The `starknet_simulateTransactions` method.
    #[serde(rename = "starknet_simulateTransactions")]
    SimulateTransactions,
    /// The `starknet_traceBlockTransactions` method.
    #[serde(rename = "starknet_traceBlockTransactions")]
    TraceBlockTransactions,
}

/// JSON-RPC request.
#[derive(Debug, Clone)]
pub struct JsonRpcRequest {
    /// ID of the request. Useful for identifying responses in certain transports like `WebSocket`.
    pub id: u64,
    /// Data of the requeest.
    pub data: ProviderRequestData,
}

/// Errors from JSON-RPC client.
#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError<T> {
    /// JSON serialization/deserialization erors.
    #[error(transparent)]
    JsonError(serde_json::Error),
    /// Transport-specific errors.
    #[error(transparent)]
    TransportError(T),
    /// An unsuccessful response returned from the server is encountered.
    #[error(transparent)]
    JsonRpcError(JsonRpcError),
}

/// An unsuccessful response returned from the server.
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i64,
    /// Error message.
    pub message: String,
    /// Additional error data if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC response returned from a server.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse<T> {
    /// Successful response.
    Success {
        /// Same ID as the corresponding request.
        id: u64,
        /// Response data.
        result: T,
    },
    /// Unsuccessful response.
    Error {
        /// Same ID as the corresponding request.
        id: u64,
        /// Error details.
        error: JsonRpcError,
    },
}

/// Failures trying to parse a [`JsonRpcError`] into [`StarknetError`].
///
/// [`StarknetError`] is the standard, provider-agnostic error type that all [`Provider`]
/// implementations should strive to return in an error case, in a best-effort basis. This allows
/// for unified error handling logic.
///
/// However, not all error cases can be properly converted, and this error type represents the cases
/// when such failure happens.
#[derive(Debug, thiserror::Error)]
pub enum JsonRpcErrorConversionError {
    /// The error code is outside of the range specified by the specification.
    #[error("unknown error code")]
    UnknownCode,
    /// Error data is expected but missing.
    #[error("missing data field")]
    MissingData,
    /// Error data is malformed.
    #[error("unable to parse the data field")]
    DataParsingFailure,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Felt(#[serde_as(as = "UfeHex")] pub FeltPrimitive);

#[serde_as]
#[derive(Serialize, Deserialize)]
struct FeltArray(#[serde_as(as = "Vec<UfeHex>")] pub Vec<FeltPrimitive>);

impl<T> JsonRpcClient<T> {
    /// Constructs a new [`JsonRpcClient`] from a transport.
    pub const fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> JsonRpcClient<T>
where
    T: 'static + JsonRpcTransport + Send + Sync,
{
    async fn send_request<P, R>(&self, method: JsonRpcMethod, params: P) -> Result<R, ProviderError>
    where
        P: Serialize + Send + Sync,
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
                Err(match TryInto::<StarknetError>::try_into(&error) {
                    Ok(error) => ProviderError::StarknetError(error),
                    Err(_) => JsonRpcClientError::<T::Error>::JsonRpcError(error).into(),
                })
            }
        }
    }

    async fn send_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<ProviderResponseData>, ProviderError>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync,
    {
        let mut results = vec![];

        let responses = self
            .transport
            .send_requests(requests.as_ref().to_vec())
            .await
            .map_err(JsonRpcClientError::TransportError)?;

        for (request, response) in requests.as_ref().iter().zip(responses.into_iter()) {
            match response {
                JsonRpcResponse::Success { result, .. } => {
                    let result = match request {
                        ProviderRequestData::SpecVersion(_) => ProviderResponseData::SpecVersion(
                            String::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::GetBlockWithTxHashes(_) => {
                            ProviderResponseData::GetBlockWithTxHashes(
                                MaybePendingBlockWithTxHashes::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetBlockWithTxs(_) => {
                            ProviderResponseData::GetBlockWithTxs(
                                MaybePendingBlockWithTxs::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetBlockWithReceipts(_) => {
                            ProviderResponseData::GetBlockWithReceipts(
                                MaybePendingBlockWithReceipts::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetStateUpdate(_) => {
                            ProviderResponseData::GetStateUpdate(
                                MaybePendingStateUpdate::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetStorageAt(_) => ProviderResponseData::GetStorageAt(
                            Felt::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?
                                .0,
                        ),
                        ProviderRequestData::GetTransactionStatus(_) => {
                            ProviderResponseData::GetTransactionStatus(
                                TransactionStatus::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetTransactionByHash(_) => {
                            ProviderResponseData::GetTransactionByHash(
                                Transaction::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetTransactionByBlockIdAndIndex(_) => {
                            ProviderResponseData::GetTransactionByBlockIdAndIndex(
                                Transaction::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetTransactionReceipt(_) => {
                            ProviderResponseData::GetTransactionReceipt(
                                TransactionReceiptWithBlockInfo::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::GetClass(_) => ProviderResponseData::GetClass(
                            ContractClass::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::GetClassHashAt(_) => {
                            ProviderResponseData::GetClassHashAt(
                                Felt::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?
                                    .0,
                            )
                        }
                        ProviderRequestData::GetClassAt(_) => ProviderResponseData::GetClassAt(
                            ContractClass::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::GetBlockTransactionCount(_) => {
                            ProviderResponseData::GetBlockTransactionCount(
                                u64::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::Call(_) => ProviderResponseData::Call(
                            FeltArray::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?
                                .0,
                        ),
                        ProviderRequestData::EstimateFee(_) => ProviderResponseData::EstimateFee(
                            Vec::<FeeEstimate>::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::EstimateMessageFee(_) => {
                            ProviderResponseData::EstimateMessageFee(
                                FeeEstimate::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::BlockNumber(_) => ProviderResponseData::BlockNumber(
                            u64::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::BlockHashAndNumber(_) => {
                            ProviderResponseData::BlockHashAndNumber(
                                BlockHashAndNumber::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::ChainId(_) => ProviderResponseData::ChainId(
                            Felt::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?
                                .0,
                        ),
                        ProviderRequestData::Syncing(_) => ProviderResponseData::Syncing(
                            SyncStatusType::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::GetEvents(_) => ProviderResponseData::GetEvents(
                            EventsPage::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                        ),
                        ProviderRequestData::GetNonce(_) => ProviderResponseData::GetNonce(
                            Felt::deserialize(result)
                                .map_err(JsonRpcClientError::<T::Error>::JsonError)?
                                .0,
                        ),
                        ProviderRequestData::AddInvokeTransaction(_) => {
                            ProviderResponseData::AddInvokeTransaction(
                                InvokeTransactionResult::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::AddDeclareTransaction(_) => {
                            ProviderResponseData::AddDeclareTransaction(
                                DeclareTransactionResult::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::AddDeployAccountTransaction(_) => {
                            ProviderResponseData::AddDeployAccountTransaction(
                                DeployAccountTransactionResult::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::TraceTransaction(_) => {
                            ProviderResponseData::TraceTransaction(
                                TransactionTrace::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::SimulateTransactions(_) => {
                            ProviderResponseData::SimulateTransactions(
                                Vec::<SimulatedTransaction>::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                        ProviderRequestData::TraceBlockTransactions(_) => {
                            ProviderResponseData::TraceBlockTransactions(
                                Vec::<TransactionTraceWithHash>::deserialize(result)
                                    .map_err(JsonRpcClientError::<T::Error>::JsonError)?,
                            )
                        }
                    };

                    results.push(result);
                }
                // TODO: add context on index of request causing the error
                JsonRpcResponse::Error { error, .. } => {
                    return Err(match TryInto::<StarknetError>::try_into(&error) {
                        Ok(error) => ProviderError::StarknetError(error),
                        Err(_) => JsonRpcClientError::<T::Error>::JsonRpcError(error).into(),
                    })
                }
            }
        }

        Ok(results)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T> Provider for JsonRpcClient<T>
where
    T: 'static + JsonRpcTransport + Sync + Send,
{
    /// Returns the version of the Starknet JSON-RPC specification being used
    async fn spec_version(&self) -> Result<String, ProviderError> {
        self.send_request(JsonRpcMethod::SpecVersion, SpecVersionRequest)
            .await
    }

    /// Get block information with transaction hashes given the block id
    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
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
    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::GetBlockWithTxs,
            GetBlockWithTxsRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get block information with full transactions and receipts given the block id
    async fn get_block_with_receipts<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithReceipts, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::GetBlockWithReceipts,
            GetBlockWithReceiptsRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get the information about the result of executing the requested block
    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
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
    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FeltPrimitive, ProviderError>
    where
        A: AsRef<FeltPrimitive> + Send + Sync,
        K: AsRef<FeltPrimitive> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
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

    /// Gets the transaction status (possibly reflecting that the tx is still in
    /// the mempool, or dropped from it)
    async fn get_transaction_status<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionStatus, ProviderError>
    where
        H: AsRef<FeltPrimitive> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::GetTransactionStatus,
            GetTransactionStatusRequestRef {
                transaction_hash: transaction_hash.as_ref(),
            },
        )
        .await
    }

    /// Get the details and status of a submitted transaction
    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError>
    where
        H: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
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
    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionReceiptWithBlockInfo, ProviderError>
    where
        H: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FeltPrimitive, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
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
    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FeltPrimitive>, ProviderError>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
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
    async fn estimate_fee<R, S, B>(
        &self,
        request: R,
        simulation_flags: S,
        block_id: B,
    ) -> Result<Vec<FeeEstimate>, ProviderError>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlagForEstimateFee]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::EstimateFee,
            EstimateFeeRequestRef {
                request: request.as_ref(),
                simulation_flags: simulation_flags.as_ref(),
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Estimate the L2 fee of a message sent on L1
    async fn estimate_message_fee<M, B>(
        &self,
        message: M,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        M: AsRef<MsgFromL1> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::EstimateMessageFee,
            EstimateMessageFeeRequestRef {
                message: message.as_ref(),
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    /// Get the most recent accepted block number
    async fn block_number(&self) -> Result<u64, ProviderError> {
        self.send_request(JsonRpcMethod::BlockNumber, BlockNumberRequest)
            .await
    }

    /// Get the most recent accepted block hash and number
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError> {
        self.send_request(JsonRpcMethod::BlockHashAndNumber, BlockHashAndNumberRequest)
            .await
    }

    /// Return the currently configured Starknet chain id
    async fn chain_id(&self) -> Result<FeltPrimitive, ProviderError> {
        Ok(self
            .send_request::<_, Felt>(JsonRpcMethod::ChainId, ChainIdRequest)
            .await?
            .0)
    }

    /// Returns an object about the sync status, or false if the node is not synching
    async fn syncing(&self) -> Result<SyncStatusType, ProviderError> {
        self.send_request(JsonRpcMethod::Syncing, SyncingRequest)
            .await
    }

    /// Returns all events matching the given filter
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError> {
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
    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FeltPrimitive, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FeltPrimitive> + Send + Sync,
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
    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync,
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
    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::AddDeclareTransaction,
            AddDeclareTransactionRequestRef {
                declare_transaction: declare_transaction.as_ref(),
            },
        )
        .await
    }

    /// Submit a new deploy account transaction
    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::AddDeployAccountTransaction,
            AddDeployAccountTransactionRequestRef {
                deploy_account_transaction: deploy_account_transaction.as_ref(),
            },
        )
        .await
    }

    /// For a given executed transaction, return the trace of its execution, including internal
    /// calls
    async fn trace_transaction<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionTrace, ProviderError>
    where
        H: AsRef<FeltPrimitive> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::TraceTransaction,
            TraceTransactionRequestRef {
                transaction_hash: transaction_hash.as_ref(),
            },
        )
        .await
    }

    /// Simulate a given sequence of transactions on the requested state, and generate the execution
    /// traces. Note that some of the transactions may revert, in which case no error is thrown, but
    /// revert details can be seen on the returned trace object. . Note that some of the
    /// transactions may revert, this will be reflected by the revert_error property in the trace.
    /// Other types of failures (e.g. unexpected error or failure in the validation phase) will
    /// result in TRANSACTION_EXECUTION_ERROR.
    async fn simulate_transactions<B, TX, S>(
        &self,
        block_id: B,
        transactions: TX,
        simulation_flags: S,
    ) -> Result<Vec<SimulatedTransaction>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        TX: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlag]> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::SimulateTransactions,
            SimulateTransactionsRequestRef {
                block_id: block_id.as_ref(),
                transactions: transactions.as_ref(),
                simulation_flags: simulation_flags.as_ref(),
            },
        )
        .await
    }

    /// Retrieve traces for all transactions in the given block.
    async fn trace_block_transactions<B>(
        &self,
        block_id: B,
    ) -> Result<Vec<TransactionTraceWithHash>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
    {
        self.send_request(
            JsonRpcMethod::TraceBlockTransactions,
            TraceBlockTransactionsRequestRef {
                block_id: block_id.as_ref(),
            },
        )
        .await
    }

    async fn batch_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<ProviderResponseData>, ProviderError>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync,
    {
        self.send_requests(requests).await
    }
}

impl ProviderRequestData {
    const fn jsonrpc_method(&self) -> JsonRpcMethod {
        match self {
            Self::SpecVersion(_) => JsonRpcMethod::SpecVersion,
            Self::GetBlockWithTxHashes(_) => JsonRpcMethod::GetBlockWithTxHashes,
            Self::GetBlockWithTxs(_) => JsonRpcMethod::GetBlockWithTxs,
            Self::GetBlockWithReceipts(_) => JsonRpcMethod::GetBlockWithReceipts,
            Self::GetStateUpdate(_) => JsonRpcMethod::GetStateUpdate,
            Self::GetStorageAt(_) => JsonRpcMethod::GetStorageAt,
            Self::GetTransactionStatus(_) => JsonRpcMethod::GetTransactionStatus,
            Self::GetTransactionByHash(_) => JsonRpcMethod::GetTransactionByHash,
            Self::GetTransactionByBlockIdAndIndex(_) => {
                JsonRpcMethod::GetTransactionByBlockIdAndIndex
            }
            Self::GetTransactionReceipt(_) => JsonRpcMethod::GetTransactionReceipt,
            Self::GetClass(_) => JsonRpcMethod::GetClass,
            Self::GetClassHashAt(_) => JsonRpcMethod::GetClassHashAt,
            Self::GetClassAt(_) => JsonRpcMethod::GetClassAt,
            Self::GetBlockTransactionCount(_) => JsonRpcMethod::GetBlockTransactionCount,
            Self::Call(_) => JsonRpcMethod::Call,
            Self::EstimateFee(_) => JsonRpcMethod::EstimateFee,
            Self::EstimateMessageFee(_) => JsonRpcMethod::EstimateMessageFee,
            Self::BlockNumber(_) => JsonRpcMethod::BlockNumber,
            Self::BlockHashAndNumber(_) => JsonRpcMethod::BlockHashAndNumber,
            Self::ChainId(_) => JsonRpcMethod::ChainId,
            Self::Syncing(_) => JsonRpcMethod::Syncing,
            Self::GetEvents(_) => JsonRpcMethod::GetEvents,
            Self::GetNonce(_) => JsonRpcMethod::GetNonce,
            Self::AddInvokeTransaction(_) => JsonRpcMethod::AddInvokeTransaction,
            Self::AddDeclareTransaction(_) => JsonRpcMethod::AddDeclareTransaction,
            Self::AddDeployAccountTransaction(_) => JsonRpcMethod::AddDeployAccountTransaction,
            Self::TraceTransaction(_) => JsonRpcMethod::TraceTransaction,
            Self::SimulateTransactions(_) => JsonRpcMethod::SimulateTransactions,
            Self::TraceBlockTransactions(_) => JsonRpcMethod::TraceBlockTransactions,
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
            JsonRpcMethod::SpecVersion => ProviderRequestData::SpecVersion(
                serde_json::from_value::<SpecVersionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockWithTxHashes => ProviderRequestData::GetBlockWithTxHashes(
                serde_json::from_value::<GetBlockWithTxHashesRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockWithTxs => ProviderRequestData::GetBlockWithTxs(
                serde_json::from_value::<GetBlockWithTxsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockWithReceipts => ProviderRequestData::GetBlockWithReceipts(
                serde_json::from_value::<GetBlockWithReceiptsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetStateUpdate => ProviderRequestData::GetStateUpdate(
                serde_json::from_value::<GetStateUpdateRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetStorageAt => ProviderRequestData::GetStorageAt(
                serde_json::from_value::<GetStorageAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetTransactionStatus => ProviderRequestData::GetTransactionStatus(
                serde_json::from_value::<GetTransactionStatusRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetTransactionByHash => ProviderRequestData::GetTransactionByHash(
                serde_json::from_value::<GetTransactionByHashRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetTransactionByBlockIdAndIndex => {
                ProviderRequestData::GetTransactionByBlockIdAndIndex(
                    serde_json::from_value::<GetTransactionByBlockIdAndIndexRequest>(
                        raw_request.params,
                    )
                    .map_err(error_mapper)?,
                )
            }
            JsonRpcMethod::GetTransactionReceipt => ProviderRequestData::GetTransactionReceipt(
                serde_json::from_value::<GetTransactionReceiptRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClass => ProviderRequestData::GetClass(
                serde_json::from_value::<GetClassRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClassHashAt => ProviderRequestData::GetClassHashAt(
                serde_json::from_value::<GetClassHashAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetClassAt => ProviderRequestData::GetClassAt(
                serde_json::from_value::<GetClassAtRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetBlockTransactionCount => {
                ProviderRequestData::GetBlockTransactionCount(
                    serde_json::from_value::<GetBlockTransactionCountRequest>(raw_request.params)
                        .map_err(error_mapper)?,
                )
            }
            JsonRpcMethod::Call => ProviderRequestData::Call(
                serde_json::from_value::<CallRequest>(raw_request.params).map_err(error_mapper)?,
            ),
            JsonRpcMethod::EstimateFee => ProviderRequestData::EstimateFee(
                serde_json::from_value::<EstimateFeeRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::EstimateMessageFee => ProviderRequestData::EstimateMessageFee(
                serde_json::from_value::<EstimateMessageFeeRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::BlockNumber => ProviderRequestData::BlockNumber(
                serde_json::from_value::<BlockNumberRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::BlockHashAndNumber => ProviderRequestData::BlockHashAndNumber(
                serde_json::from_value::<BlockHashAndNumberRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::ChainId => ProviderRequestData::ChainId(
                serde_json::from_value::<ChainIdRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::Syncing => ProviderRequestData::Syncing(
                serde_json::from_value::<SyncingRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetEvents => ProviderRequestData::GetEvents(
                serde_json::from_value::<GetEventsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::GetNonce => ProviderRequestData::GetNonce(
                serde_json::from_value::<GetNonceRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddInvokeTransaction => ProviderRequestData::AddInvokeTransaction(
                serde_json::from_value::<AddInvokeTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddDeclareTransaction => ProviderRequestData::AddDeclareTransaction(
                serde_json::from_value::<AddDeclareTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::AddDeployAccountTransaction => {
                ProviderRequestData::AddDeployAccountTransaction(
                    serde_json::from_value::<AddDeployAccountTransactionRequest>(
                        raw_request.params,
                    )
                    .map_err(error_mapper)?,
                )
            }
            JsonRpcMethod::TraceTransaction => ProviderRequestData::TraceTransaction(
                serde_json::from_value::<TraceTransactionRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::SimulateTransactions => ProviderRequestData::SimulateTransactions(
                serde_json::from_value::<SimulateTransactionsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
            JsonRpcMethod::TraceBlockTransactions => ProviderRequestData::TraceBlockTransactions(
                serde_json::from_value::<TraceBlockTransactionsRequest>(raw_request.params)
                    .map_err(error_mapper)?,
            ),
        };

        Ok(Self {
            id: raw_request.id,
            data: request_data,
        })
    }
}

impl<T> ProviderImplError for JsonRpcClientError<T>
where
    T: 'static + Error + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T> From<JsonRpcClientError<T>> for ProviderError
where
    T: 'static + Error + Send + Sync,
{
    fn from(value: JsonRpcClientError<T>) -> Self {
        Self::Other(Box::new(value))
    }
}

impl<T> From<serde_json::Error> for JsonRpcClientError<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl TryFrom<&JsonRpcError> for StarknetError {
    type Error = JsonRpcErrorConversionError;

    fn try_from(value: &JsonRpcError) -> Result<Self, Self::Error> {
        match value.code {
            1 => Ok(Self::FailedToReceiveTransaction),
            20 => Ok(Self::ContractNotFound),
            24 => Ok(Self::BlockNotFound),
            27 => Ok(Self::InvalidTransactionIndex),
            28 => Ok(Self::ClassHashNotFound),
            29 => Ok(Self::TransactionHashNotFound),
            31 => Ok(Self::PageSizeTooBig),
            32 => Ok(Self::NoBlocks),
            33 => Ok(Self::InvalidContinuationToken),
            34 => Ok(Self::TooManyKeysInFilter),
            40 => {
                let data = ContractErrorData::deserialize(
                    value
                        .data
                        .as_ref()
                        .ok_or(JsonRpcErrorConversionError::MissingData)?,
                )
                .map_err(|_| JsonRpcErrorConversionError::DataParsingFailure)?;
                Ok(Self::ContractError(data))
            }
            41 => {
                let data = TransactionExecutionErrorData::deserialize(
                    value
                        .data
                        .as_ref()
                        .ok_or(JsonRpcErrorConversionError::MissingData)?,
                )
                .map_err(|_| JsonRpcErrorConversionError::DataParsingFailure)?;
                Ok(Self::TransactionExecutionError(data))
            }
            51 => Ok(Self::ClassAlreadyDeclared),
            52 => Ok(Self::InvalidTransactionNonce),
            53 => Ok(Self::InsufficientMaxFee),
            54 => Ok(Self::InsufficientAccountBalance),
            55 => {
                let data = String::deserialize(
                    value
                        .data
                        .as_ref()
                        .ok_or(JsonRpcErrorConversionError::MissingData)?,
                )
                .map_err(|_| JsonRpcErrorConversionError::DataParsingFailure)?;
                Ok(Self::ValidationFailure(data))
            }
            56 => Ok(Self::CompilationFailed),
            57 => Ok(Self::ContractClassSizeIsTooLarge),
            58 => Ok(Self::NonAccount),
            59 => Ok(Self::DuplicateTx),
            60 => Ok(Self::CompiledClassHashMismatch),
            61 => Ok(Self::UnsupportedTxVersion),
            62 => Ok(Self::UnsupportedContractClassVersion),
            63 => {
                let data = String::deserialize(
                    value
                        .data
                        .as_ref()
                        .ok_or(JsonRpcErrorConversionError::MissingData)?,
                )
                .map_err(|_| JsonRpcErrorConversionError::DataParsingFailure)?;
                Ok(Self::UnexpectedError(data))
            }
            10 => {
                let data = NoTraceAvailableErrorData::deserialize(
                    value
                        .data
                        .as_ref()
                        .ok_or(JsonRpcErrorConversionError::MissingData)?,
                )
                .map_err(|_| JsonRpcErrorConversionError::DataParsingFailure)?;
                Ok(Self::NoTraceAvailable(data))
            }
            _ => Err(JsonRpcErrorConversionError::UnknownCode),
        }
    }
}

impl Error for JsonRpcError {}

impl Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            Some(data) => {
                write!(
                    f,
                    "JSON-RPC error: code={}, message=\"{}\", data={}",
                    self.code,
                    self.message,
                    serde_json::to_string(data).map_err(|_| std::fmt::Error)?
                )
            }
            None => {
                write!(
                    f,
                    "JSON-RPC error: code={}, message=\"{}\"",
                    self.code, self.message
                )
            }
        }
    }
}
