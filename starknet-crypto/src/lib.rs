#![doc = include_str!("../README.md")]

mod ec_point;
mod ecdsa;
mod error;
mod field_element;
mod pedersen_hash;
mod pedersen_params;
mod rfc6979;

#[cfg(test)]
mod test_utils;

pub use field_element::{FieldElement, FieldElementRepr};

pub use pedersen_hash::pedersen_hash;

pub use ecdsa::{get_public_key, sign, verify};

pub use crate::rfc6979::generate_k as rfc6979_generate_k;

pub use error::{SignError, VerifyError};
