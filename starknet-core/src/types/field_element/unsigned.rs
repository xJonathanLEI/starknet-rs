use ethereum_types::U256;
use starknet_crypto::FieldElement;
use std::fmt::{Display, LowerHex, UpperHex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsignedFieldElement {
    inner: U256,
}

#[derive(Debug, thiserror::Error)]
pub enum FromUintError<U> {
    #[error("number out of range: {0}")]
    OutOfRange(U),
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

impl TryFrom<U256> for UnsignedFieldElement {
    type Error = FromUintError<U256>;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        if value > Self::MAX.inner {
            Err(FromUintError::OutOfRange(value))
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
}
