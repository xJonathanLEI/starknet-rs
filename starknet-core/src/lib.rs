#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(not(feature = "std"))]
include!("./without_std.rs");

pub mod stdlib {
    #[cfg(feature = "std")]
    pub use crate::with_std::*;
    #[cfg(not(feature = "std"))]
    pub use crate::without_std::*;
}
pub mod serde;

pub mod types;

pub mod crypto;

pub mod utils;

pub mod chain_id;
