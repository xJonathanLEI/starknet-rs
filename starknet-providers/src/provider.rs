use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilter,
    EventsPage, FeeEstimate, FieldElement, FunctionCall, InvokeTransactionResult,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
    MaybePendingTransactionReceipt, MsgFromL1, SimulatedTransaction, SimulationFlag, StarknetError,
    SyncStatusType, Transaction, TransactionTrace, TransactionTraceWithHash,
};
use std::{any::Any, error::Error, fmt::Debug};

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Provider {
    /// Get block information with transaction hashes given the block id
    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get block information with full transactions given the block id
    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the information about the result of executing the requested block
    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the value of the storage at the given address and key
    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, ProviderError>
    where
        A: AsRef<FieldElement> + Send + Sync,
        K: AsRef<FieldElement> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Get the details and status of a submitted transaction
    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the details of a transaction by a given block id and index
    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the details of a transaction by a given block number and index
    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class definition in the given block associated with the given hash
    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class hash in the given block for the contract deployed at the given address
    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class definition in the given block at the given address
    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Get the number of transactions in a block given a block id
    async fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Call a starknet function without creating a Starknet transaction
    async fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FieldElement>, ProviderError>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Estimate the fee for a given Starknet transaction
    async fn estimate_fee<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<Vec<FeeEstimate>, ProviderError>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    async fn estimate_message_fee<M, B>(
        &self,
        message: M,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        M: AsRef<MsgFromL1> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Get the most recent accepted block number
    async fn block_number(&self) -> Result<u64, ProviderError>;

    /// Get the most recent accepted block hash and number
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, ProviderError>;

    /// Return the currently configured Starknet chain id
    async fn chain_id(&self) -> Result<FieldElement, ProviderError>;

    /// Returns the transactions in the transaction pool, recognized by this sequencer
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError>;

    /// Returns an object about the sync status, or false if the node is not synching
    async fn syncing(&self) -> Result<SyncStatusType, ProviderError>;

    /// Returns all events matching the given filter
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError>;

    /// Get the nonce associated with the given address in the given block
    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Submit a new transaction to be added to the chain
    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync;

    /// Submit a new transaction to be added to the chain
    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync;

    /// Submit a new deploy account transaction
    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync;

    /// For a given executed transaction, return the trace of its execution, including internal
    /// calls
    async fn trace_transaction<H>(
        &self,
        transaction_hash: H,
    ) -> Result<TransactionTrace, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Simulate a given sequence of transactions on the requested state, and generate the execution
    /// traces. If one of the transactions is reverted, raises CONTRACT_ERROR.
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

    /// Retrieve traces for all transactions in the given block.
    async fn trace_block_transactions<H>(
        &self,
        block_hash: H,
    ) -> Result<Vec<TransactionTraceWithHash>, ProviderError>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Same as [estimate_fee], but only with one estimate.
    async fn estimate_fee_single<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError>
    where
        R: AsRef<BroadcastedTransaction> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync,
    {
        let mut result = self
            .estimate_fee([request.as_ref().to_owned()], block_id)
            .await?;

        if result.len() == 1 {
            // Unwrapping here is safe becuase we already checked length
            Ok(result.pop().unwrap())
        } else {
            Err(ProviderError::ArrayLengthMismatch)
        }
    }

    /// Same as [simulate_transactions], but only with one simulation.
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
/// underlying transport. Therefore, it makes little sense to bloat [ProviderError] with a generic
/// parameter just for these errors. Instead, they're erased to this trait object.
///
/// This trait is used instead of a plain [std::error::Error] to allow downcasting, in case access
/// to the specific error type is indeed desired. This is achieved with the `as_any()` method.
pub trait ProviderImplError: Error + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error(transparent)]
    StarknetError(StarknetErrorWithMessage),
    #[error("Request rate limited")]
    RateLimited,
    #[error("Array length mismatch")]
    ArrayLengthMismatch,
    #[error("{0}")]
    Other(Box<dyn ProviderImplError>),
}

#[derive(Debug, thiserror::Error)]
#[error("code={code}, message=\"{message}\"")]
pub struct StarknetErrorWithMessage {
    pub code: MaybeUnknownErrorCode,
    pub message: String,
}

#[derive(Debug)]
pub enum MaybeUnknownErrorCode {
    Known(StarknetError),
    Unknown(i64),
}

impl core::fmt::Display for MaybeUnknownErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MaybeUnknownErrorCode::Known(code) => write!(f, "{}", code),
            MaybeUnknownErrorCode::Unknown(code) => write!(f, "{}", code),
        }
    }
}
