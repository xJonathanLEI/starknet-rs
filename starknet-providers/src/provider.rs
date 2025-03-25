use async_trait::async_trait;
use auto_impl::auto_impl;
use serde::Serialize;
use starknet_core::types::{
    requests::*, BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ConfirmedBlockId, ContractClass, ContractStorageKeys, DeclareTransactionResult,
    DeployAccountTransactionResult, EventFilter, EventsPage, FeeEstimate, Felt, FunctionCall,
    Hash256, InvokeTransactionResult, MaybePendingBlockWithReceipts, MaybePendingBlockWithTxHashes,
    MaybePendingBlockWithTxs, MaybePendingStateUpdate, MessageWithStatus, MsgFromL1,
    SimulatedTransaction, SimulationFlag, SimulationFlagForEstimateFee, StarknetError,
    StorageProof, SyncStatusType, Transaction, TransactionReceiptWithBlockInfo, TransactionStatus,
    TransactionTrace, TransactionTraceWithHash,
};
use std::{any::Any, error::Error, fmt::Debug};

/// A generic interface for any type allowing communication with a Starknet network.
///
/// Historically, the only official way to access the network is through the sequencer gateway,
/// implemented by [`SequencerGatewayProvider`](crate::sequencer::SequencerGatewayProvider), which
/// has since been deprecated. Currently, the recommended way of accessing the network is via the
/// JSON-RPC specification, implemented with [`JsonRpcClient`](crate::jsonrpc::JsonRpcClient).
///
/// The legacy [`SequencerGatewayProvider`](crate::sequencer::SequencerGatewayProvider) still
/// implements this trait for backward compatibility reasons, but most of its methods no longer work
/// in practice, as public sequencer servers have generally block access to most methods.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Provider {
    /// Returns the version of the Starknet JSON-RPC specification being used.
    async fn spec_version(&self) -> Result<String, ProviderError>;

    /// Gets block information with transaction hashes given the block id.
    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Gets block information with full transactions given the block id.
    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Gets block information with full transactions and receipts given the block id.
    async fn get_block_with_receipts<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithReceipts, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Gets the information about the result of executing the requested block.
    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Gets the value of the storage at the given address and key.
    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<Felt, ProviderError>
    where
        A: AsRef<Felt> + Send + Sync,
        K: AsRef<Felt> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Given an l1 tx hash, returns the associated l1_handler tx hashes and statuses for all L1 ->
    /// L2 messages sent by the l1 transaction, ordered by the l1 tx sending order
    async fn get_messages_status(
        &self,
        transaction_hash: Hash256,
    ) -> Result<Vec<MessageWithStatus>, ProviderError>;

    /// Gets the transaction status (possibly reflecting that the tx is still in the mempool, or
    /// dropped from it).
    async fn get_transaction_status<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionStatus, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync;

    /// Gets the details and status of a submitted transaction.
    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync;

    /// Gets the details of a transaction by a given block id and index.
    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Gets the details of a transaction by a given block number and index.
    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionReceiptWithBlockInfo, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync;

    /// Gets the contract class definition in the given block associated with the given hash.
    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<Felt> + Send + Sync;

    /// Gets the contract class hash in the given block for the contract deployed at the given
    /// address.
    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<Felt, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync;

    /// Gets the contract class definition in the given block at the given address.
    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync;

    /// Gets the number of transactions in a block given a block id.
    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Calls a starknet function without creating a Starknet transaction.
    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<Felt>, ProviderError>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Estimates the fee for a given Starknet transaction.
    async fn estimate_fee<R, S, B>(
        &self,
        request: R,
        simulation_flags: S,
        block_id: B,
    ) -> Result<Vec<FeeEstimate>, ProviderError>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlagForEstimateFee]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Estimates the fee for sending an L1-to-L2 message.
    async fn estimate_message_fee<M, B>(
        &self,
        message: M,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        M: AsRef<MsgFromL1> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Gets the most recent accepted block number.
    async fn block_number(&self) -> Result<u64, ProviderError>;

    /// Gets the most recent accepted block hash and number.
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError>;

    /// Returns the currently configured Starknet chain id.
    async fn chain_id(&self) -> Result<Felt, ProviderError>;

    /// Returns an object about the sync status, or false if the node is not synching.
    async fn syncing(&self) -> Result<SyncStatusType, ProviderError>;

    /// Returns all events matching the given filter.
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError>;

    /// Gets the nonce associated with the given address in the given block.
    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<Felt, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<Felt> + Send + Sync;

    /// Get merkle paths in one of the state tries: global state, classes, individual contract.
    /// A single request can query for any mix of the three types of storage proofs (classes,
    /// contracts, and storage).
    async fn get_storage_proof<B, H, A, K>(
        &self,
        block_id: B,
        class_hashes: H,
        contract_addresses: A,
        contracts_storage_keys: K,
    ) -> Result<StorageProof, ProviderError>
    where
        B: AsRef<ConfirmedBlockId> + Send + Sync,
        H: AsRef<[Felt]> + Send + Sync,
        A: AsRef<[Felt]> + Send + Sync,
        K: AsRef<[ContractStorageKeys]> + Send + Sync;

    /// Submits a new transaction to be added to the chain.
    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync;

    /// Submits a new transaction to be added to the chain.
    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync;

    /// Submits a new deploy account transaction.
    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync;

    /// For a given executed transaction, returns the trace of its execution, including internal
    /// calls.
    async fn trace_transaction<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionTrace, ProviderError>
    where
        H: AsRef<Felt> + Send + Sync;

    /// Simulates a given sequence of transactions on the requested state, and generate the
    /// execution traces. Note that some of the transactions may revert, in which case no error is
    /// thrown, but revert details can be seen on the returned trace object.
    ///
    /// Note that some of the transactions may revert, this will be reflected by the `revert_error`
    /// property in the trace. Other types of failures (e.g. unexpected error or failure in the
    /// validation phase) will result in `TRANSACTION_EXECUTION_ERROR`.
    async fn simulate_transactions<B, T, S>(
        &self,
        block_id: B,
        transactions: T,
        simulation_flags: S,
    ) -> Result<Vec<SimulatedTransaction>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        T: AsRef<[BroadcastedTransaction]> + Send + Sync,
        S: AsRef<[SimulationFlag]> + Send + Sync;

    /// Retrieves traces for all transactions in the given block.
    async fn trace_block_transactions<B>(
        &self,
        block_id: B,
    ) -> Result<Vec<TransactionTraceWithHash>, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Sends multiple requests in parallel. The function call fails if any of the requests fails.
    /// Implementations must guarantee that responses follow the exact order as the requests.
    async fn batch_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<ProviderResponseData>, ProviderError>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync;

    /// Same as [`estimate_fee`](fn.estimate_fee), but only with one estimate.
    async fn estimate_fee_single<R, S, B>(
        &self,
        request: R,
        simulation_flags: S,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        R: AsRef<BroadcastedTransaction> + Send + Sync,
        S: AsRef<[SimulationFlagForEstimateFee]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let mut result = self
            .estimate_fee([request.as_ref().to_owned()], simulation_flags, block_id)
            .await?;

        if result.len() == 1 {
            // Unwrapping here is safe becuase we already checked length
            Ok(result.pop().unwrap())
        } else {
            Err(ProviderError::ArrayLengthMismatch)
        }
    }

    /// Same as [`simulate_transactions`](fn.simulate_transactions), but only with one simulation.
    async fn simulate_transaction<B, T, S>(
        &self,
        block_id: B,
        transaction: T,
        simulation_flags: S,
    ) -> Result<SimulatedTransaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        T: AsRef<BroadcastedTransaction> + Send + Sync,
        S: AsRef<[SimulationFlag]> + Send + Sync,
    {
        let mut result = self
            .simulate_transactions(
                block_id,
                [transaction.as_ref().to_owned()],
                simulation_flags,
            )
            .await?;

        if result.len() == 1 {
            // Unwrapping here is safe becuase we already checked length
            Ok(result.pop().unwrap())
        } else {
            Err(ProviderError::ArrayLengthMismatch)
        }
    }
}

/// Trait for implementation-specific error type. These errors are irrelevant in most cases,
/// assuming that users typically care more about the specifics of RPC errors instead of the
/// underlying transport. Therefore, it makes little sense to bloat [`ProviderError`] with a generic
/// parameter just for these errors. Instead, they're erased to this trait object.
///
/// This trait is used instead of a plain [`std::error::Error`] to allow downcasting, in case access
/// to the specific error type is indeed desired. This is achieved with the `as_any()` method.
pub trait ProviderImplError: Error + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

/// Errors using any [`Provider`] implementation. This type is deliberately not made generic such
/// that:
///
/// - the [`Provider`] trait itself can be boxed;
/// - error handling is easier.
///
/// As a downside, the [`Other`](ProviderError::Other) variant contains a boxed implementation-
/// specific error. It's generally expected that users of [`Provider`] would not need to care about
/// these errors, but in the case where they do, it's slightly harder to access than if generics are
/// used instead.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    /// A Starknet-related error, usually regarding the state or transaction.
    #[error(transparent)]
    StarknetError(StarknetError),
    /// The request fails as the client is rate-limited.
    #[error("Request rate limited")]
    RateLimited,
    /// When estimating fees for or simulating a single transaction, the server unexpectedly returns
    /// data for zero or more than one transactions.
    #[error("Array length mismatch")]
    ArrayLengthMismatch,
    /// Boxed implementation-specific errors.
    #[error("{0}")]
    Other(Box<dyn ProviderImplError>),
}

/// Typed request data for [`Provider`] requests.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProviderRequestData {
    /// Request data for `starknet_specVersion`.
    SpecVersion(SpecVersionRequest),
    /// Request data for `starknet_getBlockWithTxHashes`.
    GetBlockWithTxHashes(GetBlockWithTxHashesRequest),
    /// Request data for `starknet_getBlockWithTxs`.
    GetBlockWithTxs(GetBlockWithTxsRequest),
    /// Request data for `starknet_getBlockWithReceipts`.
    GetBlockWithReceipts(GetBlockWithReceiptsRequest),
    /// Request data for `starknet_getStateUpdate`.
    GetStateUpdate(GetStateUpdateRequest),
    /// Request data for `starknet_getStorageAt`.
    GetStorageAt(GetStorageAtRequest),
    /// Request data for `starknet_getMessagesStatus`.
    GetMessagesStatus(GetMessagesStatusRequest),
    /// Request data for `starknet_getTransactionStatus`.
    GetTransactionStatus(GetTransactionStatusRequest),
    /// Request data for `starknet_getTransactionByHash`.
    GetTransactionByHash(GetTransactionByHashRequest),
    /// Request data for `starknet_getTransactionByBlockIdAndIndex`.
    GetTransactionByBlockIdAndIndex(GetTransactionByBlockIdAndIndexRequest),
    /// Request data for `starknet_getTransactionReceipt`.
    GetTransactionReceipt(GetTransactionReceiptRequest),
    /// Request data for `starknet_getClass`.
    GetClass(GetClassRequest),
    /// Request data for `starknet_getClassHashAt`.
    GetClassHashAt(GetClassHashAtRequest),
    /// Request data for `starknet_getClassAt`.
    GetClassAt(GetClassAtRequest),
    /// Request data for `starknet_getBlockTransactionCount`.
    GetBlockTransactionCount(GetBlockTransactionCountRequest),
    /// Request data for `starknet_call`.
    Call(CallRequest),
    /// Request data for `starknet_estimateFee`.
    EstimateFee(EstimateFeeRequest),
    /// Request data for `starknet_estimateMessageFee`.
    EstimateMessageFee(EstimateMessageFeeRequest),
    /// Request data for `starknet_blockNumber`.
    BlockNumber(BlockNumberRequest),
    /// Request data for `starknet_blockHashAndNumber`.
    BlockHashAndNumber(BlockHashAndNumberRequest),
    /// Request data for `starknet_chainId`.
    ChainId(ChainIdRequest),
    /// Request data for `starknet_syncing`.
    Syncing(SyncingRequest),
    /// Request data for `starknet_getEvents`.
    GetEvents(GetEventsRequest),
    /// Request data for `starknet_getNonce`.
    GetNonce(GetNonceRequest),
    /// Request data for `starknet_getStorageProof`.
    GetStorageProof(GetStorageProofRequest),
    /// Request data for `starknet_addInvokeTransaction`.
    AddInvokeTransaction(AddInvokeTransactionRequest),
    /// Request data for `starknet_addDeclareTransaction`.
    AddDeclareTransaction(AddDeclareTransactionRequest),
    /// Request data for `starknet_addDeployAccountTransaction`.
    AddDeployAccountTransaction(AddDeployAccountTransactionRequest),
    /// Request data for `starknet_traceTransaction`.
    TraceTransaction(TraceTransactionRequest),
    /// Request data for `starknet_simulateTransactions`.
    SimulateTransactions(SimulateTransactionsRequest),
    /// Request data for `starknet_traceBlockTransactions`.
    TraceBlockTransactions(TraceBlockTransactionsRequest),
}

/// Typed response data for [`Provider`] responses.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum ProviderResponseData {
    /// Response data for `starknet_specVersion`.
    SpecVersion(String),
    /// Response data for `starknet_getBlockWithTxHashes`.
    GetBlockWithTxHashes(MaybePendingBlockWithTxHashes),
    /// Response data for `starknet_getBlockWithTxs`.
    GetBlockWithTxs(MaybePendingBlockWithTxs),
    /// Response data for `starknet_getBlockWithReceipts`.
    GetBlockWithReceipts(MaybePendingBlockWithReceipts),
    /// Response data for `starknet_getStateUpdate`.
    GetStateUpdate(MaybePendingStateUpdate),
    /// Response data for `starknet_getStorageAt`.
    GetStorageAt(Felt),
    /// Response data for `starknet_getMessagesStatus`.
    GetMessagesStatus(Vec<MessageWithStatus>),
    /// Response data for `starknet_getTransactionStatus`.
    GetTransactionStatus(TransactionStatus),
    /// Response data for `starknet_getTransactionByHash`.
    GetTransactionByHash(Transaction),
    /// Response data for `starknet_getTransactionByBlockIdAndIndex`.
    GetTransactionByBlockIdAndIndex(Transaction),
    /// Response data for `starknet_getTransactionReceipt`.
    GetTransactionReceipt(TransactionReceiptWithBlockInfo),
    /// Response data for `starknet_getClass`.
    GetClass(ContractClass),
    /// Response data for `starknet_getClassHashAt`.
    GetClassHashAt(Felt),
    /// Response data for `starknet_getClassAt`.
    GetClassAt(ContractClass),
    /// Response data for `starknet_getBlockTransactionCount`.
    GetBlockTransactionCount(u64),
    /// Response data for `starknet_call`.
    Call(Vec<Felt>),
    /// Response data for `starknet_estimateFee`.
    EstimateFee(Vec<FeeEstimate>),
    /// Response data for `starknet_estimateMessageFee`.
    EstimateMessageFee(FeeEstimate),
    /// Response data for `starknet_blockNumber`.
    BlockNumber(u64),
    /// Response data for `starknet_blockHashAndNumber`.
    BlockHashAndNumber(BlockHashAndNumber),
    /// Response data for `starknet_chainId`.
    ChainId(Felt),
    /// Response data for `starknet_syncing`.
    Syncing(SyncStatusType),
    /// Response data for `starknet_getEvents`.
    GetEvents(EventsPage),
    /// Response data for `starknet_getNonce`.
    GetNonce(Felt),
    /// Response data for `starknet_getStorageProof`.
    GetStorageProof(StorageProof),
    /// Response data for `starknet_addInvokeTransaction`.
    AddInvokeTransaction(InvokeTransactionResult),
    /// Response data for `starknet_addDeclareTransaction`.
    AddDeclareTransaction(DeclareTransactionResult),
    /// Response data for `starknet_addDeployAccountTransaction`.
    AddDeployAccountTransaction(DeployAccountTransactionResult),
    /// Response data for `starknet_traceTransaction`.
    TraceTransaction(TransactionTrace),
    /// Response data for `starknet_simulateTransactions`.
    SimulateTransactions(Vec<SimulatedTransaction>),
    /// Response data for `starknet_traceBlockTransactions`.
    TraceBlockTransactions(Vec<TransactionTraceWithHash>),
}
