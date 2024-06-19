mod key_pair;
pub use key_pair::{SigningKey, VerifyingKey};

#[cfg(not(target_arch = "wasm32"))]
pub use key_pair::KeystoreError;

mod signer;
pub use signer::Signer;

pub mod local_wallet;
pub use local_wallet::LocalWallet;

#[cfg(feature = "ledger")]
pub mod ledger;
#[cfg(feature = "ledger")]
pub use ledger::{DerivationPath, LedgerError, LedgerSigner};

#[derive(Debug, thiserror::Error)]
pub enum Infallible {}
