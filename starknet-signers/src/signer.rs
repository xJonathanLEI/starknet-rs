use crate::VerifyingKey;

use async_trait::async_trait;
use auto_impl::auto_impl;
use starknet_core::{
    crypto::Signature,
    types::{Call, Felt},
};
use std::error::Error;

/// Any signer that can provide a public key as [`Felt`], and sign a raw hash for a signature
/// encoded as [`Vec<Felt>`].
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait Signer {
    /// Possible errors for calling [`get_public_key`](fn.get_public_key).
    type GetPublicKeyError: Error + Send + Sync;
    /// Possible errors for calling [`sign`](fn.sign).
    type SignError: Error + Send + Sync;

    /// Retrieves the verifying (public) key from the signer.
    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError>;

    /// Requests an ECDSA signature for a message hash.
    ///
    /// Signing a raw hash is known as "blind signing". For interactive signers (e.g. hardware
    /// wallets) that can theoretically provide better security properties via "clear signing",
    /// using blind signing is bad practice.
    ///
    /// However, as of this writing, no actual interactive signer implementation offers clear
    /// signing. When this changes in the future, this trait shall be altered to allow such clear
    /// signing capabilities.
    async fn sign_hash(&self, hash: &Felt) -> Result<Signature, Self::SignError>;

    /// Whether the underlying signer implementation is interactive, such as a hardware wallet.
    /// Implementations should return `true` if the signing operation is very expensive, even if not
    /// strictly "interactive" as in requiring human input.
    ///
    /// This mainly affects the transaction simulation strategy used by higher-level types. With
    /// non-interactive signers, it's fine to sign multiple times for getting the most accurate
    /// estimation/simulation possible; but with interactive signers, they would accept less
    /// accurate results to minimize signing requests.
    fn is_interactive(&self, context: SignerInteractivityContext<'_>) -> bool;
}

/// Context for helping signer implementations make decisions on whether to act interactively or
/// not, useful for signers with dynamic interactivity.
///
/// This type only exposes execution details as context, with everything else falling under the
/// `Other` variant, as it's deemed very much pointless to act differently in those scenarios.
/// When an execution is requested, only the list of calls is exposed.
#[derive(Debug, Clone, Copy)]
pub enum SignerInteractivityContext<'a> {
    /// An execution is being requested.
    Execution {
        /// The list of calls being authorized.
        calls: &'a [Call],
    },
    /// A class declaration or account deployment is being requested.
    Other,
}
