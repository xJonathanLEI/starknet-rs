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
    pub const fn from_bytes(bytes: [u8; HASH_256_BYTE_COUNT]) -> Self {
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

        Ok(parsed_bytes.into())
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
        value.to_bytes_be().into()
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

impl From<[u8; HASH_256_BYTE_COUNT]> for Hash256 {
    fn from(value: [u8; HASH_256_BYTE_COUNT]) -> Self {
        Self { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use super::{FromHexError, Hash256, HASH_256_BYTE_COUNT};

    use starknet_ff::FieldElement;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_hash_256_from_hex_error_unexpected_length() {
        let test_data = [
            // Hexadecimal string with correct prefix but incorrect length
            "0x25c5b1592b1743b62d7fabd4373d98219c2ff3750f49ec0608a8355fa3bb060f5",
            // Hexadecimal string without correct prefix and incorrect length
            "25c5b1592b1743b62d7fabd4373d98219c2ff3750f49ec0608a8355fa3bb060f5",
        ];

        for item in test_data.into_iter() {
            match Hash256::from_hex(item) {
                Err(FromHexError::UnexpectedLength) => {}
                _ => panic!("Unexpected test result"),
            }
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_hash_256_from_hex_error_invalid_string() {
        let test_data = [
            // Hexadecimal string with incorrect characters
            "25c5b1592b1743b62d7fabd4373d98219c2f63750f49ec0608a8355fa3bb060.",
            // Hexadecimal string with non-hex characters
            "0x?5c5b1592b1743b62d7fabd4373d98219c2f63750f49ec0608a8355fa3bb060",
        ];

        for item in test_data.into_iter() {
            match Hash256::from_hex(item) {
                Err(FromHexError::InvalidHexString) => {}
                _ => panic!("Unexpected test result"),
            }
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_hash_256_from_felt() {
        // Create a `FieldElement` from a hexadecimal string representation
        let felt = FieldElement::from_hex_be(
            "0x01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003",
        )
        .unwrap();

        // Convert the `FieldElement` to bytes and then to a vector
        let bytes = (felt.to_bytes_be()).to_vec();

        // Convert bytes to a fixed-size array representing `Hash256`
        let mut hash_bytes: [u8; HASH_256_BYTE_COUNT] = [0; HASH_256_BYTE_COUNT];
        hash_bytes.copy_from_slice(&bytes[..HASH_256_BYTE_COUNT]);

        // Convert `Hash256` bytes to `Hash256` struct
        let hash_256: Hash256 = hash_bytes.into();

        // Assert that the conversion from the `FieldElement` to `Hash256` is successful
        assert_eq!(Hash256::from_felt(&felt), hash_256);
    }
}
