use starknet_curve::{
    curve_params::{EC_ORDER, GENERATOR},
    AffinePoint,
};

use crate::{
    fe_utils::{add_unbounded, bigint_mul_mod_floor, mod_inverse, mul_mod_floor},
    FieldElement, SignError, VerifyError,
};
use std::fmt;

const ELEMENT_UPPER_BOUND: FieldElement = FieldElement::from_mont([
    18446743986131435553,
    160989183,
    18446744073709255680,
    576459263475450960,
]);

/// Stark ECDSA signature
#[derive(Debug)]
pub struct Signature {
    /// The `r` value of a signature
    pub r: FieldElement,
    /// The `s` value of a signature
    pub s: FieldElement,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            hex::encode(self.r.to_bytes_be()),
            hex::encode(self.s.to_bytes_be()),
        )
    }
}

/// Computes the public key given a Stark private key.
///
/// ### Arguments
///
/// * `private_key`: The private key
pub fn get_public_key(private_key: &FieldElement) -> FieldElement {
    (&GENERATOR * &private_key.to_bits_le()).x
}

/// Computes ECDSA signature given a Stark private key and message hash.
///
/// ### Arguments
///
/// * `private_key`: The private key
/// * `message`: The message hash
/// * `k`: A random `k` value. You **MUST NOT** use the same `k` on different signatures
pub fn sign(
    private_key: &FieldElement,
    message: &FieldElement,
    k: &FieldElement,
) -> Result<Signature, SignError> {
    if message >= &ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidMessageHash);
    }
    if k == &FieldElement::ZERO {
        return Err(SignError::InvalidK);
    }

    let r = (&GENERATOR * &k.to_bits_le()).x;
    if r == FieldElement::ZERO || r >= ELEMENT_UPPER_BOUND {
        return Err(SignError::InvalidK);
    }

    let k_inv = mod_inverse(k, &EC_ORDER);

    let s = mul_mod_floor(&r, private_key, &EC_ORDER);
    let s = add_unbounded(&s, message);
    let s = bigint_mul_mod_floor(s, &k_inv, &EC_ORDER);
    if s == FieldElement::ZERO || s >= EC_ORDER {
        return Err(SignError::InvalidK);
    }

    Ok(Signature { r, s })
}

/// Verifies if a signature is valid over a message hash given a Stark public key.
///
/// ### Arguments
///
/// * `stark_key`: The public key
/// * `msg_hash`: The message hash
/// * `r_bytes`: The `r` value of the signature
/// * `s_bytes`: The `s` value of the signature
pub fn verify(
    public_key: &FieldElement,
    message: &FieldElement,
    r: &FieldElement,
    s: &FieldElement,
) -> Result<bool, VerifyError> {
    if message >= &ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidMessageHash);
    }
    if r == &FieldElement::ZERO || r >= &ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidR);
    }
    if s == &FieldElement::ZERO || s >= &EC_ORDER {
        return Err(VerifyError::InvalidS);
    }

    let full_public_key = AffinePoint::from_x(*public_key);

    let w = mod_inverse(s, &EC_ORDER);
    if w == FieldElement::ZERO || w >= ELEMENT_UPPER_BOUND {
        return Err(VerifyError::InvalidS);
    }

    let zw = mul_mod_floor(message, &w, &EC_ORDER);
    let zw_g = &GENERATOR * &zw.to_bits_le();

    let rw = mul_mod_floor(r, &w, &EC_ORDER);
    let rw_q = &full_public_key * &rw.to_bits_le();

    Ok((&zw_g + &rw_q).x == *r || (&zw_g - &rw_q).x == *r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    // Test cases ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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

        assert!(verify(&stark_key, &msg_hash, &r_bytes, &s_bytes).unwrap());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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

        assert!(!verify(&stark_key, &msg_hash, &r_bytes, &s_bytes).unwrap());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
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

        assert!(verify(&public_key, &message, &signature.r, &signature.s).unwrap());
    }
}
