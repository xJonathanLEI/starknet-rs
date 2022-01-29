use crate::VerifyingKey;

use async_trait::async_trait;
use starknet_core::{crypto::Signature, types::UnsignedFieldElement};
use std::error::Error;

#[async_trait]
pub trait Signer {
    type GetPublicKeyError: Error;
    type SignError: Error;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    async fn sign_hash(&self, hash: &UnsignedFieldElement) -> Result<Signature, Self::SignError>;
}
