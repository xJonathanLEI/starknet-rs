#![allow(clippy::too_many_arguments)]

use crate::field_element::fr::FrParameters;
use ark_ff::{fields::Fp256, BigInteger, BigInteger256, Field, PrimeField, SquareRootField};
use bitvec::{array::BitArray, order::Lsb0};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};

mod fr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FieldElement {
    inner: Fp256<FrParameters>,
}

impl FieldElement {
    pub const ZERO: FieldElement = FieldElement::new([0, 0, 0, 0]);
    pub const ONE: FieldElement = FieldElement::new([
        18446744073709551585,
        18446744073709551615,
        18446744073709551615,
        576460752303422960,
    ]);

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

impl FieldElement {
    pub(crate) const fn new(data: [u64; 4]) -> Self {
        Self {
            inner: Fp256::new(BigInteger256::new(data)),
        }
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub(crate) fn to_bits_le(self) -> BitArray<Lsb0, [u64; 4]> {
        BitArray::<Lsb0, [u64; 4]>::new(self.inner.into_repr().0)
    }
}

// These are inefficient and crappy implementations of crypto math operations because I have
// absolutely no idea how to do them without using `num-bigint`. But hey it works!!!
//
// Contributions are welcome. Please help us get rid of this junk :)
impl FieldElement {
    // Hard-coded to use big-endian because `FieldElement` uses it
    pub(crate) fn add_unbounded(&self, addend: &FieldElement) -> BigInt {
        let augend = BigInt::from_bytes_be(num_bigint::Sign::Plus, &self.to_bytes_be());
        let addend = BigInt::from_bytes_be(num_bigint::Sign::Plus, &addend.to_bytes_be());
        augend.add(addend)
    }

    // Hard-coded to use big-endian because `FieldElement` uses it
    pub(crate) fn mul_mod_floor(
        &self,
        multiplier: &FieldElement,
        modulus: &FieldElement,
    ) -> FieldElement {
        let multiplicand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &self.to_bytes_be());
        Self::bigint_mul_mod_floor(multiplicand, multiplier, modulus)
    }

    pub(crate) fn bigint_mul_mod_floor(
        multiplicand: BigInt,
        multiplier: &FieldElement,
        modulus: &FieldElement,
    ) -> FieldElement {
        let multiplier = BigInt::from_bytes_be(num_bigint::Sign::Plus, &multiplier.to_bytes_be());
        let modulus = BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus.to_bytes_be());

        let result = multiplicand.mul(multiplier).mod_floor(&modulus);

        let (_, buffer) = result.to_bytes_be();
        let mut result = [0u8; 32];
        result[(32 - buffer.len())..].copy_from_slice(&buffer[..]);

        FieldElement::from_bytes_be(&result).unwrap()
    }

    // Hard-coded to use big-endian because `FieldElement` uses it
    pub(crate) fn mod_inverse(&self, modulus: &FieldElement) -> FieldElement {
        let operand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &self.to_bytes_be());
        let modulus = BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus.to_bytes_be());

        // Ported from:
        //   https://github.com/dignifiedquire/num-bigint/blob/56576b592fea6341b7e1711a1629e4cc1bfc419c/src/algorithms/mod_inverse.rs#L11
        let extended_gcd = operand.extended_gcd(&modulus);
        if extended_gcd.gcd != BigInt::one() {
            panic!("GCD must be one");
        }
        let result = if extended_gcd.x < BigInt::zero() {
            extended_gcd.x + modulus
        } else {
            extended_gcd.x
        };

        let (_, buffer) = result.to_bytes_be();
        let mut result = [0u8; 32];
        result[(32 - buffer.len())..].copy_from_slice(&buffer[..]);

        FieldElement::from_bytes_be(&result).unwrap()
    }
}
