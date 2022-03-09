use crate::Account;

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        AddTransactionResult, BlockId, FieldElement, InvokeFunction,
        InvokeFunctionTransactionRequest, TransactionRequest,
    },
    utils::get_selector_from_name,
};
use starknet_providers::Provider;
use starknet_signers::Signer;

pub struct SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    provider: P,
    #[allow(unused)]
    signer: S,
    address: FieldElement,
}

#[derive(Debug, thiserror::Error)]
pub enum GetNonceError<P>
where
    P: std::error::Error + Send,
{
    #[error(transparent)]
    ProviderError(P),
    #[error("invalid response length. expected {expected} but got {actual}")]
    InvalidResponseLength { expected: usize, actual: usize },
}

#[derive(Debug, thiserror::Error)]
pub enum ExecuteError<P, S>
where
    P: std::error::Error + Send,
    S: std::error::Error + Send,
{
    #[error(transparent)]
    ProviderError(P),
    #[error(transparent)]
    SignerError(S),
}

impl<P, S> SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    pub fn new(provider: P, signer: S, address: FieldElement) -> Self {
        Self {
            provider,
            signer,
            address,
        }
    }
}

#[async_trait(?Send)]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type GetNonceError = GetNonceError<P::Error>;
    type ExecuteError = ExecuteError<P::Error, S::SignError>;

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError> {
        let call_result = self
            .provider
            .call_contract(
                InvokeFunction {
                    contract_address: self.address,
                    entry_point_selector: get_selector_from_name("get_nonce").unwrap(),
                    calldata: vec![],
                    signature: vec![],
                },
                block_identifier,
            )
            .await
            .map_err(GetNonceError::ProviderError)?;

        if call_result.result.len() == 1 {
            Ok(call_result.result[0])
        } else {
            Err(GetNonceError::InvalidResponseLength {
                expected: 1,
                actual: call_result.result.len(),
            })
        }
    }

    async fn execute(
        &self,
        to: FieldElement,
        selector: FieldElement,
        calldata: &[FieldElement],
        nonce: FieldElement,
    ) -> Result<AddTransactionResult, Self::ExecuteError> {
        let message_hash = compute_hash_on_elements(&[
            self.address,
            to,
            selector,
            compute_hash_on_elements(calldata),
            nonce,
        ]);
        let signature = self
            .signer
            .sign_hash(&message_hash)
            .await
            .map_err(ExecuteError::SignerError)?;

        let mut execute_calldata = vec![to, selector, calldata.len().into()];
        for item in calldata.iter() {
            execute_calldata.push(*item);
        }
        execute_calldata.push(nonce);

        self.provider
            .add_transaction(
                TransactionRequest::InvokeFunction(InvokeFunctionTransactionRequest {
                    contract_address: self.address,
                    entry_point_selector: get_selector_from_name("execute").unwrap(),
                    calldata: execute_calldata,
                    signature: vec![signature.r, signature.s],
                }),
                None,
            )
            .await
            .map_err(ExecuteError::ProviderError)
    }
}
