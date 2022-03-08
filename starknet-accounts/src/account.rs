use crate::Call;

use async_trait::async_trait;
use starknet_core::types::{AddTransactionResult, BlockId, FieldElement};
use std::error::Error;

#[async_trait]
pub trait Account {
    type GetNonceError: Error + Send;
    type ExecuteError: Error + Send;

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError>;

    async fn execute(
        &self,
        calls: &[Call],
        nonce: FieldElement,
    ) -> Result<AddTransactionResult, Self::ExecuteError>;
}
