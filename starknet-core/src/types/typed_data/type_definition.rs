use alloc::{format, string::*, vec::*};
use core::str::FromStr;
use serde::{de::Unexpected, Deserialize, Serialize, Serializer};

use super::{
    revision::Revision,
    type_reference::{FullTypeReference, InlineTypeReference},
};
use crate::types::typed_data::type_reference::TypeReference;
use crate::{types::Felt, utils::starknet_keccak};

/// Custom SNIP-12 type definition, typically used in the `types` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefinition {
    /// Struct type definition.
    Struct(StructDefinition),
    /// Enum type definition.
    Enum(EnumDefinition),
}

/// Definition of a custom SNIP-12 struct type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDefinition {
    /// Struct fields.
    pub fields: Vec<FieldDefinition>,
}

/// Definition of a custom SNIP-12 struct field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinition {
    /// Name of the field.
    pub name: String,
    /// Type of the field.
    pub r#type: FullTypeReference,
}

impl FieldDefinition {
    /// Initializes a new field definition.
    pub fn new(name: &str, r#type: FullTypeReference) -> Self {
        Self {
            name: name.to_string(),
            r#type,
        }
    }
}

/// Definition of a custom SNIP-12 enum type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDefinition {
    /// Enum variants.
    pub variants: Vec<VariantDefinition>,
}

/// Definition of a custom SNIP-12 enum type variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantDefinition {
    /// Name of the variant.
    pub name: String,
    /// Types of the elements of the variant's list of data.
    pub tuple_types: Vec<InlineTypeReference>,
}

/// Internal trait for working with both user-defined types and preset types at the same time.
pub(crate) trait CompositeType {
    fn field_iter(&self) -> impl Iterator<Item = (&str, &FullTypeReference)>;

    fn field_len(&self) -> usize;
}

/// Internal type for type signature generation for preset types.
pub(crate) enum PresetType {
    U256,
    TokenAmount,
    NftId,
}

/// Internal type for implementing [`TypeDefinition`] deserialization.
enum FieldOrVariantDefinition {
    Field(FieldDefinition),
    Variant(VariantDefinition),
}

impl TypeDefinition {
    pub(crate) fn is_v0_domain(&self) -> bool {
        match self {
            Self::Struct(def) => {
                def.fields.len() == 3
                    && def.fields[0].name == "name"
                    && def.fields[0].r#type == FullTypeReference::Felt
                    && def.fields[1].name == "version"
                    && def.fields[1].r#type == FullTypeReference::Felt
                    && def.fields[2].name == "chainId"
                    && def.fields[2].r#type == FullTypeReference::Felt
            }
            Self::Enum(_) => false,
        }
    }

    pub(crate) fn is_v1_domain(&self) -> bool {
        match self {
            Self::Struct(def) => {
                def.fields.len() == 4
                    && def.fields[0].name == "name"
                    && def.fields[0].r#type == FullTypeReference::ShortString
                    && def.fields[1].name == "version"
                    && def.fields[1].r#type == FullTypeReference::ShortString
                    && def.fields[2].name == "chainId"
                    && def.fields[2].r#type == FullTypeReference::ShortString
                    && def.fields[3].name == "revision"
                    && def.fields[3].r#type == FullTypeReference::ShortString
            }
            Self::Enum(_) => false,
        }
    }
}

impl PresetType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::U256 => "u256",
            Self::TokenAmount => "TokenAmount",
            Self::NftId => "NftId",
        }
    }

    pub const fn type_signature(&self, revision: Revision) -> &'static str {
        match self {
            Self::U256 => match revision {
                Revision::V0 => "u256(low:u128,high:u128)",
                Revision::V1 => "\"u256\"(\"low\":\"u128\",\"high\":\"u128\")",
            },
            Self::TokenAmount => match revision {
                Revision::V0 => "TokenAmount(token_address:ContractAddress,amount:u256)",
                Revision::V1 => {
                    "\"TokenAmount\"(\"token_address\":\"ContractAddress\",\"amount\":\"u256\")"
                }
            },
            Self::NftId => match revision {
                Revision::V0 => "NftId(collection_address:ContractAddress,token_id:u256)",
                Revision::V1 => {
                    "\"NftId\"(\"collection_address\":\"ContractAddress\",\"token_id\":\"u256\")"
                }
            },
        }
    }

    // TODO: make this a const fn
    pub fn type_hash(&self, revision: Revision) -> Felt {
        match self {
            Self::U256 => starknet_keccak(self.type_signature(revision).as_bytes()),
            Self::TokenAmount | Self::NftId => starknet_keccak(
                format!(
                    "{}{}",
                    self.type_signature(revision),
                    Self::U256.type_signature(revision)
                )
                .as_bytes(),
            ),
        }
    }
}

impl<'de> Deserialize<'de> for TypeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let elements = Vec::<FieldOrVariantDefinition>::deserialize(deserializer)?;

        match elements.first() {
            Some(FieldOrVariantDefinition::Field(_)) => {
                // This is a struct definition
                let mut fields = Vec::new();
                for element in elements {
                    match element {
                        FieldOrVariantDefinition::Field(field) => fields.push(field),
                        FieldOrVariantDefinition::Variant(_) => {
                            return Err(serde::de::Error::invalid_type(
                                Unexpected::Other("enum variant definition"),
                                &"struct field definition",
                            ))
                        }
                    }
                }
                Ok(Self::Struct(StructDefinition { fields }))
            }
            Some(FieldOrVariantDefinition::Variant(_)) => {
                // This is an enum definition
                let mut variants = Vec::new();
                for element in elements {
                    match element {
                        FieldOrVariantDefinition::Variant(variant) => variants.push(variant),
                        FieldOrVariantDefinition::Field(_) => {
                            return Err(serde::de::Error::invalid_type(
                                Unexpected::Other("struct field definition"),
                                &"enum variant definition",
                            ))
                        }
                    }
                }
                Ok(Self::Enum(EnumDefinition { variants }))
            }
            None => Err(serde::de::Error::invalid_length(
                0,
                &"at least 1 field or variant",
            )),
        }
    }
}

impl Serialize for TypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Type {
            name: String,
            r#type: String,
        }
        let types: Vec<Type> = match self {
            Self::Struct(struct_def) => struct_def
                .fields
                .iter()
                .map(|field| Type {
                    name: field.name.clone(),
                    r#type: field.r#type.signature_ref_repr(),
                })
                .collect(),
            Self::Enum(enum_def) => enum_def
                .variants
                .iter()
                .map(|variant| Type {
                    name: variant.name.clone(),
                    r#type: format!(
                        "({})",
                        variant
                            .tuple_types
                            .iter()
                            .map(|t| t.signature_ref_repr())
                            .collect::<Vec<String>>()
                            .join(",")
                    ),
                })
                .collect(),
        };
        types.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FieldOrVariantDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            name: String,
            r#type: String,
            contains: Option<String>,
        }

        let raw = Raw::deserialize(deserializer)?;
        if raw.name.is_empty() {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Str(""),
                &"non-empty name",
            ));
        }

        if raw.r#type.starts_with('(') {
            // Enum variant definition

            if !raw.r#type.ends_with(')') {
                return Err(serde::de::Error::invalid_value(
                    Unexpected::Str(&raw.r#type),
                    &"enclosing parentheses",
                ));
            }
            if raw.contains.is_some() {
                // Enum variants have no `contains` field
                return Err(serde::de::Error::unknown_field(
                    "contains",
                    &["name", "type"],
                ));
            }

            let joined_tuple_types = &raw.r#type[1..(raw.r#type.len() - 1)];
            if joined_tuple_types.is_empty() {
                Ok(Self::Variant(VariantDefinition {
                    name: raw.name,
                    tuple_types: Vec::new(),
                }))
            } else {
                let tuple_types = joined_tuple_types
                    .split(',')
                    .map(|raw_type| {
                        // Trimming here feels weird but the example from SNIP-12 has a space after
                        // `,` so it seems that whitespaces are allowed.
                        InlineTypeReference::from_str(raw_type.trim()).map_err(|err| {
                            serde::de::Error::custom(format!(
                                "invalid inline type reference: {}",
                                err
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self::Variant(VariantDefinition {
                    name: raw.name,
                    tuple_types,
                }))
            }
        } else {
            // Struct field definition
            Ok(Self::Field(FieldDefinition {
                name: raw.name,
                r#type: FullTypeReference::from_parts(raw.r#type, raw.contains).map_err(|err| {
                    serde::de::Error::custom(format!("invalid full type reference: {}", err))
                })?,
            }))
        }
    }
}

impl CompositeType for StructDefinition {
    fn field_iter(&self) -> impl Iterator<Item = (&str, &FullTypeReference)> {
        self.fields
            .iter()
            .map(|field| (field.name.as_str(), &field.r#type))
    }

    fn field_len(&self) -> usize {
        self.fields.len()
    }
}

impl CompositeType for PresetType {
    fn field_iter(&self) -> impl Iterator<Item = (&str, &FullTypeReference)> {
        match self {
            Self::U256 => [
                ("low", &FullTypeReference::U128),
                ("high", &FullTypeReference::U128),
            ]
            .into_iter(),
            Self::TokenAmount => [
                ("token_address", &FullTypeReference::ContractAddress),
                ("amount", &FullTypeReference::U256),
            ]
            .into_iter(),
            Self::NftId => [
                ("collection_address", &FullTypeReference::ContractAddress),
                ("token_id", &FullTypeReference::U256),
            ]
            .into_iter(),
        }
    }

    fn field_len(&self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::super::type_reference::ElementTypeReference;
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_struct_def_deser() {
        let raw = r###"[
  { "name": "Name", "type": "string" },
  { "name": "Some Array", "type": "u128*" },
  { "name": "Some Object", "type": "My Object" },
  { "name": "Some Enum", "type": "enum", "contains": "My Enum" }
]"###;

        let def = serde_json::from_str::<TypeDefinition>(raw).unwrap();
        match def {
            TypeDefinition::Struct(struct_def) => {
                assert_eq!(struct_def.fields.len(), 4);
                assert_eq!(struct_def.fields[0].r#type, FullTypeReference::String);
                assert_eq!(
                    struct_def.fields[1].r#type,
                    FullTypeReference::Array(ElementTypeReference::U128)
                );
                assert_eq!(
                    struct_def.fields[2].r#type,
                    FullTypeReference::Object("My Object".into())
                );
                assert_eq!(
                    struct_def.fields[3].r#type,
                    FullTypeReference::Enum("My Enum".into())
                );
            }
            _ => panic!("unexpected definition type"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_enum_def_deser() {
        let raw = r###"[
  { "name": "Variant 1", "type": "()" },
  { "name": "Variant 2", "type": "(u128, u128*)" },
  { "name": "Variant N", "type": "(u128)" }
]"###;

        let def = serde_json::from_str::<TypeDefinition>(raw).unwrap();
        match def {
            TypeDefinition::Enum(enum_def) => {
                assert_eq!(enum_def.variants.len(), 3);
                assert_eq!(enum_def.variants[0].tuple_types, vec![]);
                assert_eq!(
                    enum_def.variants[1].tuple_types,
                    vec![
                        InlineTypeReference::U128,
                        InlineTypeReference::Array(ElementTypeReference::U128)
                    ]
                );
                assert_eq!(
                    enum_def.variants[2].tuple_types,
                    vec![InlineTypeReference::U128]
                );
            }
            _ => panic!("unexpected definition type"),
        }
    }
}
