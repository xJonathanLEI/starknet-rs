use async_trait::async_trait;
use starknet_core::types::{AddTransactionResult, BlockId, UnsignedFieldElement};
use std::error::Error;

#[async_trait]
pub trait Account {
    type GetNonceError: Error + Send;
    type ExecuteError: Error + Send;

    async fn get_nonce(
        &self,
        block_identifier: Option<BlockId>,
    ) -> Result<UnsignedFieldElement, Self::GetNonceError>;

    async fn execute(
        &self,
        to: UnsignedFieldElement,
        selector: UnsignedFieldElement,
        calldata: &[UnsignedFieldElement],
        nonce: UnsignedFieldElement,
    ) -> Result<AddTransactionResult, Self::ExecuteError>;
}
