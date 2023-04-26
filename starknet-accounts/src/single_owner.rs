use crate::{Account, ConnectedAccount, RawDeclaration, RawExecution, RawLegacyDeclaration};

use async_trait::async_trait;
use starknet_core::types::{contract::ComputeClassHashError, BlockId, FieldElement};
use starknet_providers::Provider;
use starknet_signers::Signer;

#[derive(Debug, Clone)]
pub struct SingleOwnerAccount<P, S>
where
    P: Provider + Send,
    S: Signer + Send,
{
    provider: P,
    signer: S,
    address: FieldElement,
    chain_id: FieldElement,
    block_id: BlockId,
}

#[derive(Debug, thiserror::Error)]
pub enum SignError<S> {
    #[error(transparent)]
    Signer(S),
    #[error(transparent)]
    ClassHash(ComputeClassHashError),
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
            block_id: BlockId::Latest,
        }
    }

    pub fn set_block_id(&mut self, block_id: BlockId) -> &Self {
        self.block_id = block_id;
        self
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<P, S> Account for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type SignError = SignError<S::SignError>;

    fn address(&self) -> FieldElement {
        self.address
    }

    fn chain_id(&self) -> FieldElement {
        self.chain_id
    }

    async fn sign_execution(
        &self,
        execution: &RawExecution,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = execution.transaction_hash(self.chain_id, self.address);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    async fn sign_declaration(
        &self,
        declaration: &RawDeclaration,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = declaration.transaction_hash(self.chain_id, self.address);
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }

    async fn sign_legacy_declaration(
        &self,
        legacy_declaration: &RawLegacyDeclaration,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash = legacy_declaration
            .transaction_hash(self.chain_id, self.address)
            .map_err(SignError::ClassHash)?;
        let signature = self
            .signer
            .sign_hash(&tx_hash)
            .await
            .map_err(SignError::Signer)?;

        Ok(vec![signature.r, signature.s])
    }
}

impl<P, S> ConnectedAccount for SingleOwnerAccount<P, S>
where
    P: Provider + Sync + Send,
    S: Signer + Sync + Send,
{
    type Provider = P;

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn block_id(&self) -> BlockId {
        self.block_id
    }
}
