use alloc::{
    borrow::{Cow, ToOwned},
    collections::BTreeMap,
    string::*,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    types::{typed_data::CommonTypeReference, Felt},
    utils::starknet_keccak,
};

use super::{
    error::TypedDataError,
    revision::Revision,
    type_definition::{PresetType, TypeDefinition},
    TypeReference,
};

#[cfg(feature = "std")]
type RandomState = std::hash::RandomState;
#[cfg(not(feature = "std"))]
type RandomState = foldhash::fast::RandomState;

const DOMAIN_TYPE_NAME_V0: &str = "StarkNetDomain";
const DOMAIN_TYPE_NAME_V1: &str = "StarknetDomain";

/// The user-defined types section of a SNIP-12 [`TypedData`](super::TypedData) instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Types {
    /// The SNIP-12 revision as inferred from the supplied domain type definition. The actual
    /// domain type definition is not stored as it's universal for all instances.
    revision: Revision,
    /// Definitions for types other than the domain type.
    user_defined_types: IndexMap<String, TypeDefinition, RandomState>,
}

enum SignatureGenerator<'a> {
    UserDefinedType(&'a TypeDefinition),
    PresetType(&'static PresetType),
}

impl Types {
    /// Initializes a new instance of `Types`.
    pub fn new(revision: Revision, types: IndexMap<String, TypeDefinition, RandomState>) -> Self {
        Self {
            revision,
            user_defined_types: types,
        }
    }

    /// Gets the revision implied from the definition of the domain type.
    ///
    /// Returns [`Revision::V0`] if and only if only `StarkNetDomain` is defined.
    ///
    /// Returns [`Revision::V1`] if and only if only `StarknetDomain` is defined.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Gets the type definition by name. Returns `None` if not defined.
    pub fn get_type(&self, type_name: &str) -> Option<&TypeDefinition> {
        self.user_defined_types.get(type_name)
    }

    /// Gets the SNIP-12 type hash of a user-defined type.
    ///
    /// Returns `Err` if the type or any of its dependancies are not defined.
    pub fn get_type_hash(&self, type_name: &str) -> Result<Felt, TypedDataError> {
        let type_def = self
            .get_type(type_name)
            .ok_or_else(|| TypedDataError::CustomTypeNotFound(type_name.to_owned()))?;

        let mut full_signature = String::new();
        SignatureGenerator::UserDefinedType(type_def).write_signature(
            type_name,
            &mut full_signature,
            self.revision(),
        );

        let mut dependency_signatures: BTreeMap<&str, SignatureGenerator<'_>> = BTreeMap::new();
        self.collect_dep_sigs_from_type_def(&mut dependency_signatures, type_def)?;

        for (name, sig) in dependency_signatures {
            sig.write_signature(name, &mut full_signature, self.revision());
        }

        Ok(starknet_keccak(full_signature.as_bytes()))
    }

    fn collect_dep_sigs_from_type_ref<'a, R>(
        &'a self,
        signatures: &mut BTreeMap<&'a str, SignatureGenerator<'a>>,
        type_ref: &'a R,
    ) -> Result<(), TypedDataError>
    where
        R: TypeReference,
    {
        #[allow(clippy::match_same_arms)]
        match type_ref.common() {
            CommonTypeReference::Custom(name) => {
                let type_def = self
                    .get_type(name)
                    .ok_or_else(|| TypedDataError::CustomTypeNotFound(name.to_owned()))?;

                // No need to advance further if the type has already been visited
                if signatures
                    .insert(name, SignatureGenerator::UserDefinedType(type_def))
                    .is_none()
                {
                    self.collect_dep_sigs_from_type_def(signatures, type_def)?;
                }
            }
            CommonTypeReference::Array(element) => {
                self.collect_dep_sigs_from_type_ref(signatures, element)?;
            }
            CommonTypeReference::MerkleTree(_) => {
                // SNIP-12 is a bit vague on whether the leaf type here should be collected as
                // dependency, as it's unclear whether `merkletree`'s leaf type counts as being
                // "referenced" by the parent type, given that `merkletree`'s own type encoding does
                // not include any information of the leaf type.
                //
                // Since the `starknet.js` implementation discards the leaf type,, we do the same
                // here to be compatible.
            }
            // Preset types
            CommonTypeReference::U256 => {
                signatures.insert(
                    PresetType::U256.name(),
                    SignatureGenerator::PresetType(&PresetType::U256),
                );
            }
            CommonTypeReference::TokenAmount => {
                signatures.insert(
                    PresetType::TokenAmount.name(),
                    SignatureGenerator::PresetType(&PresetType::TokenAmount),
                );

                // `TokenAmount` depends on `u256`
                signatures.insert(
                    PresetType::U256.name(),
                    SignatureGenerator::PresetType(&PresetType::U256),
                );
            }
            CommonTypeReference::NftId => {
                signatures.insert(
                    PresetType::NftId.name(),
                    SignatureGenerator::PresetType(&PresetType::NftId),
                );

                // `NftId` depends on `u256`
                signatures.insert(
                    PresetType::U256.name(),
                    SignatureGenerator::PresetType(&PresetType::U256),
                );
            }
            // Basic types. Nothing to collect.
            CommonTypeReference::Felt
            | CommonTypeReference::Bool
            | CommonTypeReference::String
            | CommonTypeReference::Selector
            | CommonTypeReference::U128
            | CommonTypeReference::I128
            | CommonTypeReference::ContractAddress
            | CommonTypeReference::ClassHash
            | CommonTypeReference::Timestamp
            | CommonTypeReference::ShortString => {}
        }

        Ok(())
    }

    fn collect_dep_sigs_from_type_def<'a>(
        &'a self,
        signatures: &mut BTreeMap<&'a str, SignatureGenerator<'a>>,
        type_def: &'a TypeDefinition,
    ) -> Result<(), TypedDataError> {
        match type_def {
            TypeDefinition::Struct(struct_def) => {
                for field in &struct_def.fields {
                    self.collect_dep_sigs_from_type_ref(signatures, &field.r#type)?;
                }
            }
            TypeDefinition::Enum(enum_def) => {
                for variant in &enum_def.variants {
                    for tuple_type in &variant.tuple_types {
                        self.collect_dep_sigs_from_type_ref(signatures, tuple_type)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl SignatureGenerator<'_> {
    fn write_signature(&self, name: &str, signature: &mut String, revision: Revision) {
        match self {
            Self::UserDefinedType(TypeDefinition::Struct(struct_def)) => {
                Self::write_escaped_name(name, signature, revision);
                signature.push('(');

                let mut field_iter = struct_def.fields.iter().peekable();
                while let Some(field) = field_iter.next() {
                    Self::write_escaped_name(&field.name, signature, revision);
                    signature.push(':');
                    Self::write_escaped_name(
                        &field.r#type.signature_ref_repr(),
                        signature,
                        revision,
                    );

                    if field_iter.peek().is_some() {
                        signature.push(',');
                    };
                }

                signature.push(')');
            }
            Self::UserDefinedType(TypeDefinition::Enum(enum_def)) => {
                Self::write_escaped_name(name, signature, revision);
                signature.push('(');

                let mut variant_iter = enum_def.variants.iter().peekable();
                while let Some(variant) = variant_iter.next() {
                    Self::write_escaped_name(&variant.name, signature, revision);

                    // This is technically a SNIP-12 violation. Unfortunately, as the de-facto
                    // standard, starknet.js implemented it incorrectly. Despite the fix being
                    // merged (https://github.com/starknet-io/starknet.js/issues/1286) it's expected
                    // to never be released.
                    //
                    // Context: https://github.com/starknet-io/starknet.js/pull/1292
                    signature.push(':');

                    signature.push('(');

                    let mut tuple_type_iter = variant.tuple_types.iter().peekable();
                    while let Some(tuple_type) = tuple_type_iter.next() {
                        Self::write_escaped_name(
                            &tuple_type.signature_ref_repr(),
                            signature,
                            revision,
                        );
                        if tuple_type_iter.peek().is_some() {
                            signature.push(',')
                        };
                    }

                    signature.push_str(if variant_iter.peek().is_some() {
                        "),"
                    } else {
                        ")"
                    });
                }

                signature.push(')');
            }
            Self::PresetType(preset) => {
                signature.push_str(preset.type_signature(revision));
            }
        }
    }

    fn write_escaped_name(name: &str, signature: &mut String, revision: Revision) {
        match revision {
            Revision::V0 => {
                signature.push_str(name);
            }
            Revision::V1 => {
                // TODO: check if this can ever fail
                signature.push_str(&serde_json::to_string(name).unwrap());
            }
        }
    }
}

impl Serialize for Types {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut types =
            IndexMap::<&str, Cow<'_, TypeDefinition>, RandomState>::with_capacity_and_hasher(
                self.user_defined_types.len() + 1,
                RandomState::default(),
            );

        match self.revision {
            Revision::V0 => {
                types.insert(DOMAIN_TYPE_NAME_V0, Cow::Owned(TypeDefinition::v0_domain()));
            }
            Revision::V1 => {
                types.insert(DOMAIN_TYPE_NAME_V1, Cow::Owned(TypeDefinition::v1_domain()));
            }
        }

        for (name, user_type) in &self.user_defined_types {
            types.insert(name, Cow::Borrowed(user_type));
        }

        types.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Types {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut raw = IndexMap::<String, TypeDefinition, RandomState>::deserialize(deserializer)?;

        if let Some(domain_v1) = raw.shift_remove(DOMAIN_TYPE_NAME_V1) {
            if raw.contains_key(DOMAIN_TYPE_NAME_V0) {
                Err(serde::de::Error::custom(
                    "conflicting domain type definitions",
                ))
            } else if !domain_v1.is_v1_domain() {
                Err(serde::de::Error::custom(
                    "invalid domain type definition for revision 1",
                ))
            } else {
                Ok(Self {
                    revision: Revision::V1,
                    user_defined_types: raw,
                })
            }
        } else if let Some(domain_v0) = raw.shift_remove(DOMAIN_TYPE_NAME_V0) {
            if domain_v0.is_v0_domain() {
                Ok(Self {
                    revision: Revision::V0,
                    user_defined_types: raw,
                })
            } else {
                Err(serde::de::Error::custom(
                    "invalid domain type definition for revision 0",
                ))
            }
        } else {
            Err(serde::de::Error::custom("missing domain type definition"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_V1_DATA: &str = r###"{
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
}"###;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_revision_0_serde() {
        let raw = r###"{
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
}"###;

        let types = serde_json::from_str::<Types>(raw).unwrap();
        assert_eq!(types.revision, Revision::V0);
        assert_eq!(types.user_defined_types.len(), 2);

        // Comparing on `Value` avoids false positives from formatting.
        assert_eq!(
            serde_json::to_value(&types).unwrap(),
            serde_json::from_str::<serde_json::Value>(raw).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_revision_1_serde() {
        let types = serde_json::from_str::<Types>(VALID_V1_DATA).unwrap();
        assert_eq!(types.revision, Revision::V1);
        assert_eq!(types.user_defined_types.len(), 2);

        // Comparing on `Value` avoids false positives from formatting.
        assert_eq!(
            serde_json::to_value(&types).unwrap(),
            serde_json::from_str::<serde_json::Value>(VALID_V1_DATA).unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_revision_1_type_hash() {
        let types = serde_json::from_str::<Types>(VALID_V1_DATA).unwrap();
        assert_eq!(
            types.get_type_hash("Example Message").unwrap(),
            Felt::from_hex_unchecked(
                "0x01ef2892585a840aee9165aac7aaf811ba2f8619e43c119bd76a6109f81cecc3"
            )
        );
        assert_eq!(
            types.get_type_hash("My Object").unwrap(),
            Felt::from_hex_unchecked(
                "0x02f0ee9d399d4e7ccbc5d7e96df767296cc4b8a516600c121b393427ae3779f2"
            )
        );
    }
}
