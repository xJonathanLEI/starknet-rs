#![allow(clippy::too_many_arguments)]

use bitvec::{array::BitArray, order::Lsb0};
use ff::PrimeField;

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
