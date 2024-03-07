use crate::VerifyingKey;

use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::crypto::Signature;
use starknet_types_core::felt::Felt;
use std::error::Error;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Signer {
    type GetPublicKeyError: Error + Send + Sync;
    type SignError: Error + Send + Sync;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    async fn sign_hash(&self, hash: &Felt) -> Result<Signature, Self::SignError>;
}
