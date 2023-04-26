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
