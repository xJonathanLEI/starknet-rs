use async_trait::async_trait;
use starknet_core::types::{Block, BlockId, ContractCode, H256, U256};
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
}
