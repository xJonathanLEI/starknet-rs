#![allow(clippy::too_many_arguments)]

use bitvec::{array::BitArray, order::Lsb0};
use ff::PrimeField;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};

/// Field element for the Stark curve with big-endian encoding.
#[derive(PrimeField)]
#[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "big"]
pub struct FieldElement([u64; 4]);

impl FieldElement {
    pub const ZERO: FieldElement = FieldElement([0, 0, 0, 0]);
    pub const ONE: FieldElement = FieldElement([
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
    pub fn from_bytes_be(bytes: [u8; 32]) -> Option<Self> {
        let result = FieldElement::from_repr(FieldElementRepr(bytes));
        if result.is_some().into() {
            Some(result.unwrap())
        } else {
            None
        }
    }

    /// Convert the field element into a big-endian byte representation
    pub fn to_bytes_be(&self) -> [u8; 32] {
        self.to_repr().0
    }
}

impl FieldElement {
    pub(crate) const fn new(data: [u64; 4]) -> Self {
        Self(data)
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub(crate) fn into_bits(mut self) -> BitArray<Lsb0, [u64; 4]> {
        #[cfg(not(target_endian = "little"))]
        {
            todo!("untested and probably unimplemented: big-endian targets")
        }

        #[cfg(target_endian = "little")]
        {
            self.mont_reduce(
                self.0[0usize],
                self.0[1usize],
                self.0[2usize],
                self.0[3usize],
                0,
                0,
                0,
                0,
            );

            self.0.into()
        }
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

        FieldElement::from_repr(FieldElementRepr(result)).unwrap()
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

        FieldElement::from_repr(FieldElementRepr(result)).unwrap()
    }
}
