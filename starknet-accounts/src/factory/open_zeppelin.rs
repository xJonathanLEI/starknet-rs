use crate::{AccountFactory, PreparedAccountDeployment, RawAccountDeployment};

use async_trait::async_trait;
use starknet_core::types::FieldElement;
use starknet_providers::Provider;
use starknet_signers::Signer;

pub struct OpenZeppelinAccountFactory<S, P> {
    class_hash: FieldElement,
    chain_id: FieldElement,
    public_key: FieldElement,
    signer: S,
    provider: P,
}

impl<S, P> OpenZeppelinAccountFactory<S, P>
where
    S: Signer,
{
    pub async fn new(
        class_hash: FieldElement,
        chain_id: FieldElement,
        signer: S,
        provider: P,
    ) -> Result<Self, S::GetPublicKeyError> {
        let public_key = signer.get_public_key().await?;
        Ok(Self {
            class_hash,
            chain_id,
            public_key: public_key.scalar(),
            signer,
            provider,
        })
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<S, P> AccountFactory for OpenZeppelinAccountFactory<S, P>
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
        vec![self.public_key]
    }

    fn chain_id(&self) -> FieldElement {
        self.chain_id
    }

    fn provider(&self) -> &Self::Provider {
        &self.provider
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
