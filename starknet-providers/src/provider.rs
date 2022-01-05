use async_trait::async_trait;
use starknet_core::types::{
    Block, BlockId, ContractCode, TransactionId, TransactionStatus, H256, U256,
};
use std::error::Error;

#[async_trait]
pub trait Provider {
    type Error: Error;

    async fn get_block(&self, block_hash_or_number: Option<BlockId>) -> Result<Block, Self::Error>;

    async fn get_code(
        &self,
        contract_address: H256,
        block_hash_or_number: Option<BlockId>,
    ) -> Result<ContractCode, Self::Error>;

    async fn get_storage_at(
        &self,
        contract_address: H256,
        key: U256,
        block_hash_or_number: Option<BlockId>,
    ) -> Result<U256, Self::Error>;

    async fn get_transaction_status(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionStatus, Self::Error>;
}
