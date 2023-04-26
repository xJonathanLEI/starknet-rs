use crate::VerifyingKey;

use async_trait::async_trait;
use starknet_core::{crypto::Signature, types::FieldElement};
use std::error::Error;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Signer {
    type GetPublicKeyError: Error + Send + Sync;
    type SignError: Error + Send + Sync;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    async fn sign_hash(&self, hash: &FieldElement) -> Result<Signature, Self::SignError>;
}
