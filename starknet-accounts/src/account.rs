use crate::Call;

use async_trait::async_trait;
use starknet_core::types::{AddTransactionResult, BlockId, FieldElement, ContractDefinition};
use std::error::Error;

#[async_trait(?Send)]
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

    async fn deploy_account(
        &self,
        constructor_calldata: Vec<FieldElement>,
        contract_definition: ContractDefinition,
        contract_address_salt: FieldElement,
    ) -> Result<AddTransactionResult, Self::ExecuteError>;
}
