use crate::types::FieldElement;

pub use starknet_crypto::{pedersen_hash, ExtendedSignature, Signature};
use starknet_crypto::{rfc6979_generate_k, sign, verify, SignError, VerifyError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EcdsaSignError {
    #[error("message hash out of range")]
    MessageHashOutOfRange,
}

#[derive(Debug, Error)]
pub enum EcdsaVerifyError {
    #[error("message hash out of range")]
    MessageHashOutOfRange,
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("signature r value out of range")]
    SignatureROutOfRange,
    #[error("signature s value out of range")]
    SignatureSOutOfRange,
}

pub fn compute_hash_on_elements(data: &[FieldElement]) -> FieldElement {
    let mut current_hash = FieldElement::ZERO;

    for item in data.iter() {
        current_hash = pedersen_hash(&current_hash, item);
    }

    let data_len = FieldElement::from(data.len());
    pedersen_hash(&current_hash, &data_len)
}

pub fn ecdsa_sign(
    private_key: &FieldElement,
    message_hash: &FieldElement,
) -> Result<ExtendedSignature, EcdsaSignError> {
    // Seed-retry logic ported from `cairo-lang`
    let mut seed = None;
    loop {
        let k = rfc6979_generate_k(message_hash, private_key, seed.as_ref());

        match sign(private_key, message_hash, &k) {
            Ok(sig) => {
                return Ok(sig);
            }
            Err(SignError::InvalidMessageHash) => {
                return Err(EcdsaSignError::MessageHashOutOfRange)
            }
            Err(SignError::InvalidK) => {
                // Bump seed and retry
                seed = match seed {
                    Some(prev_seed) => Some(prev_seed + FieldElement::ONE),
                    None => Some(FieldElement::ONE),
                };
            }
        };
    }
}

pub fn ecdsa_verify(
    public_key: &FieldElement,
    message_hash: &FieldElement,
    signature: &Signature,
) -> Result<bool, EcdsaVerifyError> {
    match verify(public_key, message_hash, &signature.r, &signature.s) {
        Ok(result) => Ok(result),
        Err(VerifyError::InvalidMessageHash) => Err(EcdsaVerifyError::MessageHashOutOfRange),
        Err(VerifyError::InvalidPublicKey) => Err(EcdsaVerifyError::InvalidPublicKey),
        Err(VerifyError::InvalidR) => Err(EcdsaVerifyError::SignatureROutOfRange),
        Err(VerifyError::InvalidS) => Err(EcdsaVerifyError::SignatureSOutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compute_hash_on_elements() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[
            FieldElement::from_hex_be("0xaa").unwrap(),
            FieldElement::from_hex_be("0xbb").unwrap(),
            FieldElement::from_hex_be("0xcc").unwrap(),
            FieldElement::from_hex_be("0xdd").unwrap(),
        ]);
        let expected_hash = FieldElement::from_hex_be(
            "025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69",
        )
        .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compute_hash_on_elements_empty_data() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[]);
        let expected_hash = FieldElement::from_hex_be(
            "049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804",
        )
        .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_ecdsa_sign() {
        // Generated with `cairo-lang`
        let signature = ecdsa_sign(
            &FieldElement::from_hex_be(
                "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
            )
            .unwrap(),
            &FieldElement::from_hex_be(
                "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
            )
            .unwrap(),
        )
        .unwrap();
        let expected_r = FieldElement::from_hex_be(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let expected_s = FieldElement::from_hex_be(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a",
        )
        .unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_ecdsa_sign_message_hash_out_of_range() {
        match ecdsa_sign(
            &FieldElement::from_hex_be(
                "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
            )
            .unwrap(),
            &FieldElement::from_hex_be(
                "0800000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        ) {
            Err(EcdsaSignError::MessageHashOutOfRange) => {}
            _ => panic!("Should throw error on out of range message hash"),
        };
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_ecdsa_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key = FieldElement::from_hex_be(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let message_hash = FieldElement::from_hex_be(
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
        )
        .unwrap();
        let r = FieldElement::from_hex_be(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let s = FieldElement::from_hex_be(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a",
        )
        .unwrap();

        assert!(ecdsa_verify(&public_key, &message_hash, &Signature { r, s }).unwrap());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_ecdsa_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key = FieldElement::from_hex_be(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let message_hash = FieldElement::from_hex_be(
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
        )
        .unwrap();
        let r = FieldElement::from_hex_be(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let s = FieldElement::from_hex_be(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9b",
        )
        .unwrap();

        assert!(!ecdsa_verify(&public_key, &message_hash, &Signature { r, s }).unwrap());
    }
}
