use crate::abi::cairo_types::CairoType;

use starknet_core::types::FieldElement;

/// Cairo types result.
pub type Result<T> = core::result::Result<T, Error>;

/// A cairo type error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid type string.
    InvalidTypeString(String),
    /// Error during serialization.
    Serialize(String),
    /// Error during deserialization.
    Deserialize(String),
}

impl CairoType for Error {
    type RustType = Self;

    fn serialize(_rust: &Self::RustType) -> Vec<FieldElement> {
        vec![]
    }

    fn deserialize(_felts: &[FieldElement], _offset: usize) -> Result<Self::RustType> {
        Ok(Error::Deserialize(
            "Error cairotype deserialized?".to_string(),
        ))
    }
}
