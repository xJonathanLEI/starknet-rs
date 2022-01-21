use crate::{
    ec_point::EcPoint,
    pedersen_params::{CONSTANT_POINTS, EC_ORDER},
    FieldElement, SignError, VerifyError,
};

const FIELD_ELEMENT_ZERO: FieldElement = FieldElement::new([0, 0, 0, 0]);
const ELEMENT_UPPER_BOUND: FieldElement = FieldElement::new([
    18446743986131435553,
    160989183,
    18446744073709255680,
    576459263475450960,
]);

pub struct Signature {
    pub r: FieldElement,
    pub s: FieldElement,
}

pub fn get_public_key(private_key: &FieldElement) -> FieldElement {
    (&CONSTANT_POINTS[1]).multiply(&private_key.into_bits()).x
}

pub fn sign(
    private_key: &FieldElement,
    message: &FieldElement,
    k: &FieldElement,
) -> Result<Signature, SignError> {
    if message >= &ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidMessageHash);
    }
    if k == &FIELD_ELEMENT_ZERO {
        return Err(SignError::InvalidK);
    }

    let generator = &CONSTANT_POINTS[1];

    let r = generator.multiply(&k.into_bits()).x;
    if r == FIELD_ELEMENT_ZERO || r >= ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidK);
    }

    let k_inv = k.mod_inverse(&EC_ORDER);

    let s = r.mul_mod_floor(private_key, &EC_ORDER);
    let s = s.add_unbounded(message);
    let s = FieldElement::bigint_mul_mod_floor(s, &k_inv, &EC_ORDER);
    if s == FIELD_ELEMENT_ZERO || s >= EC_ORDER {
        return Err(SignError::InvalidK);
    }

    Ok(Signature { r, s })
}

pub fn verify(
    public_key: &FieldElement,
    message: &FieldElement,
    r: &FieldElement,
    s: &FieldElement,
) -> Result<bool, VerifyError> {
    if message >= &ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidMessageHash);
    }
    if r == &FIELD_ELEMENT_ZERO || r >= &ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidR);
    }
    if s == &FIELD_ELEMENT_ZERO || s >= &EC_ORDER {
        return Err(VerifyError::InvalidS);
    }

    let full_public_key = EcPoint::from_x(*public_key);

    let generator = &CONSTANT_POINTS[1];

    let w = s.mod_inverse(&EC_ORDER);
    if w == FIELD_ELEMENT_ZERO || w >= ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidS);
    }

    let zw = message.mul_mod_floor(&w, &EC_ORDER);
    let zw_g = generator.multiply(&zw.into_bits());

    let rw = r.mul_mod_floor(&w, &EC_ORDER);
    let rw_q = full_public_key.multiply(&rw.into_bits());

    Ok(zw_g.add(&rw_q).x == *r || zw_g.subtract(&rw_q).x == *r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    // Test cases ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    fn test_get_public_key_1() {
        let private_key = field_element_from_be_hex(
            "03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc",
        );
        let expected_key = field_element_from_be_hex(
            "077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43",
        );

        assert_eq!(get_public_key(&private_key), expected_key);
    }

    #[test]
    fn test_get_public_key_2() {
        let private_key = field_element_from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000012",
        );
        let expected_key = field_element_from_be_hex(
            "019661066e96a8b9f06a1d136881ee924dfb6a885239caa5fd3f87a54c6b25c4",
        );

        assert_eq!(get_public_key(&private_key), expected_key);
    }

    #[test]
    fn test_verify_valid_message() {
        let stark_key = field_element_from_be_hex(
            "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca",
        );
        let msg_hash = field_element_from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000002",
        );
        let r_bytes = field_element_from_be_hex(
            "0411494b501a98abd8262b0da1351e17899a0c4ef23dd2f96fec5ba847310b20",
        );
        let s_bytes = field_element_from_be_hex(
            "0405c3191ab3883ef2b763af35bc5f5d15b3b4e99461d70e84c654a351a7c81b",
        );

        assert_eq!(
            verify(&stark_key, &msg_hash, &r_bytes, &s_bytes).unwrap(),
            true
        );
    }

    #[test]
    fn test_verify_invalid_message() {
        let stark_key = field_element_from_be_hex(
            "077a4b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43",
        );
        let msg_hash = field_element_from_be_hex(
            "0397e76d1667c4454bfb83514e120583af836f8e32a516765497823eabe16a3f",
        );
        let r_bytes = field_element_from_be_hex(
            "0173fd03d8b008ee7432977ac27d1e9d1a1f6c98b1a2f05fa84a21c84c44e882",
        );
        let s_bytes = field_element_from_be_hex(
            "01f2c44a7798f55192f153b4c48ea5c1241fbb69e6132cc8a0da9c5b62a4286e",
        );

        assert_eq!(
            verify(&stark_key, &msg_hash, &r_bytes, &s_bytes).unwrap(),
            false
        );
    }

    #[test]
    fn test_sign() {
        let private_key = field_element_from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000001",
        );
        let message = field_element_from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000002",
        );
        let k = field_element_from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000003",
        );

        let signature = sign(&private_key, &message, &k).unwrap();
        let public_key = get_public_key(&private_key);

        assert_eq!(
            verify(&public_key, &message, &signature.r, &signature.s).unwrap(),
            true
        );
    }
}
