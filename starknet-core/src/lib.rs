#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain)]
#![doc = include_str!("../README.md")]

#[cfg(all(not(feature = "std"), any(test, feature = "alloc")))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

pub mod serde;

pub mod types;

pub mod crypto;

pub mod utils;

pub mod chain_id;
