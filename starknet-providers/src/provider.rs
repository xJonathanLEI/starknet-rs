use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::types::{
    contract::{legacy::LegacyContractCode, CompiledClass, DeployedClass},
    AccountTransaction, AddTransactionResult, Block, BlockId, BlockTraces, CallContractResult,
    CallFunction, CallL1Handler, ContractAddresses, FeeEstimate, FieldElement, StarknetError,
    StateUpdate, TransactionInfo, TransactionReceipt, TransactionRequest,
    TransactionSimulationInfo, TransactionStatusInfo, TransactionTrace,
};
use std::error::Error;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Provider {
    type Error: Error + Send + Sync;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<AddTransactionResult, ProviderError<Self::Error>>;

    async fn get_contract_addresses(&self)
        -> Result<ContractAddresses, ProviderError<Self::Error>>;

    async fn call_contract(
        &self,
        call_function: CallFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, ProviderError<Self::Error>>;

    async fn estimate_fee(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>;

    async fn estimate_fee_bulk(
        &self,
        txs: &[AccountTransaction],
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<Vec<FeeEstimate>, ProviderError<Self::Error>>;

    async fn estimate_message_fee(
        &self,
        call_l1_handler: CallL1Handler,
        block_identifier: BlockId,
    ) -> Result<FeeEstimate, ProviderError<Self::Error>>;

    async fn simulate_transaction(
        &self,
        tx: AccountTransaction,
        block_identifier: BlockId,
        skip_validate: bool,
    ) -> Result<TransactionSimulationInfo, ProviderError<Self::Error>>;

    async fn get_block(
        &self,
        block_identifier: BlockId,
    ) -> Result<Block, ProviderError<Self::Error>>;

    async fn get_block_traces(
        &self,
        block_identifier: BlockId,
    ) -> Result<BlockTraces, ProviderError<Self::Error>>;

    async fn get_state_update(
        &self,
        block_identifier: BlockId,
    ) -> Result<StateUpdate, ProviderError<Self::Error>>;

    async fn get_code(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<LegacyContractCode, ProviderError<Self::Error>>;

    async fn get_full_contract(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>>;

    async fn get_compiled_class_by_class_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<CompiledClass, ProviderError<Self::Error>>;

    async fn get_class_hash_at(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>>;

    async fn get_class_by_hash(
        &self,
        class_hash: FieldElement,
        block_identifier: BlockId,
    ) -> Result<DeployedClass, ProviderError<Self::Error>>;

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>>;

    async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, ProviderError<Self::Error>>;

    async fn get_transaction_status(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionStatusInfo, ProviderError<Self::Error>>;

    async fn get_transaction(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionInfo, ProviderError<Self::Error>>;

    async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionReceipt, ProviderError<Self::Error>>;

    async fn get_transaction_trace(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<TransactionTrace, ProviderError<Self::Error>>;

    async fn get_block_hash_by_id(
        &self,
        block_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>>;

    async fn get_block_id_by_hash(
        &self,
        block_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>>;

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<FieldElement, ProviderError<Self::Error>>;

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<u64, ProviderError<Self::Error>>;

    async fn get_last_batch_id(&self) -> Result<u64, ProviderError<Self::Error>>;

    async fn get_l1_blockchain_id(&self) -> Result<u64, ProviderError<Self::Error>>;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError<E> {
    #[error(transparent)]
    StarknetError(StarknetError),
    #[error("Request rate limited")]
    RateLimited,
    #[error(transparent)]
    Other(E),
}

#[cfg(feature = "blocking")]
pub use blocking::BlockingProvider;

#[cfg(feature = "blocking")]
mod blocking {
    use starknet_core::types::FieldElement;

    use crate::jsonrpc::models::*;

    /// A non-async provider trait. Note that this trait is currently also used for experimenting with
    /// a JSONRPC-centric provider trait. So the functions are different from [Provider].
    // #[auto_impl(&, Box, Arc)]
    pub trait BlockingProvider {
        type Error: std::error::Error;

        /// Get block information with transaction hashes given the block id
        fn get_block_with_tx_hashes<B>(
            &self,
            block_id: B,
        ) -> Result<MaybePendingBlockWithTxHashes, Self::Error>
        where
            B: AsRef<BlockId>;

        /// Get block information with full transactions given the block id
        fn get_block_with_txs<B>(
            &self,
            block_id: B,
        ) -> Result<MaybePendingBlockWithTxs, Self::Error>
        where
            B: AsRef<BlockId>;

        /// Get the information about the result of executing the requested block
        fn get_state_update<B>(&self, block_id: B) -> Result<StateUpdate, Self::Error>
        where
            B: AsRef<BlockId>;

        /// Get the value of the storage at the given address and key
        fn get_storage_at<A, K, B>(
            &self,
            contract_address: A,
            key: K,
            block_id: B,
        ) -> Result<FieldElement, Self::Error>
        where
            A: AsRef<FieldElement>,
            K: AsRef<FieldElement>,
            B: AsRef<BlockId>;

        /// Get the details and status of a submitted transaction
        fn get_transaction_by_hash<H>(
            &self,
            transaction_hash: H,
        ) -> Result<Transaction, Self::Error>
        where
            H: AsRef<FieldElement>;

        /// Get the details of a transaction by a given block id and index
        fn get_transaction_by_block_id_and_index<B>(
            &self,
            block_id: B,
            index: u64,
        ) -> Result<Transaction, Self::Error>
        where
            B: AsRef<BlockId>;

        /// Get the details of a transaction by a given block number and index
        fn get_transaction_receipt<H>(
            &self,
            transaction_hash: H,
        ) -> Result<MaybePendingTransactionReceipt, Self::Error>
        where
            H: AsRef<FieldElement>;

        /// Get the contract class definition in the given block associated with the given hash
        fn get_class<B, H>(&self, block_id: B, class_hash: H) -> Result<ContractClass, Self::Error>
        where
            B: AsRef<BlockId>,
            H: AsRef<FieldElement>;

        /// Get the contract class hash in the given block for the contract deployed at the given address
        fn get_class_hash_at<B, A>(
            &self,
            block_id: B,
            contract_address: A,
        ) -> Result<FieldElement, Self::Error>
        where
            B: AsRef<BlockId>,
            A: AsRef<FieldElement>;

        /// Get the contract class definition in the given block at the given address
        fn get_class_at<B, A>(
            &self,
            block_id: B,
            contract_address: A,
        ) -> Result<ContractClass, Self::Error>
        where
            B: AsRef<BlockId>,
            A: AsRef<FieldElement>;

        /// Get the number of transactions in a block given a block id
        fn get_block_transaction_count<B>(&self, block_id: B) -> Result<u64, Self::Error>
        where
            B: AsRef<BlockId>;

        /// Call a starknet function without creating a Starknet transaction
        fn call<R, B>(&self, request: R, block_id: B) -> Result<Vec<FieldElement>, Self::Error>
        where
            R: AsRef<FunctionCall>,
            B: AsRef<BlockId>;

        /// Estimate the fee for a given Starknet transaction
        fn estimate_fee<R, B>(&self, request: R, block_id: B) -> Result<FeeEstimate, Self::Error>
        where
            R: AsRef<BroadcastedTransaction>,
            B: AsRef<BlockId>;

        /// Get the most recent accepted block number
        fn block_number(&self) -> Result<u64, Self::Error>;

        /// Get the most recent accepted block hash and number
        fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, Self::Error>;

        /// Return the currently configured Starknet chain id
        fn chain_id(&self) -> Result<FieldElement, Self::Error>;

        /// Returns the transactions in the transaction pool, recognized by this sequencer
        fn pending_transactions(&self) -> Result<Vec<Transaction>, Self::Error>;

        /// Returns an object about the sync status, or false if the node is not synching
        fn syncing(&self) -> Result<SyncStatusType, Self::Error>;

        /// Returns all events matching the given filter
        fn get_events(
            &self,
            filter: EventFilter,
            continuation_token: Option<String>,
            chunk_size: u64,
        ) -> Result<EventsPage, Self::Error>;

        /// Get the nonce associated with the given address in the given block
        fn get_nonce<B, A>(
            &self,
            block_id: B,
            contract_address: A,
        ) -> Result<FieldElement, Self::Error>
        where
            B: AsRef<BlockId>,
            A: AsRef<FieldElement>;

        /// Submit a new transaction to be added to the chain
        fn add_invoke_transaction<I>(
            &self,
            invoke_transaction: I,
        ) -> Result<InvokeTransactionResult, Self::Error>
        where
            I: AsRef<BroadcastedInvokeTransaction>;

        /// Submit a new transaction to be added to the chain
        fn add_declare_transaction<D>(
            &self,
            declare_transaction: D,
        ) -> Result<DeclareTransactionResult, Self::Error>
        where
            D: AsRef<BroadcastedDeclareTransaction>;

        /// Submit a new deploy contract transaction
        fn add_deploy_transaction<D>(
            &self,
            deploy_transaction: D,
        ) -> Result<DeployTransactionResult, Self::Error>
        where
            D: AsRef<BroadcastedDeployTransaction>;

        /// Submit a new deploy account transaction
        fn add_deploy_account_transaction<D>(
            &self,
            deploy_account_transaction: D,
        ) -> Result<DeployAccountTransactionResult, Self::Error>
        where
            D: AsRef<BroadcastedDeployAccountTransaction>;
    }
}
