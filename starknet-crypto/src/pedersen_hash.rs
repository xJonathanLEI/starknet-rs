use crate::{ec_point::EcPoint, field_element::FieldElement, pedersen_params::CONSTANT_POINTS};

const SHIFT_POINT: EcPoint = CONSTANT_POINTS[0];
const PEDERSEN_P0: EcPoint = CONSTANT_POINTS[2];
const PEDERSEN_P1: EcPoint = CONSTANT_POINTS[250];
const PEDERSEN_P2: EcPoint = CONSTANT_POINTS[254];
const PEDERSEN_P3: EcPoint = CONSTANT_POINTS[502];

/// Computes the [Starknet Pedersen hash] on `x` and `y`.
///
/// [Starknet Pedersen hash]: https://docs.starkware.co/starkex-v3/crypto/pedersen-hash-function
pub fn pedersen_hash(x: &FieldElement, y: &FieldElement) -> FieldElement {
    let mut result = SHIFT_POINT;
    let x = x.into_bits();
    let y = y.into_bits();

    // Add a_low * P1
    let tmp = PEDERSEN_P0.multiply(&x[..248]);
    result = result.add(&tmp);

    // Add a_high * P2
    let tmp = PEDERSEN_P1.multiply(&x[248..252]);
    result = result.add(&tmp);

    // Add b_low * P3
    let tmp = PEDERSEN_P2.multiply(&y[..248]);
    result = result.add(&tmp);

    // Add b_high * P4
    let tmp = PEDERSEN_P3.multiply(&y[248..252]);
    result = result.add(&tmp);

    // Return x-coordinate
    result.x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_element::{FieldElement, FieldElementRepr};

    use ff::PrimeField;
    use hex_literal::hex;

    // Test case ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    fn test_pedersen_hash() {
        let in1 = hex!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
        let in2 = hex!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");
        let expected_hash =
            hex!("030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662");

        let in1 = FieldElement::from_repr(FieldElementRepr(in1)).unwrap();
        let in2 = FieldElement::from_repr(FieldElementRepr(in2)).unwrap();
        let expected_hash = FieldElement::from_repr(FieldElementRepr(expected_hash)).unwrap();

        assert_eq!(pedersen_hash(&in1, &in2), expected_hash);
    }
}
