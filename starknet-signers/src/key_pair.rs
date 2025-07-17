use crypto_bigint::{Encoding, NonZero, U256};
use hex;
use rand::{rngs::StdRng, Rng, SeedableRng};
use secrecy::{ExposeSecret, SecretString};
use starknet_core::types::FromStrError;
use starknet_core::{
    crypto::{ecdsa_sign, ecdsa_verify, EcdsaSignError, EcdsaVerifyError, Signature},
    types::Felt,
};
use starknet_crypto::get_public_key;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A ECDSA signing (private) key on the STARK curve.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SigningKey {
    secret_scalar: Box<Felt>,
}

/// A ECDSA verifying (public) key on the STARK curve.
#[derive(Debug, Clone)]
pub struct VerifyingKey {
    scalar: Felt,
}

/// Errors using an encrypted JSON keystore.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Debug, thiserror::Error)]
pub enum KeystoreError {
    /// The file path is invalid.
    #[error("invalid path")]
    InvalidPath,
    /// The decrypted secret scalar is not a valid private key.
    #[error("invalid decrypted secret scalar")]
    InvalidScalar,
    /// Upstream `eth-keystore` error propagated.
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
        let mut secret_scalar = Felt::from_bytes_be_slice(&secret_scalar.to_be_bytes());

        let result = Self {
            secret_scalar: Box::new(secret_scalar),
        };

        secret_scalar.zeroize();

        result
    }

    /// Constructs [`SigningKey`] directly from a secret scalar.
    pub fn from_secret(secret_value: SecretString) -> Result<Self, FromStrError> {
        Ok(Self {
            secret_scalar: Box::new(Felt::from_hex(secret_value.expose_secret())?),
        })
    }

    /// Loads the private key from a Web3 Secret Storage Definition keystore.
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn from_keystore<P>(path: P, password: &str) -> Result<Self, KeystoreError>
    where
        P: AsRef<std::path::Path>,
    {
        let key = eth_keystore::decrypt_key(path, password).map_err(KeystoreError::Inner)?;

        Ok(Self::from_secret(SecretString::from(hex::encode(key)))
            .map_err(|_| KeystoreError::InvalidScalar)?)
    }

    /// Encrypts and saves the private key to a Web3 Secret Storage Definition JSON file.
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
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

    /// Gets the secret scalar in the signing key.
    pub const fn secret_scalar(&self) -> &Felt {
        &self.secret_scalar
    }

    /// Derives the verifying (public) key that corresponds to the signing key.
    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from_scalar(get_public_key(&self.secret_scalar))
    }

    /// Signs a raw hash using ECDSA for a signature.
    pub fn sign(&self, hash: &Felt) -> Result<Signature, EcdsaSignError> {
        ecdsa_sign(&self.secret_scalar, hash).map(|sig| sig.into())
    }
}

impl VerifyingKey {
    /// Constructs [`VerifyingKey`] directly from a scalar.
    pub const fn from_scalar(scalar: Felt) -> Self {
        Self { scalar }
    }

    /// Gets the scalar in the verifying key.
    pub const fn scalar(&self) -> Felt {
        self.scalar
    }

    /// Verifies that an ECDSA signature is valid for the verifying key against a certain message
    /// hash.
    pub fn verify(&self, hash: &Felt, signature: &Signature) -> Result<bool, EcdsaVerifyError> {
        ecdsa_verify(&self.scalar, hash, signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_secret_scalar() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79";

        let signing_key = SigningKey::from_secret(SecretString::from(private_key)).unwrap();

        assert_eq!(
            *signing_key.secret_scalar(),
            Felt::from_hex_unchecked(private_key)
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_verifying_key() {
        // Generated with `cairo-lang`
        let private_key = SecretString::new(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79".into(),
        );
        let expected_public_key =
            Felt::from_hex("02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159")
                .unwrap();

        let signing_key = SigningKey::from_secret(private_key).unwrap();
        let verifying_key = signing_key.verifying_key();

        assert_eq!(verifying_key.scalar(), expected_public_key);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_sign() {
        // Generated with `cairo-lang`
        let private_key = SecretString::new(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79".into(),
        );
        let hash =
            Felt::from_hex("06fea80189363a786037ed3e7ba546dad0ef7de49fccae0e31eb658b7dd4ea76")
                .unwrap();
        let expected_r =
            Felt::from_hex("061ec782f76a66f6984efc3a1b6d152a124c701c00abdd2bf76641b4135c770f")
                .unwrap();
        let expected_s =
            Felt::from_hex("04e44e759cea02c23568bb4d8a09929bbca8768ab68270d50c18d214166ccd9a")
                .unwrap();

        let signing_key = SigningKey::from_secret(private_key).unwrap();
        let signature = signing_key.sign(&hash).unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_hash_out_of_range() {
        let private_key = SecretString::new(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79".into(),
        );
        let hash =
            Felt::from_hex("0800000000000000000000000000000000000000000000000000000000000000")
                .unwrap();

        let signing_key = SigningKey::from_secret(private_key).unwrap();

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

    #[test]
    fn test_zeroize_signing_key() {
        let private_key = SecretString::new(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79".into(),
        );
        let mut signing_key = SigningKey::from_secret(private_key).unwrap();
        signing_key.zeroize();

        let ptr = signing_key.secret_scalar.as_ref() as *const Felt as *const u8;
        let after_zeroize = unsafe { std::slice::from_raw_parts(ptr, size_of::<Felt>()) };
        assert_eq!(after_zeroize, vec![0; 32]);
    }

    #[test]
    fn test_keystore() {
        let signing_key = SigningKey::from_secret(SecretString::new(
            "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79".into(),
        ))
        .unwrap();

        let temp_dir = TempDir::new("temp_folder").unwrap();
        let mut keystore_path = temp_dir.into_path();

        keystore_path.push("keystore");

        let password = "1234";
        signing_key
            .save_as_keystore(&keystore_path, password)
            .unwrap();

        let signing_key_from_keystore =
            SigningKey::from_keystore(&keystore_path, password).unwrap();

        assert_eq!(
            signing_key.secret_scalar,
            signing_key_from_keystore.secret_scalar
        );
    }
}
