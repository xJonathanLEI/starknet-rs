use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize};
use starknet_crypto::poseidon_hash_many;

use super::{revision::Revision, shortstring, TypedDataError};
use crate::utils::cairo_short_string_to_felt;
use crate::{crypto::compute_hash_on_elements, types::Felt};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    /// Domain name.
    pub name: String,
    /// Domain version.
    pub version: Felt,
    /// Domain chain ID.
    pub chain_id: String,
    /// Domain revision.
    pub revision: Revision,
}

impl Domain {
    /// Computes the SNIP-12 hash of the encoded domain.
    ///
    /// The resulting hash is typically used in calculating the full typed data hash as per SNIP-12.
    pub fn encoded_hash(&self) -> Result<Felt, TypedDataError> {
        match self.revision {
            Revision::V0 => Ok(compute_hash_on_elements(&[
                DOMAIN_TYPE_HASH_V0,
                cairo_short_string_to_felt(&self.name)
                    .map_err(|_| TypedDataError::InvalidShortString(self.name.clone()))?,
                self.version,
                cairo_short_string_to_felt(&self.chain_id)
                    .map_err(|_| TypedDataError::InvalidShortString(self.chain_id.clone()))?,
            ])),
            Revision::V1 => Ok(poseidon_hash_many(&[
                DOMAIN_TYPE_HASH_V1,
                cairo_short_string_to_felt(&self.name)
                    .map_err(|_| TypedDataError::InvalidShortString(self.name.clone()))?,
                self.version,
                cairo_short_string_to_felt(&self.chain_id)
                    .map_err(|_| TypedDataError::InvalidShortString(self.chain_id.clone()))?,
                Felt::ONE,
            ])),
        }
    }
}

impl<'de> Deserialize<'de> for Domain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
        pub struct Raw {
            pub name: String,
            #[serde(deserialize_with = "shortstring::deserialize")]
            pub version: Felt,
            #[serde(rename = "chainId")]
            pub chain_id: String,
            #[serde(default = "default_revision")]
            pub revision: Revision,
        }

        let raw = Raw::deserialize(deserializer)?;

        // We want to verify that the short strings are valid.
        cairo_short_string_to_felt(&raw.name).map_err(serde::de::Error::custom)?;
        cairo_short_string_to_felt(&raw.chain_id).map_err(serde::de::Error::custom)?;

        Ok(Domain {
            name: raw.name,
            version: raw.version,
            chain_id: raw.chain_id,
            revision: raw.revision,
        })
    }
}

const fn default_revision() -> Revision {
    Revision::V0
}

impl Serialize for Domain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Domain", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("version", &self.version.to_string())?;
        state.serialize_field("chainId", &self.chain_id)?;
        if let Revision::V1 = self.revision {
            state.serialize_field("revision", "1")?;
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod serialize {
        use super::*;
        use serde_json::Value;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_serialize_when_revision_0() {
            // Given
            let domain = Domain {
                name: "Starknet Example".to_string(),
                version: Felt::ONE,
                chain_id: "SN_MAIN".to_string(),
                revision: Revision::V0,
            };
            let expected: &str = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN"
}"###;

            // When
            let result = serde_json::to_string(&domain).unwrap();

            // Then
            let expected_json: Value = serde_json::from_str(expected).unwrap();
            let result_json: Value = serde_json::from_str(&result).unwrap();
            assert_eq!(expected_json, result_json);
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_serialize_when_revision_1() {
            // Given
            let domain = Domain {
                name: "Starknet Example".to_string(),
                version: Felt::from(2),
                chain_id: "SN_MAIN".to_string(),
                revision: Revision::V1,
            };
            let expected: &str = r###"{
  "name": "Starknet Example",
  "version": "2",
  "chainId": "SN_MAIN",
  "revision": "1"
}"###;

            // When
            let result = serde_json::to_string(&domain).unwrap();

            // Then
            let expected_json: Value = serde_json::from_str(expected).unwrap();
            let result_json: Value = serde_json::from_str(&result).unwrap();
            assert_eq!(expected_json, result_json);
        }
    }

    #[cfg(test)]
    mod deserialize {
        use super::*;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_init_domain_when_version_1_and_no_revision() {
            // Given
            let raw = r###"{
  "name": "Starknet Example",
  "version": "1",
  "chainId": "SN_MAIN"
}"###;

            // When
            let domain = serde_json::from_str::<Domain>(raw).unwrap();

            // Then
            assert_eq!(domain.revision, Revision::V0);
            // `shortstring` spec deviation for `starknet.js` compatibility
            assert_eq!(domain.version, Felt::ONE);
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_init_domain_when_version_1_and_revision_0() {
            // Given
            let raw = r###"{
  "name": "Starknet Example",
  "version": 1,
  "chainId": "SN_MAIN",
  "revision": "0"
}"###;

            // When
            let domain = serde_json::from_str::<Domain>(raw).unwrap();

            // Then
            assert_eq!(domain.revision, Revision::V0);
            // `shortstring` spec deviation for `starknet.js` compatibility
            assert_eq!(domain.version, Felt::ONE);
        }

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_init_domain_when_version_2_and_revision_1() {
            // Given
            let raw = r###"{
  "name": "Starknet Example",
  "version": "2",
  "chainId": "SN_MAIN",
  "revision": "1"
}"###;

            // When
            let domain = serde_json::from_str::<Domain>(raw).unwrap();

            // Then
            assert_eq!(domain.revision, Revision::V1);
            // `shortstring` spec deviation for `starknet.js` compatibility
            assert_eq!(domain.version, Felt::from(2));
        }
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
            domain.encoded_hash().unwrap(),
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
            domain.encoded_hash().unwrap(),
            Felt::from_hex_unchecked(
                "0x03bfc3e1ff0f5c85c05bb8073a64a40b038eed00a449bc337c8cd2758f634640"
            )
        );
    }
}
