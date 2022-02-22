#![allow(clippy::comparison_chain)]

use crate::fr::FrParameters;

use ark_ff::{fields::Fp256, BigInteger, BigInteger256, Field, PrimeField, SquareRootField};
use bitvec::{array::BitArray, order::Lsb0};
use crypto_bigint::{CheckedAdd, CheckedMul, Zero, U256};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, LowerHex, UpperHex};

mod fr;

const U256_BYTE_COUNT: usize = 32;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FieldElement {
    inner: Fp256<FrParameters>,
}

#[derive(Debug, thiserror::Error)]
pub enum FromDecStrError {
    #[error("invalid character")]
    InvalidCharacter,
    #[error("number out of range")]
    OutOfRange,
}

#[derive(Debug, thiserror::Error)]
pub enum FromHexError {
    #[error("invalid character")]
    InvalidCharacter,
    #[error("invalid length")]
    InvalidLength,
    #[error("number out of range")]
    OutOfRange,
}

#[derive(Debug, thiserror::Error)]
pub enum FromByteSliceError {
    #[error("invalid length")]
    InvalidLength,
    #[error("number out of range")]
    OutOfRange,
}

#[derive(Debug, thiserror::Error)]
#[error("number out of range")]
pub struct FromByteArrayError;

struct InnerDebug<'a>(pub &'a FieldElement);

impl FieldElement {
    /// [FieldElement] constant that's equal to 0
    pub const ZERO: FieldElement = FieldElement::from_mont([0, 0, 0, 0]);

    /// [FieldElement] constant that's equal to 1
    pub const ONE: FieldElement = FieldElement::from_mont([
        18446744073709551585,
        18446744073709551615,
        18446744073709551615,
        576460752303422960,
    ]);

    /// Maximum value of [FieldElement]. Equals to 2^251 + 17 * 2^192.
    pub const MAX: FieldElement = FieldElement::from_mont([32, 0, 0, 544]);

    /// Create a new [FieldElement] from its Montgomery representation
    pub const fn from_mont(data: [u64; 4]) -> Self {
        Self {
            inner: Fp256::new(BigInteger256::new(data)),
        }
    }

    pub fn from_dec_str(value: &str) -> Result<Self, FromDecStrError> {
        // Ported from:
        //   https://github.com/paritytech/parity-common/blob/b37d0b312d39fa47c61c4430b30ca87d90e45a08/uint/src/uint.rs#L599

        let mut res = U256::ZERO;
        for b in value.bytes().map(|b| b.wrapping_sub(b'0')) {
            if b > 9 {
                return Err(FromDecStrError::InvalidCharacter);
            }
            let r = {
                let product = res.checked_mul(&U256::from_u8(10));
                if product.is_some().into() {
                    product.unwrap()
                } else {
                    return Err(FromDecStrError::OutOfRange);
                }
            };
            let r = {
                let sum = r.checked_add(&U256::from_u8(b));
                if sum.is_some().into() {
                    sum.unwrap()
                } else {
                    return Err(FromDecStrError::OutOfRange);
                }
            };
            res = r;
        }

        Fp256::<FrParameters>::from_repr(BigInteger256::new(res.to_uint_array()))
            .map(|inner| Self { inner })
            .ok_or(FromDecStrError::OutOfRange)
    }

    pub fn from_hex_be(value: &str) -> Result<Self, FromHexError> {
        let value = value.trim_start_matches("0x");

        let hex_chars_len = value.len();
        let expected_hex_length = U256_BYTE_COUNT * 2;

        let parsed_bytes: [u8; U256_BYTE_COUNT] = if hex_chars_len == expected_hex_length {
            let mut buffer = [0u8; U256_BYTE_COUNT];
            hex::decode_to_slice(value, &mut buffer).map_err(|_| FromHexError::InvalidCharacter)?;
            buffer
        } else if hex_chars_len < expected_hex_length {
            let mut padded_hex = str::repeat("0", expected_hex_length - hex_chars_len);
            padded_hex.push_str(value);

            let mut buffer = [0u8; U256_BYTE_COUNT];
            hex::decode_to_slice(&padded_hex, &mut buffer)
                .map_err(|_| FromHexError::InvalidCharacter)?;
            buffer
        } else {
            return Err(FromHexError::InvalidLength);
        };

        match Self::from_bytes_be(&parsed_bytes) {
            Ok(value) => Ok(value),
            Err(_) => Err(FromHexError::OutOfRange),
        }
    }

    /// Attempts to convert a big-endian byte representation of a field element into an element of
    /// this prime field. Returns error if the input is not canonical (is not smaller than the
    /// field's modulus).
    ///
    /// ### Arguments
    ///
    /// * `bytes`: The byte array in **big endian** format
    pub fn from_bytes_be(bytes: &[u8; 32]) -> Result<Self, FromByteArrayError> {
        Self::from_byte_slice(bytes).ok_or(FromByteArrayError)
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub fn to_bits_le(self) -> BitArray<Lsb0, [u64; 4]> {
        BitArray::<Lsb0, [u64; 4]>::new(self.inner.into_repr().0)
    }

    /// Convert the field element into a big-endian byte representation
    pub fn to_bytes_be(&self) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&self.inner.into_repr().to_bytes_be());

        buffer
    }

    pub fn invert(&self) -> Option<FieldElement> {
        self.inner.inverse().map(|inner| Self { inner })
    }

    pub fn sqrt(&self) -> Option<FieldElement> {
        self.inner.sqrt().map(|inner| Self { inner })
    }

    /// For internal use only. The input must be of length [U256_BYTE_COUNT].
    fn from_byte_slice(bytes: &[u8]) -> Option<Self> {
        let mut bits = [false; U256_BYTE_COUNT * 8];
        for (ind_byte, byte) in bytes.iter().enumerate() {
            for ind_bit in 0..8 {
                bits[ind_byte * 8 + ind_bit] = (byte >> (7 - ind_bit)) & 1 == 1;
            }
        }

        // No need to check range as `from_repr` already does that
        let big_int = BigInteger256::from_bits_be(&bits);
        Fp256::<FrParameters>::from_repr(big_int).map(|inner| Self { inner })
    }
}

impl std::ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner + rhs.inner,
        }
    }
}

impl std::ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner - rhs.inner,
        }
    }
}

impl std::ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner * rhs.inner,
        }
    }
}

impl std::ops::Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        FieldElement { inner: -self.inner }
    }
}

impl Debug for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldElement")
            .field("inner", &InnerDebug(self))
            .finish()
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ported from:
        //   https://github.com/paritytech/parity-common/blob/b37d0b312d39fa47c61c4430b30ca87d90e45a08/uint/src/uint.rs#L1650

        let repr = U256::from_uint_array(self.inner.into_repr().0);

        if repr.is_zero().into() {
            return write!(f, "0");
        }

        let mut buf = [0u8; 4 * 20];
        let mut i = buf.len() - 1;
        let mut current = repr;
        let ten = U256::from_u8(10u8);

        loop {
            let digit = (current.checked_rem(&ten)).unwrap().to_uint_array()[0] as u8;
            buf[i] = digit + b'0';
            current = current.checked_div(&ten).unwrap();
            if current.is_zero().into() {
                break;
            }
            i -= 1;
        }

        // sequence of `'0'..'9'` chars is guaranteed to be a valid UTF8 string
        let s = unsafe { std::str::from_utf8_unchecked(&buf[i..]) };
        f.pad_integral(true, "", s)
    }
}

impl LowerHex for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = U256::from_uint_array(self.inner.into_repr().0);

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
        for ch in repr.to_uint_array().iter().rev() {
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

impl UpperHex for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = U256::from_uint_array(self.inner.into_repr().0);

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
        for ch in repr.to_uint_array().iter().rev() {
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

impl Serialize for FieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for FieldElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_dec_str(&value)
            .map_err(|err| serde::de::Error::custom(format!("invalid decimal string: {}", err)))
    }
}

impl From<usize> for FieldElement {
    fn from(value: usize) -> Self {
        Self {
            inner: Fp256::<FrParameters>::from_repr(BigInteger256::new([value as u64, 0, 0, 0]))
                .unwrap(),
        }
    }
}

impl<'a> Debug for InnerDebug<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#064x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_padded_hex_fmt() {
        let fe = FieldElement::from_hex_be("0x1234abcd").unwrap();

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
        let additions = [
            ["1", "1", "2"],
            [
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "1",
                "0",
            ],
        ];

        for item in additions.iter() {
            assert_eq!(
                FieldElement::from_dec_str(item[0]).unwrap()
                    + FieldElement::from_dec_str(item[1]).unwrap(),
                FieldElement::from_dec_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    fn test_subtraction() {
        let subtractions = [
            ["10", "7", "3"],
            [
                "0",
                "3618502788666131213697322783095070105623107215331596699973092056135872020480",
                "1",
            ],
        ];

        for item in subtractions.iter() {
            assert_eq!(
                FieldElement::from_dec_str(item[0]).unwrap()
                    - FieldElement::from_dec_str(item[1]).unwrap(),
                FieldElement::from_dec_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    fn test_multiplication() {
        let multiplications = [
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
                FieldElement::from_dec_str(item[0]).unwrap()
                    * FieldElement::from_dec_str(item[1]).unwrap(),
                FieldElement::from_dec_str(item[2]).unwrap()
            );
        }
    }
}
