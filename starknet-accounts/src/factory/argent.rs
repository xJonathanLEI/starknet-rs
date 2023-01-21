use crate::{AccountFactory, PreparedAccountDeployment, RawAccountDeployment};

use async_trait::async_trait;
use starknet_core::types::FieldElement;
use starknet_providers::Provider;
use starknet_signers::Signer;

/// Selector for "initialize"
const SELECTOR_INITIALIZE: FieldElement = FieldElement::from_mont([
    14382173896205878522,
    7380089477680411368,
    4404362358337226556,
    132905214994424316,
]);

pub struct ArgentAccountFactory<S, P> {
    proxy_class_hash: FieldElement,
    impl_class_hash: FieldElement,
    chain_id: FieldElement,
    signer_public_key: FieldElement,
    guardian_public_key: FieldElement,
    signer: S,
    provider: P,
}

impl<S, P> ArgentAccountFactory<S, P>
where
    S: Signer,
{
    pub async fn new(
        proxy_class_hash: FieldElement,
        impl_class_hash: FieldElement,
        chain_id: FieldElement,
        guardian_public_key: FieldElement,
        signer: S,
        provider: P,
    ) -> Result<Self, S::GetPublicKeyError> {
        let signer_public_key = signer.get_public_key().await?;
        Ok(Self {
            proxy_class_hash,
            impl_class_hash,
            chain_id,
            signer_public_key: signer_public_key.scalar(),
            guardian_public_key,
            signer,
            provider,
        })
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
        self.proxy_class_hash
    }

    fn calldata(&self) -> Vec<FieldElement> {
        vec![
            self.impl_class_hash,
            SELECTOR_INITIALIZE,
            FieldElement::TWO,
            self.signer_public_key,
            self.guardian_public_key,
        ]
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
