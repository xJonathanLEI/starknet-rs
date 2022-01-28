use async_trait::async_trait;
use starknet_core::{crypto::Signature, types::UnsignedFieldElement};
use std::error::Error;

#[async_trait]
pub trait Signer {
    type GetAddressError: Error;
    type SignError: Error;

    async fn get_address(&self) -> Result<UnsignedFieldElement, Self::GetAddressError>;

    async fn sign_hash(&self, hash: &UnsignedFieldElement) -> Result<Signature, Self::SignError>;
}
