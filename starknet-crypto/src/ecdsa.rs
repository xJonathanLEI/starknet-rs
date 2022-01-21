use crate::{
    pedersen_params::{CONSTANT_POINTS, EC_ORDER},
    FieldElement,
};

pub struct Signature {
    pub r: FieldElement,
    pub s: FieldElement,
}

pub fn get_public_key(private_key: &FieldElement) -> FieldElement {
    (&CONSTANT_POINTS[1]).multiply(&private_key.into_bits()).x
}

pub fn sign(private_key: &FieldElement, message: &FieldElement, k: &FieldElement) -> Signature {
    let generator = &CONSTANT_POINTS[1];

    let r = generator.multiply(&k.into_bits()).x;

    let k_inv = k.mod_inverse(&EC_ORDER);

    let s = r.mul_mod_floor(private_key, &EC_ORDER);
    let s = s + message;
    let s = s.mul_mod_floor(&k_inv, &EC_ORDER);

    Signature { r, s }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_element::{FieldElement, FieldElementRepr};

    use ff::PrimeField;
    use hex_literal::hex;

    // Test cases ported from:
    //   https://github.com/starkware-libs/crypto-cpp/blob/95864fbe11d5287e345432dbe1e80dea3c35fc58/src/starkware/crypto/ffi/crypto_lib_test.go

    #[test]
    fn test_get_public_key_1() {
        let private_key = hex!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let expected_key = hex!("077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43");

        let private_key = FieldElement::from_repr(FieldElementRepr(private_key)).unwrap();
        let expected_key = FieldElement::from_repr(FieldElementRepr(expected_key)).unwrap();

        assert_eq!(get_public_key(&private_key), expected_key);
    }

    #[test]
    fn test_get_public_key_2() {
        let private_key = hex!("0000000000000000000000000000000000000000000000000000000000000012");
        let expected_key = hex!("019661066e96a8b9f06a1d136881ee924dfb6a885239caa5fd3f87a54c6b25c4");

        let private_key = FieldElement::from_repr(FieldElementRepr(private_key)).unwrap();
        let expected_key = FieldElement::from_repr(FieldElementRepr(expected_key)).unwrap();

        assert_eq!(get_public_key(&private_key), expected_key);
    }

    #[test]
    fn test_sign() {
        let private_key = hex!("0000000000000000000000000000000000000000000000000000000000000001");
        let message = hex!("0000000000000000000000000000000000000000000000000000000000000002");
        let k = hex!("0000000000000000000000000000000000000000000000000000000000000003");
        let expected_r = hex!("0411494b501a98abd8262b0da1351e17899a0c4ef23dd2f96fec5ba847310b20");
        let expected_s = hex!("0405c3191ab3883ef2b763af35bc5f5d15b3b4e99461d70e84c654a351a7c81b");

        let private_key = FieldElement::from_repr(FieldElementRepr(private_key)).unwrap();
        let message = FieldElement::from_repr(FieldElementRepr(message)).unwrap();
        let k = FieldElement::from_repr(FieldElementRepr(k)).unwrap();
        let expected_r = FieldElement::from_repr(FieldElementRepr(expected_r)).unwrap();
        let expected_s = FieldElement::from_repr(FieldElementRepr(expected_s)).unwrap();

        let signature = sign(&private_key, &message, &k);

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }
}
