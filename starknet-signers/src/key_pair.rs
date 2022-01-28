use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, EcdsaSignError, EcdsaVerifyError, Signature},
    types::UnsignedFieldElement,
};
use starknet_crypto::get_public_key;

#[derive(Debug)]
pub struct SigningKey {
    secret_scalar: UnsignedFieldElement,
}

#[derive(Debug)]
pub struct VerifyingKey {
    scalar: UnsignedFieldElement,
}

impl SigningKey {
    pub fn from_secret_scalar(secret_scalar: UnsignedFieldElement) -> Self {
        Self { secret_scalar }
    }

    pub fn secret_scalar(&self) -> UnsignedFieldElement {
        self.secret_scalar
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from_scalar(get_public_key(&self.secret_scalar.into()).into())
    }

    pub fn sign(&self, hash: &UnsignedFieldElement) -> Result<Signature, EcdsaSignError> {
        ecdsa_sign(&self.secret_scalar, hash)
    }
}

impl VerifyingKey {
    pub fn from_scalar(scalar: UnsignedFieldElement) -> Self {
        Self { scalar }
    }

    pub fn scalar(&self) -> UnsignedFieldElement {
        self.scalar
    }

    pub fn verify(
        &self,
        hash: &UnsignedFieldElement,
        signature: &Signature,
    ) -> Result<bool, EcdsaVerifyError> {
        ecdsa_verify(&self.scalar, hash, signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_secret_scalar() {
        // Generated with `cairo-lang`
        let private_key = UnsignedFieldElement::from_hex_str(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
        )
        .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);

        assert_eq!(signing_key.secret_scalar(), private_key);
    }

    #[test]
    fn test_get_verifying_key() {
        // Generated with `cairo-lang`
        let private_key = UnsignedFieldElement::from_hex_str(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
        )
        .unwrap();
        let expected_public_key = UnsignedFieldElement::from_hex_str(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);
        let verifying_key = signing_key.verifying_key();

        assert_eq!(verifying_key.scalar(), expected_public_key);
    }

    #[test]
    fn test_sign() {
        // Generated with `cairo-lang`
        let private_key = UnsignedFieldElement::from_hex_str(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
        )
        .unwrap();
        let hash = UnsignedFieldElement::from_hex_str(
            "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76",
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

        let signing_key = SigningKey::from_secret_scalar(private_key);
        let signature = signing_key.sign(&hash).unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    fn test_hash_out_of_range() {
        let private_key = UnsignedFieldElement::from_hex_str(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79",
        )
        .unwrap();
        let hash = UnsignedFieldElement::from_hex_str(
            "0800000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);

        match signing_key.sign(&hash) {
            Err(EcdsaSignError::MessageHashOutOfRange) => {}
            _ => panic!("Should throw error on out of range hash"),
        };
    }

    #[test]
    fn test_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key = UnsignedFieldElement::from_hex_str(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let hash = UnsignedFieldElement::from_hex_str(
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

        let verifying_key = VerifyingKey::from_scalar(public_key);

        assert_eq!(
            verifying_key.verify(&hash, &Signature { r, s }).unwrap(),
            true
        );
    }

    #[test]
    fn test_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key = UnsignedFieldElement::from_hex_str(
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159",
        )
        .unwrap();
        let hash = UnsignedFieldElement::from_hex_str(
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

        let verifying_key = VerifyingKey::from_scalar(public_key);

        assert_eq!(
            verifying_key.verify(&hash, &Signature { r, s }).unwrap(),
            false
        );
    }
}
