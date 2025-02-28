use crate::{AccountFactory, PreparedAccountDeploymentV3, RawAccountDeploymentV3};

use async_trait::async_trait;
use starknet_core::{
    codec::Encode,
    types::{BlockId, BlockTag, Felt},
};
use starknet_providers::Provider;
use starknet_signers::{Signer, SignerInteractivityContext};

/// [`AccountFactory`] implementation for deploying `Argent X` account contracts (v0.4.0).
#[derive(Debug)]
pub struct ArgentAccountFactory<S, P> {
    class_hash: Felt,
    chain_id: Felt,
    owner_public_key: Felt,
    guardian_public_key: Option<Felt>,
    signer: S,
    provider: P,
    block_id: BlockId,
}

/// Constructor parameters for Argent account v0.4.0.
#[derive(Encode)]
#[starknet(core = "starknet_core")]
struct ArgentAccountConstructorParams {
    owner: ArgentSigner,
    guardian: Option<ArgentSigner>,
}

/// A simplified version of `argent::signer::signer_signature::Signer` that only supports the simple
/// Starknet signer.
#[derive(Encode)]
#[starknet(core = "starknet_core")]
enum ArgentSigner {
    Starknet(Felt),
}

impl<S, P> ArgentAccountFactory<S, P>
where
    S: Signer,
{
    /// Constructs a new [`ArgentAccountFactory`].
    pub async fn new(
        class_hash: Felt,
        chain_id: Felt,
        guardian_public_key: Option<Felt>,
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

    /// Sets a new block ID to run queries against.
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

    fn class_hash(&self) -> Felt {
        self.class_hash
    }

    fn calldata(&self) -> Vec<Felt> {
        let mut calldata = vec![];

        // Encoding this sturct never fails
        ArgentAccountConstructorParams {
            owner: ArgentSigner::Starknet(self.owner_public_key),
            guardian: self.guardian_public_key.map(ArgentSigner::Starknet),
        }
        .encode(&mut calldata)
        .unwrap();

        calldata
    }

    fn chain_id(&self) -> Felt {
        self.chain_id
    }

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn is_signer_interactive(&self) -> bool {
        self.signer
            .is_interactive(SignerInteractivityContext::Other)
    }

    fn block_id(&self) -> BlockId {
        self.block_id
    }

    async fn sign_deployment_v3(
        &self,
        deployment: &RawAccountDeploymentV3,
        query_only: bool,
    ) -> Result<Vec<Felt>, Self::SignError> {
        let tx_hash = PreparedAccountDeploymentV3::from_raw(deployment.clone(), self)
            .transaction_hash(query_only);
        let signature = self.signer.sign_hash(&tx_hash).await?;

        Ok(vec![signature.r, signature.s])
    }
}
