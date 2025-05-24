use alloc::format;

use serde::Deserialize;
use starknet_crypto::{PedersenHasher, PoseidonHasher};

use crate::types::Felt;

mod domain;
pub use domain::Domain;

mod error;
pub use error::TypedDataError;

mod hasher;
use hasher::TypedDataHasher;

mod revision;
pub use revision::Revision;

mod shortstring;

mod type_definition;
pub use type_definition::{
    CompositeType, EnumDefinition, FieldDefinition, StructDefinition, TypeDefinition,
    VariantDefinition,
};

mod type_reference;
pub use type_reference::{
    CommonTypeReference, ElementTypeReference, FullTypeReference, InlineTypeReference,
    TypeReference,
};

mod types;
pub use types::Types;

mod value;
pub use value::{ArrayValue, ObjectValue, Value, ValueKind};

mod encoder;
pub use encoder::{CompositeFieldEncodingIter, Encoder};

/// Cairo short string encoding of `StarkNet Message`.
const STARKNET_MESSAGE_PREFIX: Felt = Felt::from_raw([
    257012186512350467,
    18446744073709551605,
    10480951322775611302,
    16156019428408348868,
]);

/// SNIP-12 typed data for off-chain signatures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedData {
    /// Encoder for encoding the message of the primary type.
    encoder: Encoder,
    /// Reference to the primary/entrypoint type that the `message` field represents.
    primary_type: InlineTypeReference,
    /// The main message data to be signed, structured as per `primary_type`'s definition.
    message: Value,
}

impl TypedData {
    /// Creates a new [`TypedDataError`]. Returns `Err` if `types` and `domain` use
    /// different revisions.
    pub fn new(
        types: Types,
        domain: Domain,
        primary_type: InlineTypeReference,
        message: Value,
    ) -> Result<Self, TypedDataError> {
        Ok(Self {
            encoder: Encoder::new(types, domain)?,
            primary_type,
            message,
        })
    }

    /// Gets the SNIP-12 revision of this [`TypedData`].
    pub const fn revision(&self) -> Revision {
        self.encoder.revision()
    }

    /// Gets a reference to the message's encoder.
    pub const fn encoder(&self) -> &Encoder {
        &self.encoder
    }

    /// Gets a reference to the message's primary type reference.
    pub const fn primary_type(&self) -> &InlineTypeReference {
        &self.primary_type
    }

    /// Gets a reference to the main message value.
    pub const fn message(&self) -> &Value {
        &self.message
    }

    /// Computes the SNIP-12 typed data hash to be used for message signing and verification.
    ///
    /// On-chain signature verification usually involves calling the `is_valid_signature()` function
    /// with this hash.
    pub fn message_hash(&self, address: Felt) -> Result<Felt, TypedDataError> {
        match self.revision() {
            Revision::V0 => self.message_hash_with_hasher::<PedersenHasher>(address),
            Revision::V1 => self.message_hash_with_hasher::<PoseidonHasher>(address),
        }
    }

    fn message_hash_with_hasher<H>(&self, address: Felt) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
    {
        let mut hasher = H::default();
        hasher.update(STARKNET_MESSAGE_PREFIX);
        hasher.update(self.encoder.domain().encoded_hash());
        hasher.update(address);
        hasher.update(
            self.encoder
                .encode_value(&self.primary_type, &self.message)?,
        );
        Ok(hasher.finalize())
    }
}

impl<'de> Deserialize<'de> for TypedData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            types: Types,
            domain: Domain,
            #[serde(rename = "primaryType")]
            primary_type: InlineTypeReference,
            message: Value,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.types, raw.domain, raw.primary_type, raw.message)
            .map_err(|err| serde::de::Error::custom(format!("{}", err)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_V0_DATA: &str = r###"{
  "types": {
    "StarkNetDomain": [
      { "name": "name", "type": "felt" },
      { "name": "version", "type": "felt" },
      { "name": "chainId", "type": "felt" }
    ],
    "Example Message": [
      { "name": "Name", "type": "string" },
      { "name": "Some Array", "type": "u128*" },
      { "name": "Some Object", "type": "My Object" }
    ],
    "My Object": [
      { "name": "Some Selector", "type": "selector" },
      { "name": "Some Contract Address", "type": "ContractAddress" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN"
  },
  "message": {
    "Name": "some name",
    "Some Array": [1, 2, 3, 4],
    "Some Object": {
      "Some Selector": "transfer",
      "Some Contract Address": "0x0123"
    }
  }
}"###;

    const VALID_V1_DATA: &str = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Name", "type": "string" },
      { "name": "Some Array", "type": "u128*" },
      { "name": "Some Object", "type": "My Object" }
    ],
    "My Object": [
      { "name": "Some Selector", "type": "selector" },
      { "name": "Some Contract Address", "type": "ContractAddress" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Name": "some name",
    "Some Array": [1, 2, 3, 4],
    "Some Object": {
      "Some Selector": "transfer",
      "Some Contract Address": "0x0123"
    }
  }
}"###;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_successful_deser_v0() {
        serde_json::from_str::<TypedData>(VALID_V0_DATA).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_successful_deser_v1() {
        serde_json::from_str::<TypedData>(VALID_V1_DATA).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_inconsistent_revision_deser() {
        let raw = r###"{
  "types": {
    "StarkNetDomain": [
      { "name": "name", "type": "felt" },
      { "name": "version", "type": "felt" },
      { "name": "chainId", "type": "felt" }
    ],
    "Example Message": [
      { "name": "Name", "type": "string" },
      { "name": "Some Array", "type": "u128*" },
      { "name": "Some Object", "type": "My Object" }
    ],
    "My Object": [
      { "name": "Some Selector", "type": "selector" },
      { "name": "Some Contract Address", "type": "ContractAddress" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Name": "some name",
    "Some Array": [1, 2, 3, 4],
    "Some Object": {
      "Some Selector": "transfer",
      "Some Contract Address": "0x0123"
    }
  }
}"###;

        assert_eq!(
            serde_json::from_str::<TypedData>(raw)
                .unwrap_err()
                .to_string(),
            "`types` implies revision 0 but `domain` uses revision 1"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v0() {
        let data = serde_json::from_str::<TypedData>(VALID_V0_DATA).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            Felt::from_hex_unchecked(
                "0x0778d68fe2baf73ee78a6711c29bad4722680984c1553a8035c8cb3feb5310c9"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_struct() {
        let data = serde_json::from_str::<TypedData>(VALID_V1_DATA).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            Felt::from_hex_unchecked(
                "0x045bca39274d2b7fdf7dc7c4ecf75f6549f614ce44359cc62ec106f4e5cc87b4"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_basic_types() {
        let raw = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Bool", "type": "bool" },
      { "name": "I128", "type": "i128" },
      { "name": "Classhash", "type": "ClassHash" },
      { "name": "Timestamp", "type": "timestamp" },
      { "name": "Short1", "type": "shortstring" },
      { "name": "Short2", "type": "shortstring" },
      { "name": "Short3", "type": "shortstring" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Bool": true,
    "I128": -123,
    "Classhash": "0x1234",
    "Timestamp": 1234,
    "Short1": 123,
    "Short2": "0x123",
    "Short3": "hello"
  }
}"###;

        let data = serde_json::from_str::<TypedData>(raw).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            Felt::from_hex_unchecked(
                "0x0795c7e03a0ef83c4e3dee6942ef64d4126a91cafbda207356dae1de3bed4063"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_preset() {
        let raw = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Uint", "type": "u256" },
      { "name": "Amount", "type": "TokenAmount" },
      { "name": "Id", "type": "NftId" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Uint": {
      "low": "1234",
      "high": "0x5678"
    },
    "Amount": {
      "token_address": "0x11223344",
      "amount": {
        "low": 1000000,
        "high": 0
      }
    },
    "Id": {
      "collection_address": "0x55667788",
      "token_id": {
        "low": "0x12345678",
        "high": 0
      }
    }
  }
}"###;

        let data = serde_json::from_str::<TypedData>(raw).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            Felt::from_hex_unchecked(
                "0x068b85f4061d8155c0445f7e3c6bae1e7641b88b1d3b7c034c0b4f6c30eb5049"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_simple_enum() {
        let raw = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Value", "type": "enum", "contains": "My Enum" }
    ],
    "My Enum": [
      { "name": "Variant 1", "type": "()" },
      { "name": "Variant 2", "type": "(string)" },
      { "name": "Variant 3", "type": "(u128)" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Value": {
      "Variant 2": ["tuple element"]
    }
  }
}"###;

        let data = serde_json::from_str::<TypedData>(raw).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            // This expected hash was generated with starknet.js v6.24.1, due to the expectation
            // that the following fixes, despite being merged, would never be released:
            // - https://github.com/starknet-io/starknet.js/pull/1281
            // - https://github.com/starknet-io/starknet.js/pull/1288
            Felt::from_hex_unchecked(
                "0x05cb0569ef378e0c17c07c13cb86bc6e067f824ccffd79fd49d875ecc0296124"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_enum_nested() {
        let raw = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Value", "type": "enum", "contains": "My Enum" }
    ],
    "My Enum": [
      { "name": "Variant 1", "type": "()" },
      { "name": "Variant 2", "type": "(string,My Object*)" },
      { "name": "Variant 3", "type": "(u128)" }
    ],
    "My Object": [
      { "name": "Some Selector", "type": "selector" },
      { "name": "Some Contract Address", "type": "ContractAddress" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Value": {
      "Variant 2": [
        "tuple element",
        [
          {
            "Some Selector": "transfer",
            "Some Contract Address": "0x1234"
          },
          {
            "Some Selector": "approve",
            "Some Contract Address": "0x5678"
          }
        ]
      ]
    }
  }
}"###;

        let data = serde_json::from_str::<TypedData>(raw).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            // This expected hash was generated with starknet.js v6.24.1
            Felt::from_hex_unchecked(
                "0x0470e6107a4d464e16d8f77ff673c06f6fbfe107fef1e496e53b10d3744afd42"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_message_hash_v1_with_merkletree() {
        let raw = r###"{
  "types": {
    "StarknetDomain": [
      { "name": "name", "type": "shortstring" },
      { "name": "version", "type": "shortstring" },
      { "name": "chainId", "type": "shortstring" },
      { "name": "revision", "type": "shortstring" }
    ],
    "Example Message": [
      { "name": "Value", "type": "merkletree", "contains": "My Object" }
    ],
    "My Object": [
      { "name": "Some Selector", "type": "selector" },
      { "name": "Some Contract Address", "type": "ContractAddress" }
    ]
  },
  "primaryType": "Example Message",
  "domain": {
    "name": "Starknet Example",
    "version": "1",
    "chainId": "SN_MAIN",
    "revision": "1"
  },
  "message": {
    "Value": [
      {
        "Some Selector": "selector1",
        "Some Contract Address": "0x1111"
      },
      {
        "Some Selector": "selector2",
        "Some Contract Address": "0x2222"
      },
      {
        "Some Selector": "selector3",
        "Some Contract Address": "0x3333"
      },
      {
        "Some Selector": "selector4",
        "Some Contract Address": "0x4444"
      },
      {
        "Some Selector": "selector5",
        "Some Contract Address": "0x5555"
      }
    ]
  }
}"###;

        let data = serde_json::from_str::<TypedData>(raw).unwrap();

        assert_eq!(
            data.message_hash(Felt::from_hex_unchecked("0x1234"))
                .unwrap(),
            Felt::from_hex_unchecked(
                "0x064bd27eb802de8c83ff1437394c142bbe771530a248c548fab27ac3bcd2a503"
            )
        );
    }
}
