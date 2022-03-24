use crate::VerifyingKey;

use async_trait::async_trait;
use starknet_core::{crypto::Signature, types::FieldElement};
use std::error::Error;

#[async_trait]
pub trait Signer {
    type GetPublicKeyError: Error + Send;
    type SignError: Error + Send;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    async fn sign_hash(&self, hash: &FieldElement) -> Result<Signature, Self::SignError>;
}
