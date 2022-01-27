use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, EcdsaError, Signature},
    types::{H256, U256},
};
use starknet_crypto::{get_public_key, FieldElement};

#[derive(Debug)]
pub struct SigningKey {
    secret_scalar: U256,
}

#[derive(Debug)]
pub struct VerifyingKey {
    scalar: U256,
}

#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("scalar out of range: {0}")]
    ScalarOutOfRange(U256),
    #[error("hash out of range: {0}")]
    HashOutOfRange(H256),
}

impl SigningKey {
    pub fn from_secret_scalar(secret_scalar: &U256) -> Result<Self, KeyError> {
        // Make use of `FieldElement` for the range check
        // TODO: use `U256` constant for the check or unify types (#13) so that no range check is
        //       required at all.
        let mut buffer = [0u8; 32];
        secret_scalar.to_big_endian(&mut buffer);
        if FieldElement::from_bytes_be(buffer).is_some() {
            Ok(Self {
                secret_scalar: *secret_scalar,
            })
        } else {
            Err(KeyError::ScalarOutOfRange(*secret_scalar))
        }
    }

    pub fn secret_scalar(&self) -> U256 {
        self.secret_scalar
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        let mut buffer = [0u8; 32];
        self.secret_scalar.to_big_endian(&mut buffer);

        VerifyingKey::from_scalar(&U256::from_big_endian(
            &get_public_key(&FieldElement::from_bytes_be(buffer).unwrap()).to_bytes_be(),
        ))
        .unwrap()
    }

    pub fn sign(&self, hash: H256) -> Result<Signature, KeyError> {
        // Quite inefficient here as we're converting between `U256` and `FieldElement`
        // unnecessarily. This shall be fixed once we unify the types as tracked here:
        //   https://github.com/xJonathanLEI/starknet-rs/issues/13
        match ecdsa_sign(&self.secret_scalar, hash) {
            Ok(sig) => Ok(sig),
            Err(EcdsaError::MessageHashOutOfRange(hash)) => Err(KeyError::HashOutOfRange(hash)),
            _ => panic!("unexpected error type"), // impossible
        }
    }
}

impl VerifyingKey {
    pub fn from_scalar(scalar: &U256) -> Result<Self, KeyError> {
        // Make use of `FieldElement` for the range check
        // TODO: use `U256` constant for the check or unify types (#13) so that no range check is
        //       required at all.
        let mut buffer = [0u8; 32];
        scalar.to_big_endian(&mut buffer);
        if FieldElement::from_bytes_be(buffer).is_some() {
            Ok(Self { scalar: *scalar })
        } else {
            Err(KeyError::ScalarOutOfRange(*scalar))
        }
    }

    pub fn scalar(&self) -> U256 {
        self.scalar
    }

    pub fn verify(&self, hash: H256, signature: &Signature) -> Result<bool, KeyError> {
        match ecdsa_verify(&self.scalar, hash, signature) {
            Ok(result) => Ok(result),
            Err(EcdsaError::MessageHashOutOfRange(hash)) => Err(KeyError::HashOutOfRange(hash)),
            Err(EcdsaError::SignatureOutOfRange(sig)) => Err(KeyError::ScalarOutOfRange(sig)),
            _ => panic!("unexpected error type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_secret_scalar() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();

        let signing_key = SigningKey::from_secret_scalar(&private_key).unwrap();

        assert_eq!(signing_key.secret_scalar(), private_key);
    }

    #[test]
    fn test_get_verifying_key() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();
        let expected_public_key =
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
                .parse::<U256>()
                .unwrap();

        let signing_key = SigningKey::from_secret_scalar(&private_key).unwrap();
        let verifying_key = signing_key.verifying_key();

        assert_eq!(verifying_key.scalar(), expected_public_key);
    }

    #[test]
    fn test_sign() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();
        let hash = "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
            .parse::<H256>()
            .unwrap();
        let expected_r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let expected_s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a"
            .parse::<U256>()
            .unwrap();

        let signing_key = SigningKey::from_secret_scalar(&private_key).unwrap();
        let signature = signing_key.sign(hash).unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    fn test_secret_scalar_out_of_range() {
        match SigningKey::from_secret_scalar(
            &"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
        ) {
            Err(KeyError::ScalarOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range private key"),
        };
    }

    #[test]
    fn test_hash_out_of_range() {
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();
        let hash = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            .parse::<H256>()
            .unwrap();

        let signing_key = SigningKey::from_secret_scalar(&private_key).unwrap();

        match signing_key.sign(hash) {
            Err(KeyError::HashOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range hash"),
        };
    }

    #[test]
    fn test_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key = "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
            .parse::<U256>()
            .unwrap();
        let hash = "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
            .parse::<H256>()
            .unwrap();
        let r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a"
            .parse::<U256>()
            .unwrap();

        let verifying_key = VerifyingKey::from_scalar(&public_key).unwrap();

        assert_eq!(
            verifying_key.verify(hash, &Signature { r, s }).unwrap(),
            true
        );
    }

    #[test]
    fn test_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key = "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
            .parse::<U256>()
            .unwrap();
        let hash = "06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76"
            .parse::<H256>()
            .unwrap();
        let r = "061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f"
            .parse::<U256>()
            .unwrap();
        let s = "04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9b"
            .parse::<U256>()
            .unwrap();

        let verifying_key = VerifyingKey::from_scalar(&public_key).unwrap();

        assert_eq!(
            verifying_key.verify(hash, &Signature { r, s }).unwrap(),
            false
        );
    }
}
