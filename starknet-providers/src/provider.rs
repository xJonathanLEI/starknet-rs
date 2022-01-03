use async_trait::async_trait;
use starknet_core::types::Block;
use std::error::Error;

#[async_trait]
pub trait Provider {
    type Error: Error;

    async fn get_block(&self) -> Result<Block, Self::Error>;
}
