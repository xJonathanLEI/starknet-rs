use async_trait::async_trait;
use starknet_core::types::{Block, BlockId};
use std::error::Error;

#[async_trait]
pub trait Provider {
    type Error: Error;

    async fn get_block(&self, block_hash_or_number: Option<BlockId>) -> Result<Block, Self::Error>;
}
