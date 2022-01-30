use async_trait::async_trait;
use starknet_core::types::{BlockId, UnsignedFieldElement};
use std::error::Error;

#[async_trait]
pub trait Account {
    type Error: Error;

    async fn get_nonce(
        &self,
        block_identifier: Option<BlockId>,
    ) -> Result<UnsignedFieldElement, Self::Error>;
}
