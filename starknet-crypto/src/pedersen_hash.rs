use starknet_curve::{curve_params, AffinePoint, ProjectivePoint};
use starknet_ff::FieldElement;

use crate::pedersen_points::*;

const SHIFT_POINT: ProjectivePoint = ProjectivePoint::from_affine_point(&curve_params::SHIFT_POINT);

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
///
/// ### Arguments
///
/// * `x`: The x coordinate
/// * `y`: The y coordinate
pub fn pedersen_hash(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let x = x.to_bits_le();
    let y = y.to_bits_le();

    // Preprocessed material is lookup-tables for each chunk of bits
    let table_size = (1 << CURVE_CONSTS_BITS) - 1;
    let add_points = |acc: &mut ProjectivePoint, bits: &[bool], prep: &[AffinePoint]| {
        bits.chunks(CURVE_CONSTS_BITS)
            .enumerate()
            .for_each(|(i, v)| {
                let offset = bools_to_usize_le(v);
                if offset > 0 {
                    // Table lookup at 'offset-1' in table for chunk 'i'
                    *acc += &prep[i * table_size + offset - 1];
                }
            });
    };

    // Compute hash
    let mut acc = SHIFT_POINT;
    add_points(&mut acc, &x[..248], &CURVE_CONSTS_P0); // Add a_low * P1
    add_points(&mut acc, &x[248..252], &CURVE_CONSTS_P1); // Add a_high * P2
    add_points(&mut acc, &y[..248], &CURVE_CONSTS_P2); // Add b_low * P3
    add_points(&mut acc, &y[248..252], &CURVE_CONSTS_P3); // Add b_high * P4

    // Convert to affine
    let result = AffinePoint::from(&acc);

    // Return x-coordinate
    result.x
}

#[inline]
fn bools_to_usize_le(bools: &[bool]) -> usize {
    let mut result: usize = 0;
    for (ind, bit) in bools.iter().enumerate() {
        if *bit {
            result += 1 << ind;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    // Test case ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_pedersen_hash() {
        let in1 = field_element_from_be_hex(
            "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        );
        let in2 = field_element_from_be_hex(
            "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
        );
        let expected_hash = field_element_from_be_hex(
            "030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662",
        );

        assert_eq!(pedersen_hash(&in1, &in2), expected_hash);
    }
}
