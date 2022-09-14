use crate::{
    account::{AccountCall, AttachedAccountCall},
    Account, Call,
};

use async_trait::async_trait;
use starknet_core::{
    crypto::compute_hash_on_elements,
    types::{
        AccountTransaction, AddTransactionResult, BlockId, FeeEstimate, FieldElement,
        InvokeFunctionTransactionRequest, TransactionRequest,
    },
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

#[derive(Debug, Clone)]
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
pub enum TransactionError<P, S> {
    #[error(transparent)]
    ProviderError(P),
    #[error(transparent)]
    SignerError(S),
}

impl<P, S> SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    pub fn new(provider: P, signer: S, address: FieldElement, chain_id: FieldElement) -> Self {
        Self {
            provider,
            signer,
            address,
            chain_id,
        }
    }

    pub fn provider(&self) -> &P {
        &self.provider
    }

    pub fn signer(&self) -> &S {
        &self.signer
    }

    pub fn address(&self) -> FieldElement {
        self.address
    }

    pub fn chain_id(&self) -> FieldElement {
        self.chain_id
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

        let transaction_hash = compute_hash_on_elements(&[
            PREFIX_INVOKE,
            FieldElement::ONE, // version
            self.address,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&execute_calldata),
            max_fee,
            self.chain_id,
            nonce,
        ]);
        let signature = self.signer.sign_hash(&transaction_hash).await?;

        Ok(InvokeFunctionTransactionRequest {
            contract_address: self.address,
            calldata: execute_calldata,
            signature: vec![signature.r, signature.s],
            max_fee,
            nonce,
        })
    }

    async fn get_nonce_for_call<C>(&self, call: &C) -> Result<FieldElement, P::Error>
    where
        C: AccountCall,
    {
        match call.get_nonce() {
            Some(value) => Ok(value.to_owned()),
            None => self.get_nonce(BlockId::Latest).await,
        }
    }

    async fn estimate_fee_for_calls(
        &self,
        calls: &[Call],
        nonce: Option<&FieldElement>,
    ) -> Result<FeeEstimate, TransactionError<P::Error, S::SignError>> {
        let nonce = match nonce {
            Some(value) => value.to_owned(),
            None => self
                .get_nonce(BlockId::Latest)
                .await
                .map_err(TransactionError::ProviderError)?,
        };
        let estimate_fee_request = self
            .generate_invoke_request(calls, nonce, FieldElement::ZERO)
            .await
            .map_err(TransactionError::SignerError)?;
        self.provider
            .estimate_fee(
                AccountTransaction::InvokeFunction(estimate_fee_request),
                BlockId::Latest,
            )
            .await
            .map_err(TransactionError::ProviderError)
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type GetNonceError = P::Error;
    type EstimateFeeError = TransactionError<P::Error, S::SignError>;
    type SendTransactionError = TransactionError<P::Error, S::SignError>;

    fn address(&self) -> FieldElement {
        self.address
    }

    async fn get_nonce(
        &self,
        block_identifier: BlockId,
    ) -> Result<FieldElement, Self::GetNonceError> {
        self.provider
            .get_nonce(self.address, block_identifier)
            .await
    }

    fn execute(&self, calls: &[Call]) -> AttachedAccountCall<Self> {
        AttachedAccountCall::<Self> {
            calls: calls.to_vec(),
            nonce: None,
            max_fee: None,
            account: self,
        }
    }

    async fn estimate_fee<C>(&self, call: &C) -> Result<FeeEstimate, Self::EstimateFeeError>
    where
        C: AccountCall + Sync,
    {
        self.estimate_fee_for_calls(call.get_calls(), call.get_nonce().as_ref())
            .await
    }

    async fn send_transaction<C>(
        &self,
        call: &C,
    ) -> Result<AddTransactionResult, Self::SendTransactionError>
    where
        C: AccountCall + Sync,
    {
        let nonce = self
            .get_nonce_for_call(call)
            .await
            .map_err(Self::SendTransactionError::ProviderError)?;
        let max_fee = match call.get_max_fee() {
            Some(value) => value.to_owned(),
            None => {
                let fee_estimate = self
                    .estimate_fee_for_calls(call.get_calls(), Some(&nonce))
                    .await?;

                // Adds 10% fee buffer
                (fee_estimate.overall_fee * 11 / 10).into()
            }
        };

        let add_transaction_request = self
            .generate_invoke_request(call.get_calls(), nonce, max_fee)
            .await
            .map_err(Self::SendTransactionError::SignerError)?;
        self.provider
            .add_transaction(
                TransactionRequest::InvokeFunction(add_transaction_request),
                None,
            )
            .await
            .map_err(Self::SendTransactionError::ProviderError)
    }
}
