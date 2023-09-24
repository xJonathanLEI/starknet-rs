//! This crate contains the definition of traits and types
//! that map to Cairo types.
//!
//! Some of the Cairo types are provided in the ABI event if they are very generic
//! like `Option`, `Result`, etc...
//! This crate provides the `CairoType` implementation for those types and all basic
//! types from Cairo (integers, felt etc...).
//!
mod error;
pub use error::{Error, Result};

mod types;

use starknet_core::types::FieldElement;

/// Basic cairo structs that are already implemented inside
/// this crate and hence skipped during ABI generation.
pub const CAIRO_BASIC_STRUCTS: [&str; 4] = ["Span", "ClassHash", "ContractAddress", "EthAddress"];

/// Same as `CAIRO_BASIC_STRUCTS`, but for enums.
pub const CAIRO_BASIC_ENUMS: [&str; 2] = ["Option", "Result"];

/// CairoType trait to implement in order to serialize/deserialize
/// a Rust type to/from a CairoType.
pub trait CairoType {
    /// The corresponding Rust type.
    type RustType;

    /// The serialized size of the type in felts, if known at compile time.
    const SERIALIZED_SIZE: Option<usize> = Some(1);

    /// Whether the serialized size is dynamic.
    const DYNAMIC: bool = Self::SERIALIZED_SIZE.is_none();

    /// Calculates the serialized size of the data for a single felt
    /// it will always be 1.
    /// If the type is dynamic, SERIALIZED_SIZE is None, but this
    /// function is overriden to correctly compute the size.
    #[inline]
    fn serialized_size(_rust: &Self::RustType) -> usize {
        Self::SERIALIZED_SIZE.unwrap()
    }

    /// Serializes the given type into a FieldElement sequence.
    fn serialize(rust: &Self::RustType) -> Vec<FieldElement>;

    /// TODO: add serialize_to(rust: &Self::RustType, out: &mut Vec<FieldElement>)
    /// for large buffers optimization.

    /// Deserializes an array of felts into the given type.
    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType>;
}
