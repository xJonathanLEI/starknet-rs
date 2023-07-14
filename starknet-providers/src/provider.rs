use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilter,
    EventsPage, FeeEstimate, FieldElement, FunctionCall, InvokeTransactionResult,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingStateUpdate,
    MaybePendingTransactionReceipt, StarknetError, SyncStatusType, Transaction,
};
use std::error::Error;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Provider {
    type Error: Error + Send + Sync;

    /// Get block information with transaction hashes given the block id
    async fn get_block_with_tx_hashes<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxHashes, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get block information with full transactions given the block id
    async fn get_block_with_txs<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingBlockWithTxs, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the information about the result of executing the requested block
    async fn get_state_update<B>(
        &self,
        block_id: B,
    ) -> Result<MaybePendingStateUpdate, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the value of the storage at the given address and key
    async fn get_storage_at<A, K, B>(
        &self,
        contract_address: A,
        key: K,
        block_id: B,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        A: AsRef<FieldElement> + Send + Sync,
        K: AsRef<FieldElement> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Get the details and status of a submitted transaction
    async fn get_transaction_by_hash<H>(
        &self,
        transaction_hash: H,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the details of a transaction by a given block id and index
    async fn get_transaction_by_block_id_and_index<B>(
        &self,
        block_id: B,
        index: u64,
    ) -> Result<Transaction, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Get the details of a transaction by a given block number and index
    async fn get_transaction_receipt<H>(
        &self,
        transaction_hash: H,
    ) -> Result<MaybePendingTransactionReceipt, ProviderError<Self::Error>>
    where
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class definition in the given block associated with the given hash
    async fn get_class<B, H>(
        &self,
        block_id: B,
        class_hash: H,
    ) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        H: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class hash in the given block for the contract deployed at the given address
    async fn get_class_hash_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Get the contract class definition in the given block at the given address
    async fn get_class_at<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<ContractClass, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Get the number of transactions in a block given a block id
    async fn get_block_transaction_count<B>(
        &self,
        block_id: B,
    ) -> Result<u64, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync;

    /// Call a starknet function without creating a Starknet transaction
    async fn call<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<Vec<FieldElement>, ProviderError<Self::Error>>
    where
        R: AsRef<FunctionCall> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Estimate the fee for a given Starknet transaction
    async fn estimate_fee<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>>
    where
        R: AsRef<[BroadcastedTransaction]> + Send + Sync,
        B: AsRef<BlockId> + Send + Sync;

    /// Get the most recent accepted block number
    async fn block_number(&self) -> Result<u64, ProviderError<Self::Error>>;

    /// Get the most recent accepted block hash and number
    async fn block_hash_and_number(&self)
        -> Result<BlockHashAndNumber, ProviderError<Self::Error>>;

    /// Return the currently configured Starknet chain id
    async fn chain_id(&self) -> Result<FieldElement, ProviderError<Self::Error>>;

    /// Returns the transactions in the transaction pool, recognized by this sequencer
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, ProviderError<Self::Error>>;

    /// Returns an object about the sync status, or false if the node is not synching
    async fn syncing(&self) -> Result<SyncStatusType, ProviderError<Self::Error>>;

    /// Returns all events matching the given filter
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, ProviderError<Self::Error>>;

    /// Get the nonce associated with the given address in the given block
    async fn get_nonce<B, A>(
        &self,
        block_id: B,
        contract_address: A,
    ) -> Result<FieldElement, ProviderError<Self::Error>>
    where
        B: AsRef<BlockId> + Send + Sync,
        A: AsRef<FieldElement> + Send + Sync;

    /// Submit a new transaction to be added to the chain
    async fn add_invoke_transaction<I>(
        &self,
        invoke_transaction: I,
    ) -> Result<InvokeTransactionResult, ProviderError<Self::Error>>
    where
        I: AsRef<BroadcastedInvokeTransaction> + Send + Sync;

    /// Submit a new transaction to be added to the chain
    async fn add_declare_transaction<D>(
        &self,
        declare_transaction: D,
    ) -> Result<DeclareTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeclareTransaction> + Send + Sync;

    /// Submit a new deploy account transaction
    async fn add_deploy_account_transaction<D>(
        &self,
        deploy_account_transaction: D,
    ) -> Result<DeployAccountTransactionResult, ProviderError<Self::Error>>
    where
        D: AsRef<BroadcastedDeployAccountTransaction> + Send + Sync;

    /// Same as [estimate_fee], but only with one estimate.
    async fn estimate_fee_single<R, B>(
        &self,
        request: R,
        block_id: B,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>
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
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError<E> {
    #[error(transparent)]
    StarknetError(StarknetErrorWithMessage),
    #[error("Request rate limited")]
    RateLimited,
    #[error("Array length mismatch")]
    ArrayLengthMismatch,
    #[error(transparent)]
    Other(E),
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
