#![allow(clippy::too_many_arguments)]

use ark_ff::{fields::Fp256, BigInteger, BigInteger256, Field, PrimeField, SquareRootField};
use bitvec::{array::BitArray, order::Lsb0};

use crate::fr::FrParameters;

mod fr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FieldElement {
    inner: Fp256<FrParameters>,
}

impl FieldElement {
    pub const ZERO: FieldElement = FieldElement::from_mont([0, 0, 0, 0]);
    pub const ONE: FieldElement = FieldElement::from_mont([
        18446744073709551585,
        18446744073709551615,
        18446744073709551615,
        576460752303422960,
    ]);

    /// Create a new [FieldElement] from its Montgomery representation
    pub const fn from_mont(data: [u64; 4]) -> Self {
        Self {
            inner: Fp256::new(BigInteger256::new(data)),
        }
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub fn to_bits_le(self) -> BitArray<Lsb0, [u64; 4]> {
        BitArray::<Lsb0, [u64; 4]>::new(self.inner.into_repr().0)
    }

    /// Attempts to convert a big-endian byte representation of a field element into an element of
    /// this prime field. Returns None if the input is not canonical (is not smaller than the
    /// field's modulus).
    ///
    /// ### Arguments
    ///
    /// * `bytes`: The byte array in **big endian** format
    pub fn from_bytes_be(bytes: &[u8; 32]) -> Option<Self> {
        let mut bits = [false; 32 * 8];
        for (ind_byte, byte) in bytes.iter().enumerate() {
            for ind_bit in 0..8 {
                bits[ind_byte * 8 + ind_bit] = (byte >> (7 - ind_bit)) & 1 == 1;
            }
        }

        // No need to check range as `from_repr` already does that
        let big_int = BigInteger256::from_bits_be(&bits);
        Fp256::<FrParameters>::from_repr(big_int).map(|inner| Self { inner })
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
