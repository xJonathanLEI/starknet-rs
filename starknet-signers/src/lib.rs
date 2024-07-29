//! Starknet signer interface and common implementations.

#![deny(missing_docs)]

mod key_pair;
pub use key_pair::{SigningKey, VerifyingKey};

#[cfg(not(target_arch = "wasm32"))]
pub use key_pair::KeystoreError;

mod signer;
pub use signer::Signer;

/// Module containing types related to the use of a simple in-memory signer.
pub mod local_wallet;
pub use local_wallet::LocalWallet;

/// Module containing types related to the Ledger hardware wallet.
#[cfg(feature = "ledger")]
pub mod ledger;
#[cfg(feature = "ledger")]
pub use ledger::{DerivationPath, LedgerError, LedgerSigner};

/// An error type that indicates an error cannot possibly occur. Used as placeholder where
/// [`Result`] is expected.
#[derive(Debug, thiserror::Error)]
pub enum Infallible {}
