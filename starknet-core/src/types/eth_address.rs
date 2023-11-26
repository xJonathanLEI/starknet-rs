use alloc::{fmt::Formatter, format};
use core::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};
use starknet_ff::FieldElement;

// 0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF
const MAX_L1_ADDRESS: FieldElement = FieldElement::from_mont([
    18406070939574861858,
    74766790688767,
    18446743936270598144,
    461478224317121089,
]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthAddress {
    inner: [u8; 20],
}

struct EthAddressVisitor;

mod errors {
    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub enum FromHexError {
        UnexpectedLength,
        InvalidHexString,
    }

    #[derive(Debug)]
    pub struct FromFieldElementError;

    #[derive(Debug)]
    pub struct FromBytesSliceError(pub alloc::string::String);

    #[cfg(feature = "std")]
    impl std::error::Error for FromHexError {}

    #[cfg(feature = "std")]
    impl std::error::Error for FromFieldElementError {}

    #[cfg(feature = "std")]
    impl std::error::Error for FromBytesSliceError {}

    impl Display for FromHexError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::UnexpectedLength => {
                    write!(f, "unexpected length for ETH address")
                }
                Self::InvalidHexString => {
                    write!(f, "invalid hex string")
                }
            }
        }
    }

    impl Display for FromFieldElementError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "FieldElement value out of range")
        }
    }

    impl Display for FromBytesSliceError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "invalid slice for ETH address: {}", self.0)
        }
    }
}
pub use errors::{FromBytesSliceError, FromFieldElementError, FromHexError};

impl EthAddress {
    pub fn from_hex(hex: &str) -> Result<Self, FromHexError> {
        hex.parse()
    }

    pub fn from_felt(felt: &FieldElement) -> Result<Self, FromFieldElementError> {
        felt.try_into()
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.inner
    }
}

impl Serialize for EthAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(self.inner)))
    }
}

impl<'de> Deserialize<'de> for EthAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(EthAddressVisitor)
    }
}

impl<'de> Visitor<'de> for EthAddressVisitor {
    type Value = EthAddress;

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

impl FromStr for EthAddress {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.trim_start_matches("0x");

        if value.len() == 40 {
            match hex::decode(value) {
                Ok(bytes) => {
                    Ok(Self {
                        // It's safe to unwrap here as the length must be 20
                        inner: bytes.try_into().unwrap(),
                    })
                }
                Err(_) => Err(FromHexError::InvalidHexString),
            }
        } else {
            Err(FromHexError::UnexpectedLength)
        }
    }
}

impl TryFrom<FieldElement> for EthAddress {
    type Error = FromFieldElementError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl TryFrom<&FieldElement> for EthAddress {
    type Error = FromFieldElementError;

    fn try_from(value: &FieldElement) -> Result<Self, Self::Error> {
        if value <= &MAX_L1_ADDRESS {
            let mut buffer = [0u8; 20];
            buffer.copy_from_slice(&value.to_bytes_be()[12..]);
            Ok(Self { inner: buffer })
        } else {
            Err(FromFieldElementError)
        }
    }
}

impl From<EthAddress> for FieldElement {
    fn from(value: EthAddress) -> Self {
        // Safe to unwrap here as the value is never out of range
        FieldElement::from_byte_slice_be(&value.inner).unwrap()
    }
}

impl TryFrom<&[u8]> for EthAddress {
    type Error = FromBytesSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 20 {
            return Err(FromBytesSliceError(format!(
                "expected 20 bytes found {}",
                value.len()
            )));
        }

        Ok(Self {
            inner: value
                .try_into()
                .map_err(|e| FromBytesSliceError(format!("can't convert slice to array: {}", e)))?,
        })
    }
}

impl From<[u8; 20]> for EthAddress {
    fn from(value: [u8; 20]) -> Self {
        Self { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use super::EthAddress;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_eth_address_from_bytes_array() {
        // address: e7f1725e7734ce288f8367e1bb143e90bb3f0512
        let address_bytes: [u8; 20] = [
            231, 241, 114, 94, 119, 52, 206, 40, 143, 131, 103, 225, 187, 20, 62, 144, 187, 63, 5,
            18,
        ];

        let eth_address: EthAddress = address_bytes.into();
        assert_eq!(
            EthAddress::from_hex("0xe7f1725e7734ce288f8367e1bb143e90bb3f0512").unwrap(),
            eth_address
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_eth_address_from_slice() {
        // address: e7f1725e7734ce288f8367e1bb143e90bb3f0512, inside a buffer with more data.
        let buffer: Vec<u8> = vec![
            0, 1, 2, 231, 241, 114, 94, 119, 52, 206, 40, 143, 131, 103, 225, 187, 20, 62, 144,
            187, 63, 5, 18, 11, 22, 33,
        ];

        let eth_address: EthAddress = (&buffer[3..23])
            .try_into()
            .expect("failed to get eth_address from slice");
        assert_eq!(
            EthAddress::from_hex("0xe7f1725e7734ce288f8367e1bb143e90bb3f0512").unwrap(),
            eth_address
        );
    }

    #[test]
    #[should_panic(expected = "fail: FromBytesSliceError(\"expected 20 bytes found 4\")")]
    fn test_eth_address_from_slice_invalid_slice() {
        let buffer: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];

        EthAddress::try_from(&buffer[0..4]).expect("fail");
    }
}
