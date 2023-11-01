#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain)]

#[cfg(all(not(feature = "std"), any(test, feature = "alloc")))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

use core::{
    fmt, ops,
    str::{self, FromStr},
};

use crate::fr::Fr;

use ark_ff::{
    fields::{Field, Fp256, PrimeField},
    BigInteger, BigInteger256,
};
use crypto_bigint::{CheckedAdd, CheckedMul, NonZero, Zero, U256};

mod fr;

const U256_BYTE_COUNT: usize = 32;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FieldElement {
    inner: Fr,
}

mod from_str_error {

    #[derive(Debug)]
    pub enum FromStrError {
        InvalidCharacter,
        OutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for FromStrError {}

    impl core::fmt::Display for FromStrError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidCharacter => write!(f, "invalid character"),
                Self::OutOfRange => write!(f, "number out of range"),
            }
        }
    }
}
pub use from_str_error::FromStrError;

mod from_bytes_slice_error {

    #[derive(Debug)]
    pub enum FromByteSliceError {
        InvalidLength,
        OutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for FromByteSliceError {}

    impl core::fmt::Display for FromByteSliceError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidLength => write!(f, "invalid length"),
                Self::OutOfRange => write!(f, "number out of range"),
            }
        }
    }
}
pub use from_bytes_slice_error::FromByteSliceError;

mod from_byte_array_error {
    #[derive(Debug)]
    pub struct FromByteArrayError;

    #[cfg(feature = "std")]
    impl std::error::Error for FromByteArrayError {}

    impl core::fmt::Display for FromByteArrayError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "number out of range")
        }
    }
}
pub use from_byte_array_error::FromByteArrayError;

mod value_out_of_range_error {
    #[derive(Debug)]
    pub struct ValueOutOfRangeError;

    #[cfg(feature = "std")]
    impl std::error::Error for ValueOutOfRangeError {}

    impl core::fmt::Display for ValueOutOfRangeError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "field element value out of range")
        }
    }
}
pub use value_out_of_range_error::ValueOutOfRangeError;

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

    /// [FieldElement] constant that's equal to 2
    pub const TWO: FieldElement = FieldElement::from_mont([
        18446744073709551553,
        18446744073709551615,
        18446744073709551615,
        576460752303422416,
    ]);

    /// [FieldElement] constant that's equal to 3
    pub const THREE: FieldElement = FieldElement::from_mont([
        18446744073709551521,
        18446744073709551615,
        18446744073709551615,
        576460752303421872,
    ]);

    /// Maximum value of [FieldElement]. Equals to 2^251 + 17 * 2^192.
    pub const MAX: FieldElement = FieldElement::from_mont([32, 0, 0, 544]);

    /// Create a new [FieldElement] from its Montgomery representation
    pub const fn from_mont(data: [u64; 4]) -> Self {
        Self {
            inner: Fp256::new_unchecked(BigInteger256::new(data)),
        }
    }

    pub fn from_dec_str(value: &str) -> Result<Self, FromStrError> {
        // Ported from:
        //   https://github.com/paritytech/parity-common/blob/b37d0b312d39fa47c61c4430b30ca87d90e45a08/uint/src/uint.rs#L599

        let mut res = U256::ZERO;
        for b in value.bytes().map(|b| b.wrapping_sub(b'0')) {
            if b > 9 {
                return Err(FromStrError::InvalidCharacter);
            }
            let r = {
                let product = res.checked_mul(&U256::from_u8(10));
                if product.is_some().into() {
                    product.unwrap()
                } else {
                    return Err(FromStrError::OutOfRange);
                }
            };
            let r = {
                let sum = r.checked_add(&U256::from_u8(b));
                if sum.is_some().into() {
                    sum.unwrap()
                } else {
                    return Err(FromStrError::OutOfRange);
                }
            };
            res = r;
        }

        Fr::from_bigint(u256_to_biginteger256(&res))
            .map(|inner| Self { inner })
            .ok_or(FromStrError::OutOfRange)
    }

    pub fn from_hex_be(value: &str) -> Result<Self, FromStrError> {
        let value = value.trim_start_matches("0x");

        let hex_chars_len = value.len();
        let expected_hex_length = U256_BYTE_COUNT * 2;

        let parsed_bytes: [u8; U256_BYTE_COUNT] = if hex_chars_len == expected_hex_length {
            let mut buffer = [0u8; U256_BYTE_COUNT];
            hex::decode_to_slice(value, &mut buffer).map_err(|_| FromStrError::InvalidCharacter)?;
            buffer
        } else if hex_chars_len < expected_hex_length {
            let mut padded_hex = str::repeat("0", expected_hex_length - hex_chars_len);
            padded_hex.push_str(value);

            let mut buffer = [0u8; U256_BYTE_COUNT];
            hex::decode_to_slice(&padded_hex, &mut buffer)
                .map_err(|_| FromStrError::InvalidCharacter)?;
            buffer
        } else {
            return Err(FromStrError::OutOfRange);
        };

        match Self::from_bytes_be(&parsed_bytes) {
            Ok(value) => Ok(value),
            Err(_) => Err(FromStrError::OutOfRange),
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

    /// Same as `from_bytes_be` except this function takes a slice.
    pub fn from_byte_slice_be(bytes: &[u8]) -> Result<Self, FromByteSliceError> {
        if bytes.len() > U256_BYTE_COUNT {
            Err(FromByteSliceError::InvalidLength)
        } else {
            let mut buffer = [0u8; U256_BYTE_COUNT];
            buffer[(U256_BYTE_COUNT - bytes.len())..].copy_from_slice(bytes);
            Self::from_byte_slice(&buffer).ok_or(FromByteSliceError::OutOfRange)
        }
    }

    /// Interprets the field element as a decimal number of a certain decimal places.
    #[cfg(feature = "bigdecimal")]
    pub fn to_big_decimal<D: Into<i64>>(&self, decimals: D) -> bigdecimal::BigDecimal {
        use num_bigint::{BigInt, Sign};

        bigdecimal::BigDecimal::new(
            BigInt::from_bytes_be(Sign::Plus, &self.to_bytes_be()),
            decimals.into(),
        )
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub fn to_bits_le(self) -> [bool; 256] {
        let mut bits = [false; 256];
        for (ind_element, element) in self.inner.into_bigint().0.iter().enumerate() {
            for ind_bit in 0..64 {
                bits[ind_element * 64 + ind_bit] = (element >> ind_bit) & 1 == 1;
            }
        }

        bits
    }

    /// Convert the field element into a big-endian byte representation
    pub fn to_bytes_be(&self) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&self.inner.into_bigint().to_bytes_be());

        buffer
    }

    /// Transforms [FieldElement] into its Montgomery representation
    pub const fn into_mont(self) -> [u64; 4] {
        self.inner.0 .0
    }

    pub fn invert(&self) -> Option<FieldElement> {
        self.inner.inverse().map(|inner| Self { inner })
    }

    pub fn sqrt(&self) -> Option<FieldElement> {
        self.inner.sqrt().map(|inner| Self { inner })
    }

    /// Performs a floor division. It's not implemented as the `Div` trait on purpose to
    /// distinguish from the "felt division".
    pub fn floor_div(&self, rhs: FieldElement) -> FieldElement {
        let lhs: U256 = self.into();
        let rhs: U256 = (&rhs).into();
        let is_rhs_zero: bool = rhs.is_zero().into();

        if !is_rhs_zero {
            let rhs = NonZero::from_uint(rhs);

            let div_result = lhs.div_rem(&rhs);
            let (quotient, _) = div_result;

            // It's safe to unwrap here since `rem` is never out of range
            FieldElement {
                inner: Fr::from_bigint(u256_to_biginteger256(&quotient)).unwrap(),
            }
        } else {
            // TODO: add `checked_floor_div` for panic-less use
            panic!("division by zero");
        }
    }

    /// For internal use only. The input must be of length [U256_BYTE_COUNT].
    fn from_byte_slice(bytes: &[u8]) -> Option<Self> {
        let mut bits = [false; U256_BYTE_COUNT * 8];
        for (ind_byte, byte) in bytes.iter().enumerate() {
            for ind_bit in 0..8 {
                bits[ind_byte * 8 + ind_bit] = (byte >> (7 - ind_bit)) & 1 == 1;
            }
        }

        // No need to check range as `from_bigint` already does that
        let big_int = BigInteger256::from_bits_be(&bits);
        Fr::from_bigint(big_int).map(|inner| Self { inner })
    }
}

impl Default for FieldElement {
    fn default() -> Self {
        Self::ZERO
    }
}

impl AsRef<FieldElement> for FieldElement {
    fn as_ref(&self) -> &FieldElement {
        self
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner + rhs.inner,
        }
    }
}

impl ops::AddAssign<FieldElement> for FieldElement {
    fn add_assign(&mut self, rhs: FieldElement) {
        self.inner = self.inner + rhs.inner;
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner - rhs.inner,
        }
    }
}

impl ops::SubAssign<FieldElement> for FieldElement {
    fn sub_assign(&mut self, rhs: FieldElement) {
        self.inner = self.inner - rhs.inner;
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement {
            inner: self.inner * rhs.inner,
        }
    }
}

impl ops::MulAssign<FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: FieldElement) {
        self.inner = self.inner * rhs.inner;
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        FieldElement { inner: -self.inner }
    }
}

impl ops::Rem<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn rem(self, rhs: FieldElement) -> Self::Output {
        if self.inner < rhs.inner {
            return self;
        }

        let lhs: U256 = (&self).into();
        let rhs: U256 = (&rhs).into();
        let is_rhs_zero: bool = rhs.is_zero().into();

        if !is_rhs_zero {
            let rhs = NonZero::from_uint(rhs);

            let (_, rem) = lhs.div_rem(&rhs);

            // It's safe to unwrap here since `rem` is never out of range
            FieldElement {
                inner: Fr::from_bigint(u256_to_biginteger256(&rem)).unwrap(),
            }
        } else {
            // TODO: add `checked_rem` for panic-less use
            panic!("division by zero");
        }
    }
}

impl ops::BitAnd<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn bitand(self, rhs: FieldElement) -> Self::Output {
        let lhs: U256 = (&self).into();
        let rhs: U256 = (&rhs).into();

        // It's safe to unwrap here since the result is never out of range
        FieldElement {
            inner: Fr::from_bigint(u256_to_biginteger256(&(lhs & rhs))).unwrap(),
        }
    }
}

impl ops::BitOr<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn bitor(self, rhs: FieldElement) -> Self::Output {
        let lhs: U256 = (&self).into();
        let rhs: U256 = (&rhs).into();

        // It's safe to unwrap here since the result is never out of range
        FieldElement {
            inner: Fr::from_bigint(u256_to_biginteger256(&(lhs | rhs))).unwrap(),
        }
    }
}

impl core::iter::Sum for FieldElement {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Self::ZERO;
        iter.for_each(|item| {
            sum += item;
        });
        sum
    }
}

impl<'a> core::iter::Sum<&'a FieldElement> for FieldElement {
    fn sum<I: Iterator<Item = &'a FieldElement>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldElement")
            .field("inner", &InnerDebug(self))
            .finish()
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Ported from:
        //   https://github.com/paritytech/parity-common/blob/b37d0b312d39fa47c61c4430b30ca87d90e45a08/uint/src/uint.rs#L1650

        let repr: U256 = self.into();

        if repr.is_zero().into() {
            return write!(f, "0");
        }

        let mut buf = [0u8; 4 * 20];
        let mut i = buf.len() - 1;
        let mut current = repr;
        let ten = U256::from_u8(10u8);

        loop {
            let digit = if current < ten {
                current.to_words()[0] as u8
            } else {
                (current.checked_rem(&ten)).unwrap().to_words()[0] as u8
            };
            buf[i] = digit + b'0';
            current = current.checked_div(&ten).unwrap();
            if current.is_zero().into() {
                break;
            }
            i -= 1;
        }

        // sequence of `'0'..'9'` chars is guaranteed to be a valid UTF8 string
        let s = unsafe { str::from_utf8_unchecked(&buf[i..]) };
        f.pad_integral(true, "", s)
    }
}

impl fmt::LowerHex for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: U256 = self.into();

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
        for ch in u256_to_u64_array(&repr).iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64)) >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0 || (64 - ind_nibble <= width);
                }
                if latch {
                    write!(f, "{nibble:x}")?;
                }
                ind_nibble += 1;
            }
        }
        Ok(())
    }
}

impl fmt::UpperHex for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: U256 = self.into();

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
        for ch in u256_to_u64_array(&repr).iter().rev() {
            for x in 0..16 {
                let nibble = (ch & (15u64 << ((15 - x) * 4) as u64)) >> (((15 - x) * 4) as u64);
                if !latch {
                    latch = nibble != 0 || (64 - ind_nibble <= width);
                }
                if latch {
                    write!(f, "{nibble:X}")?;
                }
                ind_nibble += 1;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
mod serde_field_element {
    #[cfg(feature = "std")]
    use core::fmt::{Formatter, Result as FmtResult};

    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::{
        fmt::{Formatter, Result as FmtResult},
        string::ToString,
    };
    use serde::{de::Visitor, Deserialize, Serialize};

    struct FieldElementVisitor;

    impl Serialize for FieldElement {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&ToString::to_string(&self))
        }
    }

    impl<'de> Deserialize<'de> for FieldElement {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(FieldElementVisitor)
        }
    }

    impl<'de> Visitor<'de> for FieldElementVisitor {
        type Value = FieldElement;

        fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
            write!(formatter, "string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            FieldElement::from_str(v).map_err(serde::de::Error::custom)
        }
    }
}

impl From<u8> for FieldElement {
    fn from(value: u8) -> Self {
        Self {
            inner: Fr::from_bigint(BigInteger256::new([value as u64, 0, 0, 0])).unwrap(),
        }
    }
}

impl From<u16> for FieldElement {
    fn from(value: u16) -> Self {
        Self {
            inner: Fr::from_bigint(BigInteger256::new([value as u64, 0, 0, 0])).unwrap(),
        }
    }
}

impl From<u32> for FieldElement {
    fn from(value: u32) -> Self {
        Self {
            inner: Fr::from_bigint(BigInteger256::new([value as u64, 0, 0, 0])).unwrap(),
        }
    }
}

impl From<u64> for FieldElement {
    fn from(value: u64) -> Self {
        Self {
            inner: Fr::from_bigint(BigInteger256::new([value, 0, 0, 0])).unwrap(),
        }
    }
}

impl From<u128> for FieldElement {
    fn from(value: u128) -> Self {
        let low = value % (u64::MAX as u128 + 1);
        let high = value / (u64::MAX as u128 + 1);

        Self {
            inner: Fr::from_bigint(BigInteger256::new([low as u64, high as u64, 0, 0])).unwrap(),
        }
    }
}

impl From<usize> for FieldElement {
    fn from(value: usize) -> Self {
        Self {
            inner: Fr::from_bigint(BigInteger256::new([value as u64, 0, 0, 0])).unwrap(),
        }
    }
}

impl FromStr for FieldElement {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            FieldElement::from_hex_be(s)
        } else {
            FieldElement::from_dec_str(s)
        }
    }
}

impl TryFrom<FieldElement> for u8 {
    type Error = ValueOutOfRangeError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        let repr = value.inner.into_bigint().0;
        if repr[0] > u8::MAX as u64 || repr[1] > 0 || repr[2] > 0 || repr[3] > 0 {
            Err(ValueOutOfRangeError)
        } else {
            Ok(repr[0] as u8)
        }
    }
}

impl TryFrom<FieldElement> for u16 {
    type Error = ValueOutOfRangeError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        let repr = value.inner.into_bigint().0;
        if repr[0] > u16::MAX as u64 || repr[1] > 0 || repr[2] > 0 || repr[3] > 0 {
            Err(ValueOutOfRangeError)
        } else {
            Ok(repr[0] as u16)
        }
    }
}

impl TryFrom<FieldElement> for u32 {
    type Error = ValueOutOfRangeError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        let repr = value.inner.into_bigint().0;
        if repr[0] > u32::MAX as u64 || repr[1] > 0 || repr[2] > 0 || repr[3] > 0 {
            Err(ValueOutOfRangeError)
        } else {
            Ok(repr[0] as u32)
        }
    }
}

impl TryFrom<FieldElement> for u64 {
    type Error = ValueOutOfRangeError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        let repr = value.inner.into_bigint().0;
        if repr[1] > 0 || repr[2] > 0 || repr[3] > 0 {
            Err(ValueOutOfRangeError)
        } else {
            Ok(repr[0])
        }
    }
}

impl TryFrom<FieldElement> for u128 {
    type Error = ValueOutOfRangeError;

    fn try_from(value: FieldElement) -> Result<Self, Self::Error> {
        let repr = value.inner.into_bigint().0;
        if repr[2] > 0 || repr[3] > 0 {
            Err(ValueOutOfRangeError)
        } else {
            Ok((repr[0] as u128) + (repr[1] as u128) * (u64::MAX as u128 + 1))
        }
    }
}

impl From<&FieldElement> for U256 {
    #[cfg(target_pointer_width = "64")]
    fn from(value: &FieldElement) -> Self {
        U256::from_words(value.inner.into_bigint().0)
    }

    #[cfg(target_pointer_width = "32")]
    fn from(value: &FieldElement) -> Self {
        U256::from_words(unsafe {
            core::mem::transmute::<[u64; 4], [u32; 8]>(value.inner.into_bigint().0)
        })
    }
}

impl<'a> fmt::Debug for InnerDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#064x}", self.0)
    }
}

#[inline]
fn u256_to_biginteger256(num: &U256) -> BigInteger256 {
    BigInteger256::new(u256_to_u64_array(num))
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn u256_to_u64_array(num: &U256) -> [u64; 4] {
    num.to_words()
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn u256_to_u64_array(num: &U256) -> [u64; 4] {
    unsafe { core::mem::transmute::<[u32; 8], [u64; 4]>(num.to_words()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_default_value() {
        assert_eq!(FieldElement::default(), FieldElement::ZERO)
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_dec_fmt() {
        let nums = [
            "0",
            "1",
            "10",
            "11",
            "3618502788666131213697322783095070105623107215331596699973092056135872020480",
        ];

        for num in nums.iter() {
            assert_eq!(
                &format!("{}", FieldElement::from_dec_str(num).unwrap()),
                num
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_zero_padded_hex_fmt() {
        let fe = FieldElement::from_hex_be("0x1234abcd").unwrap();

        assert_eq!(format!("{fe:011x}"), "0001234abcd");
        assert_eq!(format!("{fe:011X}"), "0001234ABCD");
        assert_eq!(format!("{fe:08x}"), "1234abcd");
        assert_eq!(format!("{fe:06x}"), "1234abcd");
        assert_eq!(format!("{fe:#x}"), "0x1234abcd");
        assert_eq!(
            format!("{fe:#064x}"),
            "0x000000000000000000000000000000000000000000000000000000001234abcd"
        );

        // Ignore if requesting more than 64 nibbles (or should we not?)
        assert_eq!(
            format!("{fe:#0100x}"),
            "0x000000000000000000000000000000000000000000000000000000001234abcd"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
            let mut lhs = FieldElement::from_dec_str(item[0]).unwrap();
            let rhs = FieldElement::from_dec_str(item[1]).unwrap();
            let result = FieldElement::from_dec_str(item[2]).unwrap();
            assert_eq!(lhs + rhs, result);

            lhs += rhs;
            assert_eq!(lhs, result);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
            let mut lhs = FieldElement::from_dec_str(item[0]).unwrap();
            let rhs = FieldElement::from_dec_str(item[1]).unwrap();
            let result = FieldElement::from_dec_str(item[2]).unwrap();
            assert_eq!(lhs - rhs, result);

            lhs -= rhs;
            assert_eq!(lhs, result);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
            let mut lhs = FieldElement::from_dec_str(item[0]).unwrap();
            let rhs = FieldElement::from_dec_str(item[1]).unwrap();
            let result = FieldElement::from_dec_str(item[2]).unwrap();
            assert_eq!(lhs * rhs, result);

            lhs *= rhs;
            assert_eq!(lhs, result);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_remainder() {
        let remainders = [["123456", "100", "56"], ["7", "3", "1"], ["3", "6", "3"]];

        for item in remainders.iter() {
            assert_eq!(
                FieldElement::from_dec_str(item[0]).unwrap()
                    % FieldElement::from_dec_str(item[1]).unwrap(),
                FieldElement::from_dec_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bitwise_and() {
        let operands = [[123456_u64, 567890], [613221132151, 4523451]];

        for item in operands.iter() {
            let lhs: FieldElement = item[0].into();
            let rhs: FieldElement = item[1].into();
            assert_eq!(lhs & rhs, (item[0] & item[1]).into());
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_bitwise_or() {
        let operands = [[123456_u64, 567890], [613221132151, 4523451]];

        for item in operands.iter() {
            let lhs: FieldElement = item[0].into();
            let rhs: FieldElement = item[1].into();
            assert_eq!(lhs | rhs, (item[0] | item[1]).into());
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_iter_sum() {
        let elements = [FieldElement::ONE, FieldElement::TWO, FieldElement::THREE];

        assert_eq!(
            elements.iter().sum::<FieldElement>(),
            FieldElement::from_dec_str("6").unwrap()
        );
        assert_eq!(
            elements.into_iter().sum::<FieldElement>(),
            FieldElement::from_dec_str("6").unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_floor_division() {
        let quotients = [["123456", "100", "1234"], ["7", "3", "2"], ["3", "6", "0"]];

        for item in quotients.iter() {
            assert_eq!(
                FieldElement::from_dec_str(item[0])
                    .unwrap()
                    .floor_div(FieldElement::from_dec_str(item[1]).unwrap()),
                FieldElement::from_dec_str(item[2]).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_to_primitives() {
        let fe_256: FieldElement = 256u16.into();

        if u8::try_from(fe_256).is_ok() {
            panic!("invalid conversion");
        }

        assert_eq!(u16::try_from(fe_256).unwrap(), 256u16);
        assert_eq!(u32::try_from(fe_256).unwrap(), 256u32);
        assert_eq!(u64::try_from(fe_256).unwrap(), 256u64);
    }

    #[test]
    #[cfg(feature = "bigdecimal")]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_to_big_decimal() {
        use bigdecimal::{BigDecimal, Num};

        let nums = [
            (
                "134500",
                5,
                BigDecimal::from_str_radix("1.345", 10).unwrap(),
            ),
            (
                "134500",
                0,
                BigDecimal::from_str_radix("134500", 10).unwrap(),
            ),
            (
                "134500",
                10,
                BigDecimal::from_str_radix("0.00001345", 10).unwrap(),
            ),
        ];

        for num in nums.into_iter() {
            assert_eq!(
                FieldElement::from_dec_str(num.0)
                    .unwrap()
                    .to_big_decimal(num.1),
                num.2
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_from_str() {
        let nums = [
            ("134500", "0x20d64"),
            ("9999999999999999", "0x2386f26fc0ffff"),
        ];

        for num in nums.into_iter() {
            assert_eq!(
                num.0.parse::<FieldElement>().unwrap(),
                num.1.parse::<FieldElement>().unwrap(),
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_from_byte_slice_be() {
        let nums = [("25800", [100u8, 200u8])];

        for num in nums.into_iter() {
            assert_eq!(
                num.0.parse::<FieldElement>().unwrap(),
                FieldElement::from_byte_slice_be(&num.1).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u8_conversion() {
        let nums = [u8::MAX, u8::MAX / 3 * 2, u8::MAX / 3];

        for num in nums.into_iter() {
            let felt: FieldElement = num.into();
            assert_eq!(format!("{}", felt), format!("{}", num));

            let back_to_num: u8 = felt.try_into().unwrap();
            assert_eq!(num, back_to_num);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u16_conversion() {
        let nums = [u16::MAX, u16::MAX / 3 * 2, u16::MAX / 3];

        for num in nums.into_iter() {
            let felt: FieldElement = num.into();
            assert_eq!(format!("{}", felt), format!("{}", num));

            let back_to_num: u16 = felt.try_into().unwrap();
            assert_eq!(num, back_to_num);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u32_conversion() {
        let nums = [u32::MAX, u32::MAX / 3 * 2, u32::MAX / 3];

        for num in nums.into_iter() {
            let felt: FieldElement = num.into();
            assert_eq!(format!("{}", felt), format!("{}", num));

            let back_to_num: u32 = felt.try_into().unwrap();
            assert_eq!(num, back_to_num);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u64_conversion() {
        let nums = [u64::MAX, u64::MAX / 3 * 2, u64::MAX / 3];

        for num in nums.into_iter() {
            let felt: FieldElement = num.into();
            assert_eq!(format!("{}", felt), format!("{}", num));

            let back_to_num: u64 = felt.try_into().unwrap();
            assert_eq!(num, back_to_num);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u128_conversion() {
        let nums = [u128::MAX, u128::MAX / 3 * 2, u128::MAX / 3];

        for num in nums.into_iter() {
            let felt: FieldElement = num.into();
            assert_eq!(format!("{}", felt), format!("{}", num));

            let back_to_num: u128 = felt.try_into().unwrap();
            assert_eq!(num, back_to_num);
        }
    }
}
