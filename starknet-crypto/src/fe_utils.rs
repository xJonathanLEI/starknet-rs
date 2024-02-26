use core::ops::{Add, Mul};

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

use crate::FieldElement;

// These are inefficient and crappy implementations of crypto math operations because I have
// absolutely no idea how to do them without using `num-bigint`. But hey it works!!!
//
// Contributions are welcome. Please help us get rid of this junk :)

pub fn add_unbounded(augend: &FieldElement, addend: &FieldElement) -> BigInt {
    let augend = BigInt::from_bytes_be(num_bigint::Sign::Plus, &augend.to_bytes_be());
    let addend = BigInt::from_bytes_be(num_bigint::Sign::Plus, &addend.to_bytes_be());
    augend.add(addend)
}

pub fn mul_mod_floor(
    multiplicand: &FieldElement,
    multiplier: &FieldElement,
    modulus: &FieldElement,
) -> FieldElement {
    let multiplicand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &multiplicand.to_bytes_be());
    bigint_mul_mod_floor(multiplicand, multiplier, modulus)
}

pub fn bigint_mul_mod_floor(
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

pub fn mod_inverse(operand: &FieldElement, modulus: &FieldElement) -> FieldElement {
    let operand = BigInt::from_bytes_be(num_bigint::Sign::Plus, &operand.to_bytes_be());
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
