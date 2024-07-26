#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

#[allow(unused_extern_crates)]
#[cfg(all(not(feature = "std"), any(test, feature = "alloc")))]
extern crate alloc;

mod ecdsa;
mod error;
mod fe_utils;
mod pedersen_hash;
mod poseidon_hash;
mod rfc6979;

#[cfg(test)]
mod test_utils;

pub use starknet_types_core::felt::Felt;

pub use pedersen_hash::pedersen_hash;

pub use poseidon_hash::{poseidon_hash, poseidon_hash_many, poseidon_hash_single, PoseidonHasher};

pub use ecdsa::{get_public_key, recover, sign, verify, ExtendedSignature, Signature};

pub use crate::rfc6979::generate_k as rfc6979_generate_k;

pub use error::{RecoverError, SignError, VerifyError};
