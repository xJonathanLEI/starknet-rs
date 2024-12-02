//! Support for `Bytes31` Cairo primitive.
//! <https://github.com/starkware-libs/cairo/blob/main/corelib/src/bytes_31.cairo>.
//!
//! This type is mostly used internally for [`crate::types::ByteArray`] internal logic.
use alloc::{
    string::{FromUtf8Error, String},
    vec::Vec,
};
use starknet_types_core::felt::FromStrError;

use crate::types::Felt;

pub const MAX_BYTES_COUNT: usize = 31;

pub const BYTES31_UPPER_BOUND: Felt = Felt::from_raw([
    18446744062762287109,
    20123647,
    18446744073709514624,
    576460566199926936,
]);

/// A 31 byte array primitive used mostly for [`crate::types::ByteArray`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Bytes31(Felt);

mod errors {
    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub struct FromFieldElementError;

    #[cfg(feature = "std")]
    impl std::error::Error for FromFieldElementError {}

    impl Display for FromFieldElementError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Felt value out of range for Bytes31")
        }
    }
}
pub use errors::FromFieldElementError;

impl Bytes31 {
    /// Converts a [`Bytes31`] into a UTF-8 string.
    /// Returns an error if the [`Bytes31`] contains an invalid UTF-8 string.
    ///
    /// # Arguments
    ///
    /// * `len` - The number of bytes in the [`Bytes31`] to consider in the string, at most 31.
    pub fn to_string(self, len: usize) -> Result<String, FromUtf8Error> {
        let mut buffer = Vec::new();

        // Bytes31 always enforce to have the first byte equal to 0 in the felt.
        // That's why we start to 1.
        for byte in &self.0.to_bytes_be()[1 + MAX_BYTES_COUNT - len..] {
            buffer.push(*byte)
        }

        String::from_utf8(buffer)
    }

    /// Converts a hex string to a [`Bytes31`].
    pub fn from_hex(hex: &str) -> Result<Self, FromStrError> {
        Ok(Self(Felt::from_hex(hex)?))
    }
}

impl From<Bytes31> for Felt {
    fn from(value: Bytes31) -> Self {
        value.0
    }
}

impl TryFrom<Felt> for Bytes31 {
    type Error = FromFieldElementError;

    fn try_from(value: Felt) -> Result<Self, Self::Error> {
        if value < BYTES31_UPPER_BOUND {
            Ok(Self(value))
        } else {
            Err(FromFieldElementError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bytes31, Felt, FromFieldElementError, BYTES31_UPPER_BOUND};

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bytes31_from_felt_out_of_range() {
        match Bytes31::try_from(Felt::MAX) {
            Err(FromFieldElementError) => {}
            _ => {
                panic!("Expected Bytes31::try_from(Felt::MAX) to return Err(FromFieldElementError)")
            }
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bytes31_from_felt() {
        let expected_felt = BYTES31_UPPER_BOUND - Felt::ONE;

        match Bytes31::try_from(expected_felt) {
            Ok(bytes31) => assert_eq!(Felt::from(bytes31), expected_felt),
            _ => panic!("Expected Bytes31 from Felt to be valid"),
        }
    }

    #[test]
    #[should_panic]
    fn test_bytes31_from_invalid_utf8() {
        let invalid = b"Hello \xF0\x90\x80World";
        let felt = Felt::from_bytes_be_slice(invalid);
        let bytes31 = Bytes31::try_from(felt).unwrap();

        if bytes31.to_string(4).is_ok() {
            panic!("Expected Bytes31 to contain invalid UTF-8")
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bytes31_from_valid_utf8() {
        let felt =
            Felt::from_hex("0x000000000000000000000000000000000000000000000000f09fa680f09f8c9f")
                .unwrap();

        let bytes31 = Bytes31::try_from(felt).unwrap();
        let string = bytes31.to_string(8).unwrap();

        assert_eq!(string, "🦀🌟");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bytes31_from_string_empty() {
        let bytes31 = Bytes31::try_from(Felt::ZERO).unwrap();
        let string = bytes31.to_string(0).unwrap();

        assert_eq!(string, "");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bytes31_from_hex() {
        let bytes31 = Bytes31::from_hex("0x1").unwrap();
        assert_eq!(Felt::ONE, bytes31.into());
    }
}
