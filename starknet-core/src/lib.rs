//! Core data types and utilities for Starknet.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain)]

/// Module containing custom serialization/deserialization implementations.
pub mod serde;

/// Module containing core types for representing objects in Starknet.
pub mod types;

/// High-level utilities for cryptographic operations used in Starknet.
pub mod crypto;

/// Utilities for performing commonly used algorithms in Starknet.
pub mod utils;

/// Chain IDs for commonly used public Starknet networks.
pub mod chain_id;

extern crate alloc;
