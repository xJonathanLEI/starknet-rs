use crate::{AccountFactory, PreparedAccountDeployment, RawAccountDeployment};
use async_trait::async_trait;
use starknet_core::types::{BlockId, BlockTag, FieldElement};
use starknet_providers::Provider;
use starknet_signers::Signer;

pub struct ArgentAccountFactory<S, P> {
    class_hash: FieldElement,
    chain_id: FieldElement,
    owner_public_key: FieldElement,
    guardian_public_key: FieldElement,
    signer: S,
    provider: P,
    block_id: BlockId,
}

impl<S, P> ArgentAccountFactory<S, P>
where
    S: Signer,
{
    pub async fn new(
        class_hash: FieldElement,
        chain_id: FieldElement,
        guardian_public_key: FieldElement,
        signer: S,
        provider: P,
    ) -> Result<Self, S::GetPublicKeyError> {
        let signer_public_key = signer.get_public_key().await?;
        Ok(Self {
            class_hash,
            chain_id,
            owner_public_key: signer_public_key.scalar(),
            guardian_public_key,
            signer,
            provider,
            block_id: BlockId::Tag(BlockTag::Latest),
        })
    }

    pub fn set_block_id(&mut self, block_id: BlockId) -> &Self {
        self.block_id = block_id;
        self
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<S, P> AccountFactory for ArgentAccountFactory<S, P>
where
    S: Signer + Sync + Send,
    P: Provider + Sync + Send,
{
    type Provider = P;
    type SignError = S::SignError;

    fn class_hash(&self) -> FieldElement {
        self.class_hash
    }

    fn calldata(&self) -> Vec<FieldElement> {
        vec![self.owner_public_key, self.guardian_public_key]
    }

    fn chain_id(&self) -> FieldElement {
        self.chain_id
    }

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn block_id(&self) -> BlockId {
        self.block_id
    }

    async fn sign_deployment(
        &self,
        deployment: &RawAccountDeployment,
    ) -> Result<Vec<FieldElement>, Self::SignError> {
        let tx_hash =
            PreparedAccountDeployment::from_raw(deployment.clone(), self).transaction_hash();
        let signature = self.signer.sign_hash(&tx_hash).await?;

        Ok(vec![signature.r, signature.s])
    }
}
