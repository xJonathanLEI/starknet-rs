use ethereum_types::{FromDecStrErr as UintFromDecStrErr, U256, U512};
use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;
use std::{
    fmt::{Display, LowerHex, UpperHex},
    str::FromStr,
};

const U256_BYTE_COUNT: usize = 32;
const U256_FIELD_MODULUS: U256 = U256([1, 0, 0, 576460752303423505]);
const U512_FIELD_MODULUS: U512 = U512([1, 0, 0, 576460752303423505, 0, 0, 0, 0]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsignedFieldElement {
    inner: U256,
}

#[derive(Debug, thiserror::Error)]
pub enum FromStrError {
    #[error("invalid character")]
    InvalidCharacter,
    #[error("invalid length")]
    InvalidLength,
    #[error("number out of range")]
    OutOfRange,
}

#[derive(Debug, thiserror::Error)]
pub enum FromUintError {
    #[error("number out of range")]
    OutOfRange,
}

#[derive(Debug, thiserror::Error)]
pub enum FromByteArrayError {
    #[error("invalid length")]
    InvalidLength,
    #[error("number out of range")]
    OutOfRange,
}

impl UnsignedFieldElement {
    /// [UnsignedFieldElement] constant that's equal to 0
    pub const ZERO: Self = Self {
        inner: U256([0, 0, 0, 0]),
    };

    /// [UnsignedFieldElement] constant that's equal to 1
    pub const ONE: Self = Self {
        inner: U256([1, 0, 0, 0]),
    };

    /// Maximum value of [UnsignedFieldElement]. Equals to 2^251 + 17 * 2^192.
    pub const MAX: Self = Self {
        inner: U256([0, 0, 0, 576460752303423505]),
    };

    pub fn from_hex_str(value: &str) -> Result<Self, FromStrError> {
        let value = value.trim_start_matches("0x");

        let hex_chars_len = value.len();
        let expected_hex_length = U256_BYTE_COUNT * 2;

        let parsed_bytes: Vec<u8> = if hex_chars_len == expected_hex_length {
            hex::decode(value).map_err(|_| FromStrError::InvalidCharacter)?
        } else if hex_chars_len < expected_hex_length {
            let mut padded_hex = str::repeat("0", expected_hex_length - hex_chars_len);
            padded_hex.push_str(value);
            hex::decode(&padded_hex).map_err(|_| FromStrError::InvalidCharacter)?
        } else {
            return Err(FromStrError::InvalidLength);
        };

        let parsed_u256 = U256::from_big_endian(&parsed_bytes);

        match Self::try_from(parsed_u256) {
            Ok(value) => Ok(value),
            Err(FromUintError::OutOfRange) => Err(FromStrError::OutOfRange),
        }
    }

    pub fn try_from_bytes_be(value: &[u8]) -> Result<Self, FromByteArrayError> {
        if value.len() != 32 {
            Err(FromByteArrayError::InvalidLength)
        } else {
            let value = U256::from_big_endian(value);
            if value > Self::MAX.inner {
                Err(FromByteArrayError::OutOfRange)
            } else {
                Ok(Self { inner: value })
            }
        }
    }
}

impl std::ops::Add<UnsignedFieldElement> for UnsignedFieldElement {
    type Output = Self;

    fn add(self, rhs: UnsignedFieldElement) -> Self::Output {
        // Allow overflow to align with Cairo behavior
        Self {
            inner: U256::try_from(
                (U512::from(self.inner) + U512::from(rhs.inner)) % U512_FIELD_MODULUS,
            )
            .unwrap(),
        }
    }
}

impl std::ops::Sub<UnsignedFieldElement> for UnsignedFieldElement {
    type Output = Self;

    fn sub(self, rhs: UnsignedFieldElement) -> Self::Output {
        // Allow underflow to align with Cairo behavior
        let (mut diff, underflow) = self.inner.overflowing_sub(rhs.inner);
        if underflow {
            let (sum, _) = diff.overflowing_add(U256_FIELD_MODULUS);
            diff = sum;
        }
        Self { inner: diff }
    }
}

impl std::ops::Mul<UnsignedFieldElement> for UnsignedFieldElement {
    type Output = Self;

    fn mul(self, rhs: UnsignedFieldElement) -> Self::Output {
        // Allow overflow to align with Cairo behavior
        Self {
            inner: U256::try_from(
                (U512::from(self.inner) * U512::from(rhs.inner)) % U512_FIELD_MODULUS,
            )
            .unwrap(),
        }
    }
}

impl Display for UnsignedFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl LowerHex for UnsignedFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Modified from the implementation of `U256` instead of invoking it directly because it
        // doesn't respect zero paddings. There's a pending PR to fix it:
        //   https://github.com/paritytech/parity-common/pull/603
        // TODO: change to formatting `inner` directly once the PR above is merged.

        let width = if f.sign_aware_zero_pad() {
            f.width().unwrap().min(64)
        } else {
            1
        };
        if f.alternate() {
            write!(f, "0x")?;
        }
        let mut latch = false;
        let mut ind_nibble = 0;
        for ch in self.inner.0.iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64)) >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0 || (64 - ind_nibble <= width);
                }
                if latch {
                    write!(f, "{:x}", nibble)?;
                }
                ind_nibble += 1;
            }
        }
        Ok(())
    }
}

impl UpperHex for UnsignedFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Same as `LowerHex`
        // TODO: change to formatting `inner` directly once the PR above is merged.

        let width = if f.sign_aware_zero_pad() {
            f.width().unwrap().min(64)
        } else {
            1
        };
        if f.alternate() {
            write!(f, "0x")?;
        }
        let mut latch = false;
        let mut ind_nibble = 0;
        for ch in self.inner.0.iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64)) >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0 || (64 - ind_nibble <= width);
                }
                if latch {
                    write!(f, "{:X}", nibble)?;
                }
                ind_nibble += 1;
            }
        }
        Ok(())
    }
}

impl FromStr for UnsignedFieldElement {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Unlike the underlying `U256` type, we're treating the decimal representation as the
        // canonical form.

        match U256::from_dec_str(s) {
            Ok(value) => match Self::try_from(value) {
                Ok(value) => Ok(value),
                Err(FromUintError::OutOfRange) => Err(FromStrError::OutOfRange),
            },
            Err(UintFromDecStrErr::InvalidCharacter) => Err(FromStrError::InvalidCharacter),
            Err(UintFromDecStrErr::InvalidLength) => Err(FromStrError::InvalidLength),
        }
    }
}

impl Serialize for UnsignedFieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for UnsignedFieldElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        UnsignedFieldElement::from_str(&value)
            .map_err(|err| serde::de::Error::custom(format!("invalid decimal string: {}", err)))
    }
}

impl TryFrom<U256> for UnsignedFieldElement {
    type Error = FromUintError;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value > Self::MAX.inner {
            Err(FromUintError::OutOfRange)
        } else {
            Ok(Self { inner: value })
        }
    }
}

impl From<UnsignedFieldElement> for U256 {
    fn from(value: UnsignedFieldElement) -> Self {
        value.inner
    }
}

impl From<FieldElement> for UnsignedFieldElement {
    fn from(value: FieldElement) -> Self {
        Self {
            inner: U256::from_big_endian(&value.to_bytes_be()),
        }
    }
}

impl From<UnsignedFieldElement> for FieldElement {
    fn from(value: UnsignedFieldElement) -> Self {
        let mut buffer = [0u8; 32];
        value.inner.to_big_endian(&mut buffer);

        // This can never fail as `inner` is always smaller than field modulus
        Self::from_bytes_be(buffer).unwrap()
    }
}

impl TryFrom<&[u8; 32]> for UnsignedFieldElement {
    type Error = FromByteArrayError;

    fn try_from(value: &[u8; 32]) -> Result<Self, Self::Error> {
        let value = U256::from_big_endian(value);
        if value > Self::MAX.inner {
            Err(FromByteArrayError::OutOfRange)
        } else {
            Ok(Self { inner: value })
        }
    }
}

impl From<usize> for UnsignedFieldElement {
    fn from(value: usize) -> Self {
        Self {
            inner: U256::from(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ethereum_types::U256;

    #[test]
    fn test_zero_padded_hex_fmt() {
        let fe = UnsignedFieldElement::try_from(U256::from_str_radix("0x1234abcd", 16).unwrap())
            .unwrap();

        assert_eq!(format!("{:011x}", fe), "0001234abcd");
        assert_eq!(format!("{:011X}", fe), "0001234ABCD");
        assert_eq!(format!("{:08x}", fe), "1234abcd");
        assert_eq!(format!("{:06x}", fe), "1234abcd");
        assert_eq!(format!("{:#x}", fe), "0x1234abcd");
        assert_eq!(
            format!("{:#064x}", fe),
            "0x000000000000000000000000000000000000000000000000000000001234abcd"
        );

        // Ignore if requesting more than 64 nibbles (or should we not?)
        assert_eq!(
            format!("{:#0100x}", fe),
            "0x000000000000000000000000000000000000000000000000000000001234abcd"
        );
    }

    #[test]
    fn test_addition() {
        let additions = vec![
            ["1", "1", "2"],
            [
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "1",
                "0",
            ],
        ];

        for item in additions.iter() {
            assert_eq!(
                UnsignedFieldElement::from_str(item[0]).unwrap()
                    + UnsignedFieldElement::from_str(item[1]).unwrap(),
                UnsignedFieldElement::from_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    fn test_subtraction() {
        let subtractions = vec![
            ["10", "7", "3"],
            [
                "0",
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "1",
            ],
        ];

        for item in subtractions.iter() {
            assert_eq!(
                UnsignedFieldElement::from_str(item[0]).unwrap()
                    - UnsignedFieldElement::from_str(item[1]).unwrap(),
                UnsignedFieldElement::from_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    fn test_multiplication() {
        let multiplications = vec![
            ["2", "3", "6"],
            [
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "1",
            ],
            [
                "3141592653589793238462643383279502884197169399375105820974944592307",
                "8164062862089986280348253421170679821480865132823066470938446095505",
                "514834056922159274131066670130609582664841480950767778400381816737396274242",
            ],
        ];

        for item in multiplications.iter() {
            assert_eq!(
                UnsignedFieldElement::from_str(item[0]).unwrap()
                    * UnsignedFieldElement::from_str(item[1]).unwrap(),
                UnsignedFieldElement::from_str(item[2]).unwrap()
            );
        }
    }
}
