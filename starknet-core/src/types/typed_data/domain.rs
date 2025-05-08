use serde::{Deserialize, Serialize};
use starknet_crypto::poseidon_hash_many;

use crate::{crypto::compute_hash_on_elements, types::Felt};

use super::{revision::Revision, shortstring};

/// SNIP-12 type hash of the domain type of revision 0.
///
/// Compuated as:
///
/// ```ignore
/// starknet_keccak("StarkNetDomain(name:felt,version:felt,chainId:felt)")
/// ```
const DOMAIN_TYPE_HASH_V0: Felt = Felt::from_raw([
    454097714883350422,
    18110465409072164514,
    49961291536018317,
    11250613311408382492,
]);

/// SNIP-12 type hash of the domain type of revision 1.
///
/// Compuated as:
///
/// ```ignore
/// starknet_keccak("\"StarknetDomain\"(\"name\":\"shortstring\",\"version\":\"shortstring\",\"chainId\":\"shortstring\",\"revision\":\"shortstring\")")
/// ```
const DOMAIN_TYPE_HASH_V1: Felt = Felt::from_raw([
    45164882192052528,
    3320515356094353366,
    7437117071726711362,
    6953663458211852539,
]);

/// SNIP-12 typed data domain separator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    /// Domain name.
    #[serde(
        serialize_with = "shortstring::serialize",
        deserialize_with = "shortstring::deserialize",
    )]
    pub name: Felt,
    /// Domain version.
    #[serde(
        serialize_with = "shortstring::serialize",
        deserialize_with = "shortstring::deserialize",
    )]
    pub version: Felt,
    /// Domain chain ID.
    #[serde(
        serialize_with = "shortstring::serialize",
        deserialize_with = "shortstring::deserialize",
        rename = "chainId"
    )]
    pub chain_id: Felt,
    /// Domain revision.
    #[serde(default = "default_revision")]
    pub revision: Revision,
}

impl Domain {
    /// Computes the SNIP-12 hash of the encoded domain.
    ///
    /// The resulting hash is typically used in calculating the full typed data hash as per SNIP-12.
    pub fn encoded_hash(&self) -> Felt {
        match self.revision {
            Revision::V0 => compute_hash_on_elements(&[
                DOMAIN_TYPE_HASH_V0,
                self.name,
                self.version,
                self.chain_id,
            ]),
            Revision::V1 => poseidon_hash_many(&[
                DOMAIN_TYPE_HASH_V1,
                self.name,
                self.version,
                self.chain_id,
                Felt::ONE,
            ]),
        }
    }
}

const fn default_revision() -> Revision {
    Revision::V0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_implicit_v0_domain_deser() {
        let raw = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN"
}"###;

        let domain = serde_json::from_str::<Domain>(raw).unwrap();
        assert_eq!(domain.revision, Revision::V0);

        // `shortstring` spec deviation for `starknet.js` compatibility
        assert_eq!(domain.version, Felt::ONE);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_explicit_v0_domain_deser() {
        let raw = r###"{
  "name": "Starknet Example",
  "version": 1,
  "chainId": "SN_MAIN",
  "revision": "0"
}"###;

        let domain = serde_json::from_str::<Domain>(raw).unwrap();
        assert_eq!(domain.revision, Revision::V0);

        // `shortstring` spec deviation for `starknet.js` compatibility
        assert_eq!(domain.version, Felt::ONE);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_explicit_v1_domain_deser() {
        let raw = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN",
  "revision": "1"
}"###;

        let domain = serde_json::from_str::<Domain>(raw).unwrap();
        assert_eq!(domain.revision, Revision::V1);

        // `shortstring` spec deviation for `starknet.js` compatibility
        assert_eq!(domain.version, Felt::ONE);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_v0_domain_hash() {
        let raw = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN"
}"###;

        let domain = serde_json::from_str::<Domain>(raw).unwrap();
        assert_eq!(
            domain.encoded_hash(),
            Felt::from_hex_unchecked(
                "0x04f8ee4d303cd69ce9c78edadf62442865c89a1eec01fa413e126a058a69c28a"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_v1_domain_hash() {
        let raw = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN",
  "revision": "1"
}"###;

        let domain = serde_json::from_str::<Domain>(raw).unwrap();
        assert_eq!(
            domain.encoded_hash(),
            Felt::from_hex_unchecked(
                "0x03bfc3e1ff0f5c85c05bb8073a64a40b038eed00a449bc337c8cd2758f634640"
            )
        );
    }
}
