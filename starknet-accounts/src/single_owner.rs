use crate::Account;

use async_trait::async_trait;
use starknet_core::{
    types::{BlockId, InvokeFunction, UnsignedFieldElement},
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
    address: UnsignedFieldElement,
}

#[derive(Debug, thiserror::Error)]
pub enum Error<P>
where
    P: std::error::Error + Send,
{
    #[error(transparent)]
    ProviderError(P),
    #[error("invalid response length. expected {expected} but got {actual}")]
    InvalidResponseLength { expected: usize, actual: usize },
}

impl<P, S> SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    pub fn new(provider: P, signer: S, address: UnsignedFieldElement) -> Self {
        Self {
            provider,
            signer,
            address,
        }
    }
}

#[async_trait]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type Error = Error<P::Error>;

    async fn get_nonce(
        &self,
        block_identifier: Option<BlockId>,
    ) -> Result<UnsignedFieldElement, Self::Error> {
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
            .map_err(Error::ProviderError)?;

        if call_result.result.len() == 1 {
            Ok(call_result.result[0])
        } else {
            Err(Error::InvalidResponseLength {
                expected: 1,
                actual: call_result.result.len(),
            })
        }
    }
}
