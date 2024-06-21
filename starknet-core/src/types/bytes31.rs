use crate::types::Felt;

pub const BYTES31_UPPER_BOUND: Felt = Felt::from_raw([
    18446744062762287109,
    20123647,
    18446744073709514624,
    576460566199926936,
]);

/// A 31 byte array primitive used mostly for [`ByteArray`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
