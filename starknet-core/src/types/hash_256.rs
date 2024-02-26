use alloc::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    format,
    str::FromStr,
};

use serde::{de::Visitor, Deserialize, Serialize};
use starknet_ff::FieldElement;

const HASH_256_BYTE_COUNT: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hash256 {
    inner: [u8; HASH_256_BYTE_COUNT],
}

struct Hash256Visitor;

mod errors {
    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub enum FromHexError {
        UnexpectedLength,
        InvalidHexString,
    }

    #[derive(Debug)]
    pub struct ToFieldElementError;

    #[cfg(feature = "std")]
    impl std::error::Error for FromHexError {}

    #[cfg(feature = "std")]
    impl std::error::Error for ToFieldElementError {}

    impl Display for FromHexError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::UnexpectedLength => {
                    write!(f, "unexpected length for 256-bit hash")
                }
                Self::InvalidHexString => {
                    write!(f, "invalid hex string")
                }
            }
        }
    }

    impl Display for ToFieldElementError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "hash value out of range for FieldElement")
        }
    }
}
pub use errors::{FromHexError, ToFieldElementError};

impl Hash256 {
    pub fn from_bytes(bytes: [u8; HASH_256_BYTE_COUNT]) -> Self {
        Self { inner: bytes }
    }

    pub fn from_hex(hex: &str) -> Result<Self, FromHexError> {
        hex.parse()
    }

    pub fn from_felt(felt: &FieldElement) -> Self {
        felt.into()
    }

    pub fn as_bytes(&self) -> &[u8; HASH_256_BYTE_COUNT] {
        &self.inner
    }
}

impl Serialize for Hash256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(self.inner)))
    }
}

impl<'de> Deserialize<'de> for Hash256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Hash256Visitor)
    }
}

impl<'de> Visitor<'de> for Hash256Visitor {
    type Value = Hash256;

    fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse()
            .map_err(|err| serde::de::Error::custom(format!("{}", err)))
    }
}

impl FromStr for Hash256 {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim_start_matches("0x");

        let hex_chars_len = value.len();
        let expected_hex_length = HASH_256_BYTE_COUNT * 2;

        let parsed_bytes: [u8; HASH_256_BYTE_COUNT] = if hex_chars_len == expected_hex_length {
            let mut buffer = [0u8; HASH_256_BYTE_COUNT];
            hex::decode_to_slice(value, &mut buffer).map_err(|_| FromHexError::InvalidHexString)?;
            buffer
        } else if hex_chars_len < expected_hex_length {
            let mut padded_hex = str::repeat("0", expected_hex_length - hex_chars_len);
            padded_hex.push_str(value);

            let mut buffer = [0u8; HASH_256_BYTE_COUNT];
            hex::decode_to_slice(&padded_hex, &mut buffer)
                .map_err(|_| FromHexError::InvalidHexString)?;
            buffer
        } else {
            return Err(FromHexError::UnexpectedLength);
        };

        Ok(Self::from_bytes(parsed_bytes))
    }
}

impl Debug for Hash256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{}", hex::encode(self.inner))
    }
}

impl Display for Hash256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{}", hex::encode(self.inner))
    }
}

impl From<FieldElement> for Hash256 {
    fn from(value: FieldElement) -> Self {
        (&value).into()
    }
}

impl From<&FieldElement> for Hash256 {
    fn from(value: &FieldElement) -> Self {
        Self::from_bytes(value.to_bytes_be())
    }
}

impl TryFrom<Hash256> for FieldElement {
    type Error = ToFieldElementError;

    fn try_from(value: Hash256) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&Hash256> for FieldElement {
    type Error = ToFieldElementError;

    fn try_from(value: &Hash256) -> Result<Self, Self::Error> {
        FieldElement::from_bytes_be(&value.inner).map_err(|_| ToFieldElementError)
    }
}
