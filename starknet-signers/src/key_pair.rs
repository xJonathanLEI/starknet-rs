use starknet_core::{
    crypto::{ecdsa_sign, EcdsaSignError, Signature},
    types::{H256, U256},
};
use starknet_crypto::{get_public_key, FieldElement};

#[derive(Debug)]
pub struct KeyPair {
    private_key_u256: U256,
    private_key_fe: FieldElement,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("private key out of range: {0}")]
    KeyOutOfRange(U256),
    #[error("hash out of range: {0}")]
    HashOutOfRange(H256),
}

impl KeyPair {
    pub fn new(private_key: &U256) -> Result<Self, Error> {
        let mut buffer = [0u8; 32];
        private_key.to_big_endian(&mut buffer);

        Ok(Self {
            private_key_u256: *private_key,
            private_key_fe: FieldElement::from_bytes_be(buffer)
                .ok_or_else(|| Error::KeyOutOfRange(*private_key))?,
        })
    }

    pub fn get_private_key(&self) -> U256 {
        self.private_key_u256
    }

    pub fn get_public_key(&self) -> U256 {
        U256::from_big_endian(&get_public_key(&self.private_key_fe).to_bytes_be())
    }

    pub fn sign_hash(&self, hash: H256) -> Result<Signature, Error> {
        // Quite inefficient here as we're converting between `U256` and `FieldElement`
        // unnecessarily. This shall be fixed once we unify the types as tracked here:
        //   https://github.com/xJonathanLEI/starknet-rs/issues/13
        match ecdsa_sign(&self.private_key_u256, hash) {
            Ok(sig) => Ok(sig),
            Err(EcdsaSignError::MessageHashOutOfRange(hash)) => Err(Error::HashOutOfRange(hash)),
            _ => panic!("unexpected error type"), // impossible
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_private_key() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();

        let key_pair = KeyPair::new(&private_key).unwrap();

        assert_eq!(key_pair.get_private_key(), private_key);
    }

    #[test]
    fn test_get_public_key() {
        // Generated with `cairo-lang`
        let private_key = "0139fe4d6f02e666e86a6f58e65060f115cd3c185bd9e98bd829636931458f79"
            .parse::<U256>()
            .unwrap();
        let expected_public_key =
            "02c5dbad71c92a45cc4b40573ae661f8147869a91d57b8d9b8f48c8af7f83159"
                .parse::<U256>()
                .unwrap();

        let key_pair = KeyPair::new(&private_key).unwrap();

        assert_eq!(key_pair.get_public_key(), expected_public_key);
    }

    #[test]
    fn test_sign_hash() {
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

        let key_pair = KeyPair::new(&private_key).unwrap();
        let signature = key_pair.sign_hash(hash).unwrap();

        assert_eq!(signature.r, expected_r);
        assert_eq!(signature.s, expected_s);
    }

    #[test]
    fn test_private_key_out_of_range() {
        match KeyPair::new(
            &"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
        ) {
            Err(Error::KeyOutOfRange(_)) => {}
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

        let key_pair = KeyPair::new(&private_key).unwrap();

        match key_pair.sign_hash(hash) {
            Err(Error::HashOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range hash"),
        };
    }
}
