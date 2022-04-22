use crate::{
    account::{AccountCall, AttachedAccountCall},
    Account, Call,
};

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        AddTransactionResult, BlockId, FieldElement, InvokeFunctionTransactionRequest,
        TransactionRequest,
    },
    utils::get_selector_from_name,
};
use starknet_providers::Provider;
use starknet_signers::Signer;

/// Cairo string for "invoke"
const PREFIX_INVOKE: FieldElement = FieldElement::from_mont([
    18443034532770911073,
    18446744073709551615,
    18446744073709551615,
    513398556346534256,
]);

/// Selector for "__execute__"
const SELECTOR_EXECUTE: FieldElement = FieldElement::from_mont([
    12003533864240545316,
    425026474450283495,
    15935222606396478900,
    305947032915839070,
]);

pub struct SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    provider: P,
    #[allow(unused)]
    signer: S,
    address: FieldElement,
    chain_id: FieldElement,
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
pub enum SendTransactionError<P, S>
where
    P: std::error::Error + Send,
    S: std::error::Error + Send,
{
    #[error(transparent)]
    GetNonceError(GetNonceError<P>),
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
    pub fn new(provider: P, signer: S, address: FieldElement, chain_id: FieldElement) -> Self {
        Self {
            provider,
            signer,
            address,
            chain_id,
        }
    }

    async fn generate_invoke_request(
        &self,
        calls: &[Call],
        nonce: FieldElement,
        max_fee: FieldElement,
    ) -> Result<InvokeFunctionTransactionRequest, S::SignError> {
        let mut concated_calldata: Vec<FieldElement> = vec![];
        let mut execute_calldata: Vec<FieldElement> = vec![calls.len().into()];
        for call in calls.iter() {
            execute_calldata.push(call.to); // to
            execute_calldata.push(call.selector); // selector
            execute_calldata.push(concated_calldata.len().into()); // data_offset
            execute_calldata.push(call.calldata.len().into()); // data_len

            for item in call.calldata.iter() {
                concated_calldata.push(*item);
            }
        }
        execute_calldata.push(concated_calldata.len().into()); // calldata_len
        for item in concated_calldata.into_iter() {
            execute_calldata.push(item); // calldata
        }
        execute_calldata.push(nonce); // nonce

        let transaction_hash = compute_hash_on_elements(&[
            PREFIX_INVOKE,
            FieldElement::ZERO, // version
            self.address,
            SELECTOR_EXECUTE,
            compute_hash_on_elements(&execute_calldata),
            max_fee,
            self.chain_id,
        ]);
        let signature = self.signer.sign_hash(&transaction_hash).await?;

        Ok(InvokeFunctionTransactionRequest {
            contract_address: self.address,
            entry_point_selector: SELECTOR_EXECUTE,
            calldata: execute_calldata,
            signature: vec![signature.r, signature.s],
            max_fee,
        })
    }
}

#[async_trait(?Send)]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type GetNonceError = GetNonceError<P::Error>;
    type SendTransactionError = SendTransactionError<P::Error, S::SignError>;

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError> {
        let call_result = self
            .provider
            .call_contract(
                InvokeFunctionTransactionRequest {
                    contract_address: self.address,
                    entry_point_selector: get_selector_from_name("get_nonce").unwrap(),
                    calldata: vec![],
                    signature: vec![],
                    max_fee: FieldElement::ZERO,
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

    fn execute(&self, calls: &[Call]) -> AttachedAccountCall<Self> {
        AttachedAccountCall::<Self> {
            calls: calls.to_vec(),
            nonce: None,
            max_fee: None,
            account: self,
        }
    }

    async fn send_transaction<C>(
        &self,
        call: &C,
    ) -> Result<AddTransactionResult, Self::SendTransactionError>
    where
        C: AccountCall,
    {
        let nonce = match call.get_nonce() {
            Some(value) => value.to_owned(),
            None => match self.get_nonce(BlockId::Latest).await {
                Ok(value) => value,
                Err(err) => return Err(SendTransactionError::GetNonceError(err)),
            },
        };

        let max_fee = match call.get_max_fee() {
            Some(value) => value.to_owned(),
            None => {
                let estimate_fee_request = self
                    .generate_invoke_request(call.get_calls(), nonce, FieldElement::ZERO)
                    .await
                    .map_err(SendTransactionError::SignerError)?;
                let fee_estimate = self
                    .provider
                    .estimate_fee(estimate_fee_request, BlockId::Latest)
                    .await
                    .map_err(SendTransactionError::ProviderError)?;

                // Adds 10% fee buffer
                (fee_estimate.amount * 11 / 10).into()
            }
        };

        let add_transaction_request = self
            .generate_invoke_request(call.get_calls(), nonce, max_fee)
            .await
            .map_err(SendTransactionError::SignerError)?;
        self.provider
            .add_transaction(
                TransactionRequest::InvokeFunction(add_transaction_request),
                None,
            )
            .await
            .map_err(SendTransactionError::ProviderError)
    }
}
