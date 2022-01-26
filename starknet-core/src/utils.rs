use ethereum_types::H256;
use sha3::{Digest, Keccak256};
use thiserror::Error;

const DEFAULT_ENTRY_POINT_NAME: &str = "__default__";
const DEFAULT_L1_ENTRY_POINT_NAME: &str = "__l1_default__";

#[derive(Debug, Error)]
#[error("the provided name contains non-ASCII characters: {name}")]
pub struct NonAsciiNameError<'a> {
    pub name: &'a str,
}

/// A variant of eth-keccak that computes a value that fits in a StarkNet field element.
pub fn starknet_keccak(data: &[u8]) -> H256 {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let mut hash = hasher.finalize();

    // Remove the first 6 bits
    hash[0] &= 0b00000011;

    H256::from_slice(&hash)
}

pub fn get_selector_from_name(func_name: &str) -> Result<H256, NonAsciiNameError> {
    if func_name == DEFAULT_ENTRY_POINT_NAME || func_name == DEFAULT_L1_ENTRY_POINT_NAME {
        Ok(H256::zero())
    } else {
        let name_bytes = func_name.as_bytes();
        if name_bytes.is_ascii() {
            Ok(starknet_keccak(name_bytes))
        } else {
            Err(NonAsciiNameError { name: func_name })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starknet_keccak() {
        // Generated from `cairo-lang`
        let data = b"execute";
        let expected_hash = "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44"
            .parse::<H256>()
            .unwrap();

        let hash = starknet_keccak(data);

        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_get_selector_from_name() {
        // Generated from `cairo-lang`
        let func_name = "execute";
        let expected_selector = "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44"
            .parse::<H256>()
            .unwrap();

        let selector = get_selector_from_name(func_name).unwrap();

        assert_eq!(selector, expected_selector);
    }

    #[test]
    fn test_get_default_selector() {
        let default_selector = "0000000000000000000000000000000000000000000000000000000000000000"
            .parse::<H256>()
            .unwrap();

        assert_eq!(
            get_selector_from_name("__default__").unwrap(),
            default_selector
        );
        assert_eq!(
            get_selector_from_name("__l1_default__").unwrap(),
            default_selector
        );
    }

    #[test]
    fn test_get_selector_from_non_ascii_name() {
        let func_name = "🦀";

        match get_selector_from_name(func_name) {
            Err(_) => {}
            _ => panic!("Should throw error on non-ASCII name"),
        };
    }
}
