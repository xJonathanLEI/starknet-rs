use crate::{Infallible, Signer, SigningKey, VerifyingKey};

use async_trait::async_trait;
use starknet_core::{
    crypto::{EcdsaSignError, Signature},
    types::FieldElement,
};

pub struct LocalWallet {
    private_key: SigningKey,
}

#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error(transparent)]
    EcdsaSignError(EcdsaSignError),
}

impl LocalWallet {
    pub fn from_signing_key(key: SigningKey) -> Self {
        key.into()
    }
}

#[async_trait]
impl Signer for LocalWallet {
    type GetPublicKeyError = Infallible;
    type SignError = SignError;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError> {
        Ok(self.private_key.verifying_key())
    }

    async fn sign_hash(&self, hash: &FieldElement) -> Result<Signature, Self::SignError> {
        Ok(self.private_key.sign(hash)?)
    }
}

impl From<SigningKey> for LocalWallet {
    fn from(value: SigningKey) -> Self {
        Self { private_key: value }
    }
}

impl From<EcdsaSignError> for SignError {
    fn from(value: EcdsaSignError) -> Self {
        Self::EcdsaSignError(value)
    }
}
