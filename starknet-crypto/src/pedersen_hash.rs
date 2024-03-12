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
                let offset = v
                    .iter()
                    .rev()
                    .fold(0, |acc, &bit| (acc << 1) + bit as usize);

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

    // Return x-coordinate
    AffinePoint::from(&acc).x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    // Test case ported from:
    //   https://github.com/starkware-libs/starkex-for-spot-trading/blob/master/src/starkware/crypto/starkware/crypto/signature/test/config/signature_test_data.json

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_pedersen_hash_1() {
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

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_pedersen_hash_2() {
        let in1 = field_element_from_be_hex(
            "058f580910a6ca59b28927c08fe6c43e2e303ca384badc365795fc645d479d45",
        );
        let in2 = field_element_from_be_hex(
            "078734f65a067be9bdb39de18434d71e79f7b6466a4b66bbd979ab9e7515fe0b",
        );
        let expected_hash = field_element_from_be_hex(
            "068cc0b76cddd1dd4ed2301ada9b7c872b23875d5ff837b3a87993e0d9996b87",
        );

        assert_eq!(pedersen_hash(&in1, &in2), expected_hash);
    }
}
