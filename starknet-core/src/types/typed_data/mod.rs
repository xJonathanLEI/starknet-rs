use alloc::{borrow::ToOwned, format, vec::*};
use core::str::FromStr;

use serde::Deserialize;
use starknet_crypto::{PedersenHasher, PoseidonHasher};

use crate::{
    codec::Encode,
    types::Felt,
    utils::{cairo_short_string_to_felt, get_selector_from_name},
};

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
use type_definition::{CompositeType, PresetType};
pub use type_definition::{
    EnumDefinition, FieldDefinition, StructDefinition, TypeDefinition, VariantDefinition,
};

mod type_reference;
use type_reference::{CommonTypeReference, TypeReference};
pub use type_reference::{ElementTypeReference, FullTypeReference, InlineTypeReference};

mod types;
pub use types::Types;

mod value;
pub use value::{ArrayValue, ObjectValue, Value, ValueKind};

use super::ByteArray;

/// Cairo short string encoding of `StarkNet Message`.
const STARKNET_MESSAGE_PREFIX: Felt = Felt::from_raw([
    257012186512350467,
    18446744073709551605,
    10480951322775611302,
    16156019428408348868,
]);

/// SNIP-12 typed data hashes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDataHashes {
    /// The final hash of the entire message.
    pub hash: Felt,
    /// Hash of the `domain` component.
    pub domain_hash: Felt,
    /// Hash of the `primary_type` component.
    pub type_hash: Felt,
    /// Hash of the `message` component.
    pub message_hash: Felt,
    /// Encoded object fields.
    pub fields_hashes: Vec<Felt>,
}

/// SNIP-12 typed data for off-chain signatures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedData {
    /// Type definitions for the domain separator type and user-defined custom types.
    pub types: Types,
    /// Domain separator.
    pub domain: Domain,
    /// Reference to the primary/entrypoint type that the `message` field represents.
    pub primary_type: InlineTypeReference,
    /// The main message data to be signed, structured as per `primary_type`'s definition.
    pub message: Value,
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
        if types.revision() == domain.revision {
            Ok(Self {
                types,
                domain,
                primary_type,
                message,
            })
        } else {
            Err(TypedDataError::InconsistentRevision {
                types: types.revision(),
                domain: domain.revision,
            })
        }
    }

    /// Gets the SNIP-12 revision of this [`TypedData`].
    pub const fn revision(&self) -> Revision {
        // No need to check against `self.types` as revision consistency is maintained as an
        // invariant.
        self.domain.revision
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
        hasher.update(self.domain.encoded_hash());
        hasher.update(address);
        hasher.update(self.encode_value::<H, _>(&self.primary_type, &self.message)?);
        Ok(hasher.finalize())
    }

    /// Computes and returns the finalized SNIP-12 typed data hash along with the set of hashes for the message.
    pub fn hashes<H>(&self, address: Felt) -> Result<TypedDataHashes, TypedDataError>
    where
        H: TypedDataHasher,
    {
        let mut hasher = H::default();
        hasher.update(STARKNET_MESSAGE_PREFIX);

        let domain_hash = self.domain.encoded_hash();
        hasher.update(domain_hash);

        hasher.update(address);

        let type_hash = self
            .types
            .get_type_hash(&self.primary_type.signature_ref_repr())?;

        let mut fields_hashes = Vec::new();
        let message_hash = self.encode_value_with_introspect::<H, _>(
            &self.primary_type,
            &self.message,
            Some(&mut fields_hashes),
        )?;
        hasher.update(message_hash);

        Ok(TypedDataHashes {
            hash: hasher.finalize(),
            domain_hash,
            type_hash,
            message_hash,
            fields_hashes,
        })
    }

    fn encode_value<H, R>(&self, type_ref: &R, value: &Value) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
        R: TypeReference,
    {
        self.encode_value_with_introspect::<H, R>(type_ref, value, None)
    }

    fn encode_value_with_introspect<H, R>(
        &self,
        type_ref: &R,
        value: &Value,
        fields_hashes: Option<&mut Vec<Felt>>,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
        R: TypeReference,
    {
        let encoded = match type_ref.common() {
            CommonTypeReference::Custom(name) => {
                // This is either an enum or struct. Depending on the type of the type reference we
                // may or may not care which one it is.

                let type_def = self
                    .types
                    .get_type(name)
                    .ok_or_else(|| TypedDataError::CustomTypeNotFound(name.to_owned()))?;
                let type_hash = self.types.get_type_hash(name)?;

                // Both struct and enum require the value to be represented as an object
                let obj_value = match value {
                    Value::Object(obj_value) => obj_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Object],
                            actual: value.kind(),
                        });
                    }
                };

                match type_def {
                    TypeDefinition::Struct(struct_def) => {
                        if type_ref.must_be_enum() {
                            return Err(TypedDataError::UnexpectedStruct(name.to_owned()));
                        }

                        self.encode_composite_with_introspect::<H, _>(
                            type_hash,
                            struct_def,
                            obj_value,
                            fields_hashes,
                        )?
                    }
                    TypeDefinition::Enum(enum_def) => {
                        if type_ref.must_be_struct() {
                            return Err(TypedDataError::UnexpectedEnum(name.to_owned()));
                        }

                        self.encode_enum::<H>(enum_def, obj_value)?
                    }
                }
            }
            CommonTypeReference::Array(element_type) => {
                let arr_value = match value {
                    Value::Array(arr_value) => arr_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Array],
                            actual: value.kind(),
                        });
                    }
                };

                let mut hasher = H::default();

                for element in &arr_value.elements {
                    hasher.update(self.encode_value::<H, _>(element_type, element)?);
                }

                hasher.finalize()
            }
            // Technically, SNIP-12 specifies that `felt` and `shortstring` should behave
            // differently. Unfortunately, `starknet.js` ships a buggy implementation that treats
            // both types the same. We deviate from the spec here to be compatible:
            //
            // https://github.com/starknet-io/starknet.js/issues/1039
            CommonTypeReference::Felt | CommonTypeReference::ShortString => match value {
                Value::String(str_value) => {
                    // This is to reimplement the `starknet.js` bug
                    let decoded_as_raw = match str_value.strip_prefix("0x") {
                        Some(hexadecimal) => {
                            if hexadecimal.chars().all(|c| c.is_ascii_hexdigit()) {
                                Felt::from_hex(str_value).ok()
                            } else {
                                None
                            }
                        }
                        None => {
                            if str_value.chars().all(|c| c.is_ascii_digit()) {
                                Felt::from_dec_str(str_value).ok()
                            } else {
                                None
                            }
                        }
                    };

                    match decoded_as_raw {
                        Some(raw) => raw,
                        None => cairo_short_string_to_felt(str_value).map_err(|_| {
                            TypedDataError::InvalidShortString(str_value.to_owned())
                        })?,
                    }
                }
                Value::UnsignedInteger(int_value) => (*int_value).into(),
                Value::SignedInteger(_)
                | Value::Boolean(_)
                | Value::Object(_)
                | Value::Array(_) => {
                    return Err(TypedDataError::UnexpectedValueType {
                        expected: &[ValueKind::String, ValueKind::UnsignedInteger],
                        actual: value.kind(),
                    });
                }
            },
            CommonTypeReference::Bool => match value {
                Value::Boolean(false) => Felt::ZERO,
                Value::Boolean(true) => Felt::ONE,
                Value::String(_)
                | Value::UnsignedInteger(_)
                | Value::SignedInteger(_)
                | Value::Object(_)
                | Value::Array(_) => {
                    return Err(TypedDataError::UnexpectedValueType {
                        expected: &[ValueKind::Boolean],
                        actual: value.kind(),
                    });
                }
            },
            CommonTypeReference::String => {
                let str_value = match value {
                    Value::String(str_value) => str_value,
                    Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::String],
                            actual: value.kind(),
                        });
                    }
                };

                match self.revision() {
                    Revision::V0 => {
                        // In revision 0 `string` is treated as short string.

                        cairo_short_string_to_felt(str_value)
                            .map_err(|_| TypedDataError::InvalidShortString(str_value.to_owned()))?
                    }
                    Revision::V1 => {
                        // In revision 1 `string` is treated as `ByteArray`.

                        let mut hasher = H::default();

                        // `ByteArray` encoding never fails
                        ByteArray::from(str_value.as_str())
                            .encode(&mut hasher)
                            .unwrap();

                        hasher.finalize()
                    }
                }
            }
            CommonTypeReference::Selector => {
                let str_value = match value {
                    Value::String(str_value) => str_value,
                    Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::String],
                            actual: value.kind(),
                        });
                    }
                };

                get_selector_from_name(str_value)
                    .map_err(|_| TypedDataError::InvalidSelector(str_value.to_owned()))?
            }
            CommonTypeReference::MerkleTree(leaf) => {
                let arr_value = match value {
                    Value::Array(arr_value) => arr_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Array],
                            actual: value.kind(),
                        });
                    }
                };

                self.encode_merkletree::<H>(leaf, arr_value)?
            }
            // Technically `timestamp` should be restricted to `u64` range but `starknet.js` allows
            // it to be treated the same way as `u128`.
            CommonTypeReference::Timestamp | CommonTypeReference::U128 => {
                let int_value = match value {
                    Value::UnsignedInteger(int_value) => *int_value,
                    // Technically SNIP-12 does not allow strings here but `starknet.js` does, so we
                    // do it here to be compatible.
                    Value::String(str_value) => match str_value.strip_prefix("0x") {
                        Some(hex_str) => u128::from_str_radix(hex_str, 16),
                        None => str_value.parse::<u128>(),
                    }
                    .map_err(|_| TypedDataError::InvalidNumber(str_value.to_owned()))?,
                    Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::UnsignedInteger, ValueKind::String],
                            actual: value.kind(),
                        });
                    }
                };

                int_value.into()
            }
            CommonTypeReference::I128 => {
                let int_value = match value {
                    Value::SignedInteger(int_value) => *int_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::UnsignedInteger, ValueKind::String],
                            actual: value.kind(),
                        });
                    }
                };

                let mut encoded = Felt::ZERO;

                // Encoding `i128` never fails
                int_value.encode(&mut encoded).unwrap();

                encoded
            }
            CommonTypeReference::ContractAddress | CommonTypeReference::ClassHash => {
                let str_value = match value {
                    Value::String(str_value) => str_value,
                    Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Object(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::String],
                            actual: value.kind(),
                        });
                    }
                };

                Felt::from_str(str_value)
                    .map_err(|_| TypedDataError::InvalidNumber(str_value.to_owned()))?
            }
            CommonTypeReference::U256 => {
                let obj_value = match value {
                    Value::Object(obj_value) => obj_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Object],
                            actual: value.kind(),
                        });
                    }
                };

                self.encode_composite::<H, _>(
                    PresetType::U256.type_hash(self.revision()),
                    &PresetType::U256,
                    obj_value,
                )?
            }
            CommonTypeReference::TokenAmount => {
                let obj_value = match value {
                    Value::Object(obj_value) => obj_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Object],
                            actual: value.kind(),
                        });
                    }
                };

                self.encode_composite::<H, _>(
                    PresetType::TokenAmount.type_hash(self.revision()),
                    &PresetType::TokenAmount,
                    obj_value,
                )?
            }
            CommonTypeReference::NftId => {
                let obj_value = match value {
                    Value::Object(obj_value) => obj_value,
                    Value::String(_)
                    | Value::UnsignedInteger(_)
                    | Value::SignedInteger(_)
                    | Value::Boolean(_)
                    | Value::Array(_) => {
                        return Err(TypedDataError::UnexpectedValueType {
                            expected: &[ValueKind::Object],
                            actual: value.kind(),
                        });
                    }
                };

                self.encode_composite::<H, _>(
                    PresetType::NftId.type_hash(self.revision()),
                    &PresetType::NftId,
                    obj_value,
                )?
            }
        };

        Ok(encoded)
    }

    fn encode_composite<H, T>(
        &self,
        type_hash: Felt,
        struct_def: &T,
        value: &ObjectValue,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
        T: CompositeType,
    {
        self.encode_composite_with_introspect::<H, T>(type_hash, struct_def, value, None)
    }

    fn encode_composite_with_introspect<H, T>(
        &self,
        type_hash: Felt,
        struct_def: &T,
        value: &ObjectValue,
        mut fields_hashes: Option<&mut Vec<Felt>>,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
        T: CompositeType,
    {
        let mut hasher = H::default();
        hasher.update(type_hash);

        if value.fields.len() != struct_def.field_len() {
            return Err(TypedDataError::StructFieldCountMismatch {
                expected: struct_def.field_len(),
                actual: value.fields.len(),
            });
        }

        for (field_name, field_type) in struct_def.field_iter() {
            let value = value
                .fields
                .get(field_name)
                .ok_or_else(|| TypedDataError::FieldNotFound(field_name.to_owned()))?;
            let field_hash = self.encode_value_with_introspect::<H, _>(field_type, value, None)?;
            hasher.update(field_hash);
            if let Some(hashes) = fields_hashes.as_mut() {
                hashes.push(field_hash);
            }
        }

        Ok(hasher.finalize())
    }

    fn encode_enum<H>(
        &self,
        enum_def: &EnumDefinition,
        value: &ObjectValue,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
    {
        let mut hasher = H::default();

        // Here we're NOT hashing the enum type hash. This is technically a SNIP-12 violation.
        // Unfortunately, as the de-facto standard, starknet.js implemented it incorrectly. Despite
        // the fix being merged (https://github.com/starknet-io/starknet.js/pull/1281) it's expected
        // to never be released.
        //
        // Context: https://github.com/starknet-io/starknet.js/pull/1292

        let mut value_field_iter = value.fields.iter();

        let (variant_name, variant_value) = value_field_iter
            .next()
            .ok_or(TypedDataError::InvalidEnumFieldCount)?;
        let tuple_values = match variant_value {
            Value::Array(arr_value) => arr_value,
            Value::String(_)
            | Value::UnsignedInteger(_)
            | Value::SignedInteger(_)
            | Value::Boolean(_)
            | Value::Object(_) => {
                return Err(TypedDataError::UnexpectedValueType {
                    expected: &[ValueKind::Array],
                    actual: variant_value.kind(),
                });
            }
        };

        let (variant_ind, variant_def) = enum_def
            .variants
            .iter()
            .enumerate()
            .find(|(_, variant)| &variant.name == variant_name)
            .ok_or_else(|| TypedDataError::EnumVariantNotFound(variant_name.to_owned()))?;
        hasher.update(variant_ind.into());

        if variant_def.tuple_types.len() != tuple_values.elements.len() {
            return Err(TypedDataError::EnumElementCountMismatch {
                expected: variant_def.tuple_types.len(),
                actual: tuple_values.elements.len(),
            });
        }

        for (tuple_slot_def, tuple_slot_value) in variant_def
            .tuple_types
            .iter()
            .zip(tuple_values.elements.iter())
        {
            hasher.update(self.encode_value::<H, _>(tuple_slot_def, tuple_slot_value)?);
        }

        // Enum repr must have only one field
        if value_field_iter.next().is_some() {
            return Err(TypedDataError::InvalidEnumFieldCount);
        }

        Ok(hasher.finalize())
    }

    fn encode_merkletree<H>(
        &self,
        leaf_type_def: &InlineTypeReference,
        value: &ArrayValue,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
    {
        // It's unclear how an empty Merkle tree should be hashed. Interestingly, `starknet.js` gets
        // stuck in an infinite recursion loop when fed with an empty list of leaves. So it should
        // be safe to reject empty Merkle trees here.
        if value.elements.is_empty() {
            return Err(TypedDataError::EmptyMerkleTree);
        }

        let element_hashes = value
            .elements
            .iter()
            .map(|element| self.encode_value::<H, _>(leaf_type_def, element))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::compute_merkle_root::<H>(&element_hashes))
    }

    fn compute_merkle_root<H>(layer: &[Felt]) -> Felt
    where
        H: TypedDataHasher,
    {
        let mut new_layer = Vec::with_capacity(layer.len().div_ceil(2));
        for chunk in layer.chunks(2) {
            new_layer.push(if chunk.len() == 2 {
                if chunk[0] <= chunk[1] {
                    H::hash_two_elements(chunk[0], chunk[1])
                } else {
                    H::hash_two_elements(chunk[1], chunk[0])
                }
            } else {
                H::hash_two_elements(Felt::ZERO, chunk[0])
            })
        }

        // TODO: refactor to remove recursion and reuse a single buffer
        if new_layer.len() == 1 {
            new_layer[0]
        } else {
            Self::compute_merkle_root::<H>(&new_layer)
        }
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
