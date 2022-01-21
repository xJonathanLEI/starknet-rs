#![allow(clippy::too_many_arguments)]

use bitvec::{array::BitArray, order::Lsb0};
use ff::PrimeField;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::ops::Mul;

#[derive(PrimeField)]
#[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "big"]
pub struct FieldElement([u64; 4]);

impl FieldElement {
    pub const fn new(data: [u64; 4]) -> Self {
        Self(data)
    }

    /// Transforms [FieldElement] into little endian bit representation.
    pub fn into_bits(mut self) -> BitArray<Lsb0, [u64; 4]> {
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
    pub fn mul_mod_floor(&self, multiplier: &FieldElement, modulus: &FieldElement) -> FieldElement {
        let multiplicand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &self.to_repr().0);
        let multiplier = BigInt::from_bytes_be(num_bigint::Sign::Plus, &multiplier.to_repr().0);
        let modulus = BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus.to_repr().0);

        let result = multiplicand.mul(multiplier).mod_floor(&modulus);

        let (_, buffer) = result.to_bytes_be();
        let mut result = [0u8; 32];
        result[0..buffer.len()].copy_from_slice(&buffer[..]);

        FieldElement::from_repr(FieldElementRepr(result)).unwrap()
    }

    // Hard-coded to use big-endian because `FieldElement` uses it
    pub fn mod_inverse(&self, modulus: &FieldElement) -> FieldElement {
        let operand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &self.to_repr().0);
        let modulus = BigInt::from_bytes_be(num_bigint::Sign::Plus, &modulus.to_repr().0);

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
        result[0..buffer.len()].copy_from_slice(&buffer[..]);

        FieldElement::from_repr(FieldElementRepr(result)).unwrap()
    }
}
