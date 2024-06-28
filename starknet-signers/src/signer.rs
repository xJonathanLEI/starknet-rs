use crate::VerifyingKey;

use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::{crypto::Signature, types::Felt};
use std::error::Error;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Signer {
    type GetPublicKeyError: Error + Send + Sync;
    type SignError: Error + Send + Sync;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    async fn sign_hash(&self, hash: &Felt) -> Result<Signature, Self::SignError>;

    /// Whether the underlying signer implementation is interactive, such as a hardware wallet.
    /// Implementations should return `true` if the signing operation is very expensive, even if not
    /// strictly "interactive" as in requiring human input.
    ///
    /// This mainly affects the transaction simulation strategy used by higher-level types. With
    /// non-interactive signers, it's fine to sign multiple times for getting the most accurate
    /// estimation/simulation possible; but with interactive signers, they would accept less
    /// accurate results to minimize signing requests.
    fn is_interactive(&self) -> bool;
}
