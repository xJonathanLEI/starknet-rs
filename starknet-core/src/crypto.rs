use crate::types::UnsignedFieldElement;

use starknet_crypto::{
    pedersen_hash, rfc6979_generate_k, sign, verify, FieldElement, SignError, VerifyError,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Signature {
    pub r: UnsignedFieldElement,
    pub s: UnsignedFieldElement,
}

#[derive(Debug, Error)]
pub enum EcdsaSignError {
    #[error("message hash out of range")]
    MessageHashOutOfRange,
}

#[derive(Debug, Error)]
pub enum EcdsaVerifyError {
    #[error("message hash out of range")]
    MessageHashOutOfRange,
    #[error("signature r value out of range")]
    SignatureROutOfRange,
    #[error("signature s value out of range")]
    SignatureSOutOfRange,
}

pub fn compute_hash_on_elements(data: &[UnsignedFieldElement]) -> UnsignedFieldElement {
    let mut current_hash = FieldElement::ZERO;

    for item in data.iter() {
        current_hash = pedersen_hash(&current_hash, &(*item).into());
    }

    let data_len = UnsignedFieldElement::from(data.len());
    current_hash = pedersen_hash(&current_hash, &data_len.into());

    current_hash.into()
}

pub fn ecdsa_sign(
    private_key: &UnsignedFieldElement,
    message_hash: &UnsignedFieldElement,
) -> Result<Signature, EcdsaSignError> {
    let private_key_fe = (*private_key).into();
    let message_hash_fe = (*message_hash).into();

    // Seed-retry logic ported from `cairo-lang`
    let mut seed = None;
    loop {
        let k = rfc6979_generate_k(&message_hash_fe, &private_key_fe, seed.as_ref());

        match sign(&private_key_fe, &message_hash_fe, &k) {
            Ok(sig) => {
                return Ok(Signature {
                    r: sig.r.into(),
                    s: sig.s.into(),
                });
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
    public_key: &UnsignedFieldElement,
    message_hash: &UnsignedFieldElement,
    signature: &Signature,
) -> Result<bool, EcdsaVerifyError> {
    match verify(
        &(*public_key).into(),
        &(*message_hash).into(),
        &signature.r.into(),
        &signature.s.into(),
    ) {
        Ok(result) => Ok(result),
        Err(VerifyError::InvalidMessageHash) => Err(EcdsaVerifyError::MessageHashOutOfRange),
        Err(VerifyError::InvalidR) => Err(EcdsaVerifyError::SignatureROutOfRange),
        Err(VerifyError::InvalidS) => Err(EcdsaVerifyError::SignatureSOutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_on_elements() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[
            UnsignedFieldElement::from_hex_str("0xaa").unwrap(),
            UnsignedFieldElement::from_hex_str("0xbb").unwrap(),
            UnsignedFieldElement::from_hex_str("0xcc").unwrap(),
            UnsignedFieldElement::from_hex_str("0xdd").unwrap(),
        ]);
        let expected_hash = UnsignedFieldElement::from_hex_str(
            "025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69",
        )
        .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn test_compute_hash_on_elements_empty_data() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[]);
        let expected_hash = UnsignedFieldElement::from_hex_str(
            "049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804",
        )
        .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn test_ecdsa_sign() {
        // Generated with `cairo-lang`
        let signature = ecdsa_sign(
            &UnsignedFieldElement::from_hex_str(
                "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
            )
            .unwrap(),
            &UnsignedFieldElement::from_hex_str(
                "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
            )
            .unwrap(),
        )
        .unwrap();
        let expected_r = UnsignedFieldElement::from_hex_str(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let expected_s = UnsignedFieldElement::from_hex_str(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a",
        )
        .unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    fn test_ecdsa_sign_message_hash_out_of_range() {
        match ecdsa_sign(
            &UnsignedFieldElement::from_hex_str(
                "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
            )
            .unwrap(),
            &UnsignedFieldElement::from_hex_str(
                "0800000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        ) {
            Err(EcdsaSignError::MessageHashOutOfRange) => {}
            _ => panic!("Should throw error on out of range message hash"),
        };
    }

    #[test]
    fn test_ecdsa_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key = UnsignedFieldElement::from_hex_str(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let message_hash = UnsignedFieldElement::from_hex_str(
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
        )
        .unwrap();
        let r = UnsignedFieldElement::from_hex_str(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let s = UnsignedFieldElement::from_hex_str(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a",
        )
        .unwrap();

        assert_eq!(
            ecdsa_verify(&public_key, &message_hash, &Signature { r, s }).unwrap(),
            true
        );
    }

    #[test]
    fn test_ecdsa_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key = UnsignedFieldElement::from_hex_str(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let message_hash = UnsignedFieldElement::from_hex_str(
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
        )
        .unwrap();
        let r = UnsignedFieldElement::from_hex_str(
            "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f",
        )
        .unwrap();
        let s = UnsignedFieldElement::from_hex_str(
            "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9b",
        )
        .unwrap();

        assert_eq!(
            ecdsa_verify(&public_key, &message_hash, &Signature { r, s }).unwrap(),
            false
        );
    }
}
