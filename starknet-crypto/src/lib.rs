#![cfg_attr(any(target_arch = "wasm32", not(feature = "std")), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
include!("./without_std.rs");

/// Feature gate some code that should only be run when `std` feature is enabled.
///
/// # Example
///
/// ```
/// use sp_std::if_std;
///
/// if_std! {
///     // This code is only being compiled and executed when the `std` feature is enabled.
///     println!("Hello native world");
/// }
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! if_std {
	( $( $code:tt )* ) => {
		$( $code )*
	}
}

#[cfg(all(not(feature = "std"), feature = "alloc"))]
#[macro_export]
macro_rules! if_std {
    ( $( $code:tt )* ) => {};
}
mod ecdsa;
mod error;
mod fe_utils;
mod pedersen_hash;
mod pedersen_points;
mod rfc6979;

#[cfg(test)]
mod test_utils;

pub use starknet_ff::FieldElement;

pub use pedersen_hash::pedersen_hash;

pub use ecdsa::{get_public_key, sign, verify, Signature};

pub use crate::rfc6979::generate_k as rfc6979_generate_k;

pub use error::{SignError, VerifyError};
