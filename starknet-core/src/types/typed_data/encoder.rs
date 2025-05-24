use alloc::{borrow::ToOwned, vec::*};
use core::str::FromStr;
use starknet_crypto::{PedersenHasher, PoseidonHasher};

use crate::codec::Encode;
use crate::types::{ByteArray, Felt};
use crate::utils::{cairo_short_string_to_felt, get_selector_from_name};

use super::hasher::TypedDataHasher;
use super::type_definition::PresetType;
use super::{
    ArrayValue, CommonTypeReference, CompositeType, Domain, EnumDefinition, FullTypeReference,
    InlineTypeReference, ObjectValue, Revision, TypeDefinition, TypeReference, TypedDataError,
    Types, Value, ValueKind,
};

/// SNIP-12 typed data encoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoder {
    /// Type definitions for the domain separator type and user-defined custom types.
    types: Types,
    /// Domain separator.
    domain: Domain,
}

/// An iterator for encoding struct fields against its type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeFieldEncodingIter<'a, I> {
    encoder: &'a Encoder,
    value: &'a ObjectValue,
    fields: I,
}

impl Encoder {
    /// Creates a new [`Encoder`]. Returns `Err` if `types` and `domain` use different revisions.
    pub fn new(types: Types, domain: Domain) -> Result<Self, TypedDataError> {
        if types.revision() == domain.revision {
            Ok(Self { types, domain })
        } else {
            Err(TypedDataError::InconsistentRevision {
                types: types.revision(),
                domain: domain.revision,
            })
        }
    }

    /// Gets the SNIP-12 revision of this [`Encoder`].
    pub const fn revision(&self) -> Revision {
        // No need to check against `self.types` as revision consistency is maintained as an
        // invariant.
        self.domain.revision
    }

    /// Gets a reference to the encoder's defined custom types.
    pub fn types(&self) -> &Types {
        &self.types
    }

    /// Gets the encoder's domain.
    pub fn domain(&self) -> Domain {
        self.domain
    }

    /// Encodes a typed data value into a `Felt` hash according to SNIP-12 specification.
    ///
    /// Takes a type reference and its corresponding value, then recursively encodes the value
    /// based on its type. Handles primitive types, arrays, structs, enums, and preset types.
    pub fn encode_value<R>(&self, type_ref: &R, value: &Value) -> Result<Felt, TypedDataError>
    where
        R: TypeReference,
    {
        match self.revision() {
            Revision::V0 => self.encode_value_with_hasher::<PedersenHasher, R>(type_ref, value),
            Revision::V1 => self.encode_value_with_hasher::<PoseidonHasher, R>(type_ref, value),
        }
    }

    /// Encodes a composite type (struct) value by hashing the type hash followed by
    /// the encoded values of all its fields in order.
    ///
    /// This is a low-level API for use cases that require introspection. To simply hash a SNIP-12
    /// message, it's recommended to use [`message_hash()`](super::TypedData::message_hash)
    pub fn encode_composite<T>(
        &self,
        type_hash: Felt,
        struct_def: &T,
        value: &ObjectValue,
    ) -> Result<Felt, TypedDataError>
    where
        T: CompositeType,
    {
        match self.revision() {
            Revision::V0 => {
                self.encode_composite_with_hasher::<PedersenHasher, T>(type_hash, struct_def, value)
            }
            Revision::V1 => {
                self.encode_composite_with_hasher::<PoseidonHasher, T>(type_hash, struct_def, value)
            }
        }
    }

    /// Encodes an enum value by hashing the type hash, variant index, and encoded
    /// values of the variant's tuple elements.
    pub fn encode_enum(
        &self,
        enum_def: &EnumDefinition,
        value: &ObjectValue,
    ) -> Result<Felt, TypedDataError> {
        match self.revision() {
            Revision::V0 => self.encode_enum_with_hasher::<PedersenHasher>(enum_def, value),
            Revision::V1 => self.encode_enum_with_hasher::<PoseidonHasher>(enum_def, value),
        }
    }

    /// Encodes a Merkle tree from an array of values by first encoding each leaf
    /// according to its type, then computing the Merkle root hash.
    pub fn encode_merkletree(
        &self,
        leaf_type_def: &InlineTypeReference,
        value: &ArrayValue,
    ) -> Result<Felt, TypedDataError> {
        match self.revision() {
            Revision::V0 => {
                self.encode_merkletree_with_hasher::<PedersenHasher>(leaf_type_def, value)
            }
            Revision::V1 => {
                self.encode_merkletree_with_hasher::<PoseidonHasher>(leaf_type_def, value)
            }
        }
    }

    /// Encodes the fields of a composite type (struct) for typed data hashing.
    ///
    /// This function validates that the number of fields in the provided value matches
    /// the struct definition and returns an iterator that can be used to encode each
    /// field according to SNIP-12.
    pub fn encode_composite_fields<'a, T>(
        &'a self,
        struct_def: &'a T,
        value: &'a ObjectValue,
    ) -> Result<
        CompositeFieldEncodingIter<'a, <T as CompositeType>::FieldIterator<'a>>,
        TypedDataError,
    >
    where
        T: CompositeType,
    {
        if value.fields.len() != struct_def.field_len() {
            return Err(TypedDataError::StructFieldCountMismatch {
                expected: struct_def.field_len(),
                actual: value.fields.len(),
            });
        }

        Ok(CompositeFieldEncodingIter {
            encoder: self,
            value,
            fields: struct_def.field_iter(),
        })
    }

    fn encode_value_with_hasher<H, R>(
        &self,
        type_ref: &R,
        value: &Value,
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

                        self.encode_composite_with_hasher::<H, _>(type_hash, struct_def, obj_value)?
                    }
                    TypeDefinition::Enum(enum_def) => {
                        if type_ref.must_be_struct() {
                            return Err(TypedDataError::UnexpectedEnum(name.to_owned()));
                        }

                        self.encode_enum_with_hasher::<H>(enum_def, obj_value)?
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
                    hasher.update(self.encode_value_with_hasher::<H, _>(element_type, element)?);
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

                self.encode_merkletree_with_hasher::<H>(leaf, arr_value)?
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

                self.encode_composite_with_hasher::<H, _>(
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

                self.encode_composite_with_hasher::<H, _>(
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

                self.encode_composite_with_hasher::<H, _>(
                    PresetType::NftId.type_hash(self.revision()),
                    &PresetType::NftId,
                    obj_value,
                )?
            }
        };

        Ok(encoded)
    }

    fn encode_composite_with_hasher<H, T>(
        &self,
        type_hash: Felt,
        struct_def: &T,
        value: &ObjectValue,
    ) -> Result<Felt, TypedDataError>
    where
        H: TypedDataHasher,
        T: CompositeType,
    {
        let mut hasher = H::default();
        hasher.update(type_hash);

        for field in self.encode_composite_fields::<T>(struct_def, value)? {
            hasher.update(field?);
        }

        Ok(hasher.finalize())
    }

    fn encode_enum_with_hasher<H>(
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
            hasher.update(self.encode_value_with_hasher::<H, _>(tuple_slot_def, tuple_slot_value)?);
        }

        // Enum repr must have only one field
        if value_field_iter.next().is_some() {
            return Err(TypedDataError::InvalidEnumFieldCount);
        }

        Ok(hasher.finalize())
    }

    fn encode_merkletree_with_hasher<H>(
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
            .map(|element| self.encode_value_with_hasher::<H, _>(leaf_type_def, element))
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

impl<'a, I> Iterator for CompositeFieldEncodingIter<'a, I>
where
    I: Iterator<Item = (&'a str, &'a FullTypeReference)>,
{
    type Item = Result<Felt, TypedDataError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.fields.next().map(
            |(field_name, field_type)| match self.value.fields.get(field_name) {
                Some(value) => self.encoder.encode_value(field_type, value),
                None => Err(TypedDataError::FieldNotFound(field_name.to_owned())),
            },
        )
    }
}
