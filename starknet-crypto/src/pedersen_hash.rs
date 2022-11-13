use crate::{
    ec_point::{EcPoint, ProjectivePoint},
    pedersen_params::CONSTANT_POINTS,
    FieldElement,
};

const SHIFT_POINT: ProjectivePoint = ProjectivePoint::from_ec_point(&CONSTANT_POINTS[0]);
const PEDERSEN_P0: ProjectivePoint = ProjectivePoint::from_ec_point(&CONSTANT_POINTS[2]);
const PEDERSEN_P1: ProjectivePoint = ProjectivePoint::from_ec_point(&CONSTANT_POINTS[250]);
const PEDERSEN_P2: ProjectivePoint = ProjectivePoint::from_ec_point(&CONSTANT_POINTS[254]);
const PEDERSEN_P3: ProjectivePoint = ProjectivePoint::from_ec_point(&CONSTANT_POINTS[502]);

/// Computes the Starkware version of the Pedersen hash of x and y. All inputs are little-endian.
///
/// ### Arguments
///
/// * `x`: The x coordinate
/// * `y`: The y coordinate
pub fn pedersen_hash(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let x = x.to_bits_le();
    let y = y.to_bits_le();

    // Compute hash
    let accumulator = SHIFT_POINT.add(&PEDERSEN_P0.multiply(&x[..248])); // Add a_low * P1
    let accumulator = accumulator.add(&PEDERSEN_P1.multiply(&x[248..252])); // Add a_high * P2
    let accumulator = accumulator.add(&PEDERSEN_P2.multiply(&y[..248])); // Add b_low * P3
    let accumulator = accumulator.add(&PEDERSEN_P3.multiply(&y[248..252])); // Add b_high * P4

    // Convert to affine
    let result = EcPoint::from(&accumulator);

    // Return x-coordinate
    result.x
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
