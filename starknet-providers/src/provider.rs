use async_trait::async_trait;
use starknet_core::types::{
    AddTransactionResult, Block, BlockId, CallContractResult, ContractAddresses, ContractArtifact,
    ContractCode, FieldElement, InvokeFunction, StateUpdate, TransactionId, TransactionInfo,
    TransactionReceipt, TransactionRequest, TransactionStatusInfo,
};
use std::error::Error;

#[async_trait]
pub trait Provider {
    type Error: Error + Send;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
        token: Option<String>,
    ) -> Result<AddTransactionResult, Self::Error>;

    async fn get_contract_addresses(&self) -> Result<ContractAddresses, Self::Error>;

    async fn call_contract(
        &self,
        invoke_tx: InvokeFunction,
        block_identifier: BlockId,
    ) -> Result<CallContractResult, Self::Error>;

    async fn get_block(&self, block_identifier: BlockId) -> Result<Block, Self::Error>;

    async fn get_state_update(&self, block_identifier: BlockId)
        -> Result<StateUpdate, Self::Error>;

    async fn get_code(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<ContractCode, Self::Error>;

    async fn get_full_contract(
        &self,
        contract_address: FieldElement,
        block_identifier: BlockId,
    ) -> Result<ContractArtifact, Self::Error>;

    async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::Error>;

    async fn get_transaction_status(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionStatusInfo, Self::Error>;

    async fn get_transaction(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionInfo, Self::Error>;

    async fn get_transaction_receipt(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionReceipt, Self::Error>;

    async fn get_block_hash_by_id(&self, block_number: u64) -> Result<FieldElement, Self::Error>;

    async fn get_block_id_by_hash(&self, block_hash: FieldElement) -> Result<u64, Self::Error>;

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<FieldElement, Self::Error>;

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<u64, Self::Error>;

    async fn get_last_batch_id(&self) -> Result<u64, Self::Error>;
}
