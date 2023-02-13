#![no_std]

extern crate no_std_compat as std;

mod key_pair;
pub use key_pair::{SigningKey, VerifyingKey};

mod signer;
pub use signer::Signer;

pub mod local_wallet;
pub use local_wallet::LocalWallet;

#[derive(Debug, thiserror::Error)]
pub enum Infallible {}
