use ethereum_types::{H256, U256};
use starknet_crypto::{pedersen_hash, rfc6979_generate_k, sign, verify, FieldElement, VerifyError};
use thiserror::Error;

#[derive(Debug)]
pub struct Signature {
    pub r: U256,
    pub s: U256,
}

#[derive(Debug, Error)]
pub enum PedersenHashError {
    #[error("data must not be empty")]
    EmptyData,
    #[error("element out of range: {0}")]
    ElementOutOfRange(U256),
}

#[derive(Debug, Error)]
pub enum EcdsaError {
    #[error("private key out of range: {0}")]
    PrivateKeyOutOfRange(U256),
    #[error("public key out of range: {0}")]
    PublicKeyOutOfRange(U256),
    #[error("message hash out of range: {0}")]
    MessageHashOutOfRange(H256),
    #[error("signature out of range: {0}")]
    SignatureOutOfRange(U256),
}

pub fn compute_hash_on_elements(data: &[U256]) -> Result<H256, PedersenHashError> {
    if data.is_empty() {
        return Err(PedersenHashError::EmptyData);
    }

    // unwrap() is safe here as it'll always succeed
    let mut current_hash = FieldElement::from_bytes_be([0u8; 32]).unwrap();

    for item in data.iter() {
        current_hash = pedersen_hash(
            &current_hash,
            &u256_to_field_element(item).ok_or(PedersenHashError::ElementOutOfRange(*item))?,
        );
    }

    let data_len = U256::from(data.len());
    current_hash = pedersen_hash(
        &current_hash,
        &u256_to_field_element(&data_len).ok_or(PedersenHashError::ElementOutOfRange(data_len))?,
    );

    Ok(H256::from_slice(&current_hash.to_bytes_be()))
}

pub fn ecdsa_sign(private_key: &U256, message_hash: H256) -> Result<Signature, EcdsaError> {
    let private_key =
        u256_to_field_element(private_key).ok_or(EcdsaError::PrivateKeyOutOfRange(*private_key))?;
    let message_hash = FieldElement::from_bytes_be(message_hash.0)
        .ok_or(EcdsaError::MessageHashOutOfRange(message_hash))?;

    // Seed-retry logic ported from `cairo-lang`
    let mut seed = None;
    loop {
        let k = rfc6979_generate_k(&message_hash, &private_key, seed.as_ref());

        // The only possible error is invalid K, in which case we simply retry
        if let Ok(sig) = sign(&private_key, &message_hash, &k) {
            return Ok(Signature {
                r: U256::from_big_endian(&sig.r.to_bytes_be()),
                s: U256::from_big_endian(&sig.s.to_bytes_be()),
            });
        }

        seed = match seed {
            Some(prev_seed) => Some(prev_seed + FieldElement::ONE),
            None => Some(FieldElement::ONE),
        };
    }
}

pub fn ecdsa_verify(
    public_key: &U256,
    message_hash: H256,
    signature: &Signature,
) -> Result<bool, EcdsaError> {
    let public_key =
        u256_to_field_element(public_key).ok_or(EcdsaError::PublicKeyOutOfRange(*public_key))?;
    let hash = FieldElement::from_bytes_be(message_hash.0)
        .ok_or(EcdsaError::MessageHashOutOfRange(message_hash))?;
    let signature_r =
        u256_to_field_element(&signature.r).ok_or(EcdsaError::SignatureOutOfRange(signature.r))?;
    let signature_s =
        u256_to_field_element(&signature.s).ok_or(EcdsaError::SignatureOutOfRange(signature.s))?;

    match verify(&public_key, &hash, &signature_r, &signature_s) {
        Ok(result) => Ok(result),
        Err(VerifyError::InvalidMessageHash) => {
            Err(EcdsaError::MessageHashOutOfRange(message_hash))
        }
        Err(VerifyError::InvalidR) => Err(EcdsaError::SignatureOutOfRange(signature.r)),
        Err(VerifyError::InvalidS) => Err(EcdsaError::SignatureOutOfRange(signature.s)),
    }
}

fn u256_to_field_element(num: &U256) -> Option<FieldElement> {
    let mut buffer = [0u8; 32];
    num.to_big_endian(&mut buffer);
    FieldElement::from_bytes_be(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_on_elements() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[
            "0xaa".parse::<U256>().unwrap(),
            "0xbb".parse::<U256>().unwrap(),
            "0xcc".parse::<U256>().unwrap(),
            "0xdd".parse::<U256>().unwrap(),
        ])
        .unwrap();
        let expected_hash = "025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69"
            .parse::<H256>()
            .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn test_compute_hash_on_elements_empty_data() {
        match compute_hash_on_elements(&[]) {
            Err(PedersenHashError::EmptyData) => {}
            _ => panic!("Should throw error on empty data"),
        };
    }

    #[test]
    fn test_compute_hash_on_elements_out_of_range() {
        match compute_hash_on_elements(&[
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
        ]) {
            Err(PedersenHashError::ElementOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range data"),
        };
    }

    #[test]
    fn test_ecdsa_sign() {
        // Generated with `cairo-lang`
        let signature = ecdsa_sign(
            &"0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
                .parse::<U256>()
                .unwrap(),
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
                .parse::<H256>()
                .unwrap(),
        )
        .unwrap();
        let expected_r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let expected_s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a"
            .parse::<U256>()
            .unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    fn test_ecdsa_sign_private_key_out_of_range() {
        match ecdsa_sign(
            &"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
                .parse::<H256>()
                .unwrap(),
        ) {
            Err(EcdsaError::PrivateKeyOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range private key"),
        };
    }

    #[test]
    fn test_ecdsa_sign_message_hash_out_of_range() {
        match ecdsa_sign(
            &"0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
                .parse::<U256>()
                .unwrap(),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<H256>()
                .unwrap(),
        ) {
            Err(EcdsaError::MessageHashOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range message hash"),
        };
    }

    #[test]
    fn test_ecdsa_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key = "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
            .parse::<U256>()
            .unwrap();
        let message_hash = "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
            .parse::<H256>()
            .unwrap();
        let r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a"
            .parse::<U256>()
            .unwrap();

        assert_eq!(
            ecdsa_verify(&public_key, message_hash, &Signature { r, s }).unwrap(),
            true
        );
    }

    #[test]
    fn test_ecdsa_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key = "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
            .parse::<U256>()
            .unwrap();
        let message_hash = "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
            .parse::<H256>()
            .unwrap();
        let r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9b"
            .parse::<U256>()
            .unwrap();

        assert_eq!(
            ecdsa_verify(&public_key, message_hash, &Signature { r, s }).unwrap(),
            false
        );
    }
}
