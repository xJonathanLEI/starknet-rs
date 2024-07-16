use core::{fmt::Display, str};

use crypto_bigint::{ArrayEncoding, CheckedAdd, CheckedMul, CheckedSub, Zero};

use crate::types::Felt;

/// 256-bit unsiged integer.
///
/// In Cairo, this type is interally represented as two 128-bit words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(crypto_bigint::U256);

impl U256 {
    #[cfg(target_pointer_width = "64")]
    pub const fn from_words(low: u128, high: u128) -> Self {
        Self(crypto_bigint::U256::from_words([
            low as u64,
            (low >> 64) as u64,
            high as u64,
            (high >> 64) as u64,
        ]))
    }

    #[cfg(target_pointer_width = "32")]
    pub const fn from_words(low: u128, high: u128) -> Self {
        Self(crypto_bigint::U256::from_words([
            low as u32,
            (low >> 32) as u32,
            (low >> 64) as u32,
            (low >> 96) as u32,
            high as u32,
            (high >> 32) as u32,
            (high >> 64) as u32,
            (high >> 96) as u32,
        ]))
    }

    pub const fn low(&self) -> u128 {
        let words = u256_to_u64_array(&self.0);
        words[0] as u128 + ((words[1] as u128) << 64)
    }

    pub const fn high(&self) -> u128 {
        let words = u256_to_u64_array(&self.0);
        words[2] as u128 + ((words[3] as u128) << 64)
    }
}

impl core::ops::Add<Self> for U256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_add(&rhs.0).unwrap())
    }
}

impl core::ops::AddAssign<Self> for U256 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_add(&rhs.0).unwrap()
    }
}

impl core::ops::Sub<Self> for U256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_sub(&rhs.0).unwrap())
    }
}

impl core::ops::SubAssign<Self> for U256 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_sub(&rhs.0).unwrap()
    }
}

impl core::ops::Mul<Self> for U256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_mul(&rhs.0).unwrap())
    }
}

impl core::ops::MulAssign<Self> for U256 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_mul(&rhs.0).unwrap()
    }
}

impl core::ops::Div<Self> for U256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_div(&rhs.0).unwrap())
    }
}

impl core::ops::DivAssign<Self> for U256 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_div(&rhs.0).unwrap()
    }
}

impl core::ops::Rem<Self> for U256 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_rem(&rhs.0).unwrap())
    }
}

impl core::ops::RemAssign<Self> for U256 {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_rem(&rhs.0).unwrap()
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_zero().into() {
            return write!(f, "0");
        }

        let mut buf = [0u8; 4 * 20];
        let mut i = buf.len() - 1;
        let mut current = self.0;
        let ten = crypto_bigint::U256::from_u8(10u8);

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

impl core::fmt::LowerHex for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr = self.0;

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

impl core::fmt::UpperHex for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr = self.0;

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

impl From<u8> for U256 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<u16> for U256 {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<u32> for U256 {
    fn from(value: u32) -> Self {
        Self(value.into())
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self(value.into())
    }
}

impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        Self(value.into())
    }
}

impl From<crypto_bigint::U256> for U256 {
    fn from(value: crypto_bigint::U256) -> Self {
        Self(value)
    }
}

impl From<Felt> for U256 {
    fn from(value: Felt) -> Self {
        Self(crypto_bigint::U256::from_be_byte_array(
            value.to_bytes_be().into(),
        ))
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
const fn u256_to_u64_array(num: &crypto_bigint::U256) -> [u64; 4] {
    num.to_words()
}

#[cfg(target_pointer_width = "32")]
#[inline]
const fn u256_to_u64_array(num: &crypto_bigint::U256) -> [u64; 4] {
    unsafe { core::mem::transmute::<[u32; 8], [u64; 4]>(num.to_words()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_new_u256() {
        let u256_value: U256 = crypto_bigint::U256::from_u128(123).into();

        assert_eq!(u256_value, 123u8.into());
        assert_eq!(u256_value, 123u16.into());
        assert_eq!(u256_value, 123u32.into());
        assert_eq!(u256_value, 123u64.into());
        assert_eq!(u256_value, 123u128.into());
        assert_eq!(u256_value, Felt::from_dec_str("123").unwrap().into());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_from_words() {
        assert_eq!(
            U256::from(crypto_bigint::U256::from_be_hex(
                "0000000000112233445566778899112233445566778899112233445566778899"
            )),
            U256::from_words(
                0x33445566778899112233445566778899u128,
                0x00000000001122334455667788991122u128
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_to_words() {
        let u256_value = U256::from(crypto_bigint::U256::from_be_hex(
            "0000000000112233445566778899112233445566778899112233445566778899",
        ));

        assert_eq!(u256_value.low(), 0x33445566778899112233445566778899u128);
        assert_eq!(u256_value.high(), 0x00000000001122334455667788991122u128);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_to_dec_str() {
        assert_eq!(
            format!(
                "{}",
                U256::from(crypto_bigint::U256::from_be_hex(
                    "00000000a0112233445566778899112233445566778899112233445566778899",
                ))
            ),
            "16857015019038993176188352621511919890733941357973158131750047025305"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_to_hex_str() {
        let u256_value = U256::from(crypto_bigint::U256::from_be_hex(
            "00000000a0112233445566778899112233445566778899112233445566778899",
        ));

        assert_eq!(
            format!("{:x}", u256_value),
            "a0112233445566778899112233445566778899112233445566778899"
        );
        assert_eq!(
            format!("{:X}", u256_value),
            "A0112233445566778899112233445566778899112233445566778899"
        );
        assert_eq!(
            format!("{:#x}", u256_value),
            "0xa0112233445566778899112233445566778899112233445566778899"
        );
        assert_eq!(
            format!("{:#X}", u256_value),
            "0xA0112233445566778899112233445566778899112233445566778899"
        );
        assert_eq!(
            format!("{:#062x}", u256_value),
            "0x000000a0112233445566778899112233445566778899112233445566778899"
        );
        assert_eq!(
            format!("{:#062X}", u256_value),
            "0x000000A0112233445566778899112233445566778899112233445566778899"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_add() {
        assert_eq!(U256::from(100u32) + U256::from(3u32), U256::from(103u32));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_sub() {
        assert_eq!(U256::from(100u32) - U256::from(3u32), U256::from(97u32));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_mul() {
        assert_eq!(U256::from(100u32) * U256::from(3u32), U256::from(300u32));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_div() {
        assert_eq!(U256::from(100u32) / U256::from(3u32), U256::from(33u32));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_u256_rem() {
        assert_eq!(U256::from(100u32) % U256::from(3u32), U256::from(1u32));
    }
}
