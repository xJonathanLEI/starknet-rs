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

    #[derive(PartialEq, Debug)]
    pub struct FromFieldElementError;

    #[derive(Debug)]
    pub struct FromBytesSliceError;

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
            write!(f, "invalid slice for ETH address")
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
            Err(FromBytesSliceError)
        } else {
            // Safe to unwrap as we know length is 20.
            Ok(Self {
                inner: value.try_into().unwrap(),
            })
        }
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
    use crate::types::eth_address::FromFieldElementError;
    use alloc::vec::*;
    use starknet_ff::FieldElement;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_eth_address_from_bytes_array() {
        // Reading the JSON file
        let json_data = include_str!("./test-data/address.json");

        // Parsing the JSON
        let addresses: Vec<String> =
            serde_json::from_str(json_data).expect("Unable to parse the JSON");

        // Iterating over each element in the JSON
        for address in addresses.iter() {
            // Convert hex string to bytes
            let bytes = if let Some(stripped) = address.strip_prefix("0x") {
                hex::decode(stripped).expect("Invalid address hex")
            } else {
                hex::decode(address).expect("Invalid address hex")
            };

            // Convert bytes to a fixed-size array
            let mut address_bytes: [u8; 20] = [0; 20];
            address_bytes.copy_from_slice(&bytes[..20]);

            let eth_address: EthAddress = address_bytes.into();

            // Asserting the conversion from hex string to EthAddress
            assert_eq!(EthAddress::from_hex(address).unwrap(), eth_address);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_eth_address_from_slice() {
        // Reading the JSON file containing Ethereum addresses
        let json_data = include_str!("./test-data/address.json");

        // Parsing the JSON into a vector of strings representing Ethereum addresses
        let addresses: Vec<String> =
            serde_json::from_str(json_data).expect("Unable to parse the JSON");

        // Iterating over each address in the JSON
        for address in addresses.iter() {
            // Generate random characters to prepend or append to the address
            let chars = "0123456789abcdef";

            // Combine random characters with the address, removing the "0x" prefix
            let address_with_random = format!(
                "{}{}{}",
                chars,
                if address.starts_with("0x") {
                    &address[2..]
                } else {
                    address
                },
                chars
            );

            // Convert the modified hex string to bytes
            let bytes = hex::decode(&address_with_random[2..]).expect("Invalid address hex");

            // Convert the byte slice to an Ethereum address
            let eth_address: EthAddress = (&bytes[7..27])
                .try_into()
                .expect("failed to get EthAddress from slice");

            // Assert that the conversion from hex string to Ethereum address is correct
            assert_eq!(EthAddress::from_hex(address).unwrap(), eth_address);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    // Define the unit test function
    fn test_eth_address_from_felt() {
        // Reading the JSON file containing addresses
        let json_data = include_str!("./test-data/address.json");

        // Parsing the JSON into a vector of strings
        let addresses: Vec<String> =
            serde_json::from_str(json_data).expect("Unable to parse the JSON");

        // Iterating over each address in the JSON
        for address in addresses.iter() {
            // Asserting the conversion from hex string to EthAddress is equal to Felt conversion
            assert_eq!(
                EthAddress::from_hex(address).unwrap(),
                EthAddress::from_felt(&FieldElement::from_hex_be(address).unwrap()).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_eth_address_from_felt_error() {
        match EthAddress::from_felt(
            &FieldElement::from_hex_be("0x10000000000000000000000000000000000000000").unwrap(),
        ) {
            Ok(_) => panic!("Expected error, but got Ok"),
            Err(err) => assert_eq!(err, FromFieldElementError),
        }
    }

    #[test]
    #[should_panic(expected = "FromBytesSliceError")]
    fn test_eth_address_from_slice_invalid_slice() {
        let buffer: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];

        EthAddress::try_from(&buffer[0..4]).unwrap();
    }
}
