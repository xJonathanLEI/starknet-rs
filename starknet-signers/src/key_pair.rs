use crypto_bigint::{Encoding, NonZero, U256};
use rand::{rngs::StdRng, Rng, SeedableRng};
use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, EcdsaSignError, EcdsaVerifyError, Signature},
    types::Felt,
};
use starknet_crypto::get_public_key;

#[derive(Debug, Clone)]
pub struct SigningKey {
    secret_scalar: Felt,
}

#[derive(Debug, Clone)]
pub struct VerifyingKey {
    scalar: Felt,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, thiserror::Error)]
pub enum KeystoreError {
    #[error("invalid path")]
    InvalidPath,
    #[error("invalid decrypted secret scalar")]
    InvalidScalar,
    #[error(transparent)]
    Inner(eth_keystore::KeystoreError),
}

impl SigningKey {
    /// Generates a new key pair from a cryptographically secure RNG.
    pub fn from_random() -> Self {
        const PRIME: NonZero<U256> = NonZero::from_uint(U256::from_be_hex(
            "0800000000000011000000000000000000000000000000000000000000000001",
        ));

        let mut rng = StdRng::from_entropy();
        let mut buffer = [0u8; 32];
        rng.fill(&mut buffer);

        let random_u256 = U256::from_be_slice(&buffer);
        let secret_scalar = random_u256.rem(&PRIME);

        // It's safe to unwrap here as we're 100% sure it's not out of range
        let secret_scalar = Felt::from_bytes_be_slice(&secret_scalar.to_be_bytes());

        Self { secret_scalar }
    }

    pub const fn from_secret_scalar(secret_scalar: Felt) -> Self {
        Self { secret_scalar }
    }

    /// Loads the private key from a Web3 Secret Storage Definition keystore.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_keystore<P>(path: P, password: &str) -> Result<Self, KeystoreError>
    where
        P: AsRef<std::path::Path>,
    {
        let key = eth_keystore::decrypt_key(path, password).map_err(KeystoreError::Inner)?;
        let secret_scalar = Felt::from_bytes_be_slice(&key);
        Ok(Self::from_secret_scalar(secret_scalar))
    }

    /// Encrypts and saves the private key to a Web3 Secret Storage Definition JSON file.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_as_keystore<P>(&self, path: P, password: &str) -> Result<(), KeystoreError>
    where
        P: AsRef<std::path::Path>,
    {
        // Work around the issue of `eth-keystore` not supporting full path.
        // TODO: patch or fork `eth-keystore`
        let mut path = path.as_ref().to_path_buf();
        let file_name = path
            .file_name()
            .ok_or(KeystoreError::InvalidPath)?
            .to_str()
            .ok_or(KeystoreError::InvalidPath)?
            .to_owned();
        path.pop();

        let mut rng = StdRng::from_entropy();
        eth_keystore::encrypt_key(
            path,
            &mut rng,
            self.secret_scalar.to_bytes_be(),
            password,
            Some(&file_name),
        )
        .map_err(KeystoreError::Inner)?;

        Ok(())
    }

    pub const fn secret_scalar(&self) -> Felt {
        self.secret_scalar
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from_scalar(get_public_key(&self.secret_scalar))
    }

    pub fn sign(&self, hash: &Felt) -> Result<Signature, EcdsaSignError> {
        ecdsa_sign(&self.secret_scalar, hash).map(|sig| sig.into())
    }
}

impl VerifyingKey {
    pub const fn from_scalar(scalar: Felt) -> Self {
        Self { scalar }
    }

    pub const fn scalar(&self) -> Felt {
        self.scalar
    }

    pub fn verify(&self, hash: &Felt, signature: &Signature) -> Result<bool, EcdsaVerifyError> {
        ecdsa_verify(&self.scalar, hash, signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_secret_scalar() {
        // Generated with `cairo-lang`
        let private_key =
            Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79")
                .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);

        assert_eq!(signing_key.secret_scalar(), private_key);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_verifying_key() {
        // Generated with `cairo-lang`
        let private_key =
            Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79")
                .unwrap();
        let expected_public_key =
            Felt::from_hex("02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159")
                .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);
        let verifying_key = signing_key.verifying_key();

        assert_eq!(verifying_key.scalar(), expected_public_key);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_sign() {
        // Generated with `cairo-lang`
        let private_key =
            Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79")
                .unwrap();
        let hash =
            Felt::from_hex("06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76")
                .unwrap();
        let expected_r =
            Felt::from_hex("061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f")
                .unwrap();
        let expected_s =
            Felt::from_hex("04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a")
                .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);
        let signature = signing_key.sign(&hash).unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_hash_out_of_range() {
        let private_key =
            Felt::from_hex("0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79")
                .unwrap();
        let hash =
            Felt::from_hex("0800000000000000000000000000000000000000000000000000000000000000")
                .unwrap();

        let signing_key = SigningKey::from_secret_scalar(private_key);

        match signing_key.sign(&hash) {
            Err(EcdsaSignError::MessageHashOutOfRange) => {}
            _ => panic!("Should throw error on out of range hash"),
        };
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_verify_valid_signature() {
        // Generated with `cairo-lang`
        let public_key =
            Felt::from_hex("02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159")
                .unwrap();
        let hash =
            Felt::from_hex("06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76")
                .unwrap();
        let r = Felt::from_hex("061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f")
            .unwrap();
        let s = Felt::from_hex("04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a")
            .unwrap();

        let verifying_key = VerifyingKey::from_scalar(public_key);

        assert!(verifying_key.verify(&hash, &Signature { r, s }).unwrap());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_verify_invalid_signature() {
        // Generated with `cairo-lang`
        let public_key =
            Felt::from_hex("02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159")
                .unwrap();
        let hash =
            Felt::from_hex("06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76")
                .unwrap();
        let r = Felt::from_hex("061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f")
            .unwrap();
        let s = Felt::from_hex("04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9b")
            .unwrap();

        let verifying_key = VerifyingKey::from_scalar(public_key);

        assert!(!verifying_key.verify(&hash, &Signature { r, s }).unwrap());
    }
}
