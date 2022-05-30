#![doc = include_str ! ("../README.md")]

pub use ecdsa::{get_public_key, sign, verify, Signature};
pub use error::{SignError, VerifyError};
pub use pedersen_hash::pedersen_hash;
pub use starknet_ff::FieldElement;

pub use crate::rfc6979::generate_k as rfc6979_generate_k;

mod ec_point;
mod ecdsa;
mod error;
mod fe_utils;
mod pedersen_hash;
mod pedersen_params;
mod rfc6979;

#[cfg(test)]
mod test_utils;
