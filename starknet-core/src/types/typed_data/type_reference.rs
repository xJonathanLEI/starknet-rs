use alloc::{borrow::ToOwned, format, string::*};
use core::str::FromStr;

use serde::{de::Visitor, Deserialize};

use super::error::TypedDataError;

/// A full type reference is used for defining custom struct fields and enum variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FullTypeReference {
    /// Reference to a struct type.
    Object(String),
    /// Reference to an enum type.
    Enum(String),
    /// Reference to an array type.
    Array(ElementTypeReference),
    /// Reference to the basic type `felt`.
    Felt,
    /// Reference to the basic type `bool`.
    Bool,
    /// Reference to the basic type `string`.
    String,
    /// Reference to the basic type `selector`.
    Selector,
    /// Reference to the basic type `merkletree`.
    MerkleTree(InlineTypeReference),
    /// Reference to the basic type `u128`.
    U128,
    /// Reference to the basic type `i128`.
    I128,
    /// Reference to the basic type `ContractAddress`.
    ContractAddress,
    /// Reference to the basic type `ClassHash`.
    ClassHash,
    /// Reference to the basic type `timestamp`.
    Timestamp,
    /// Reference to the preset type `u256`.
    U256,
    /// Reference to the preset type `TokenAmount`.
    TokenAmount,
    /// Reference to the preset type `NftId`.
    NftId,
    /// Reference to the basic type `shortstring`.
    ShortString,
}

/// A type reference that can be canonically represented as a single string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineTypeReference {
    /// Reference to a user-defined type. With an inline reference it's impossible to tell whether
    /// the pointee is a struct or enum.
    Custom(String),
    /// Reference to an array type.
    Array(ElementTypeReference),
    /// Reference to the basic type `felt`.
    Felt,
    /// Reference to the basic type `bool`.
    Bool,
    /// Reference to the basic type `string`.
    String,
    /// Reference to the basic type `selector`.
    Selector,
    /// Reference to the basic type `u128`.
    U128,
    /// Reference to the basic type `i128`.
    I128,
    /// Reference to the basic type `ContractAddress`.
    ContractAddress,
    /// Reference to the basic type `ClassHash`.
    ClassHash,
    /// Reference to the basic type `timestamp`.
    Timestamp,
    /// Reference to the preset type `u256`.
    U256,
    /// Reference to the preset type `TokenAmount`.
    TokenAmount,
    /// Reference to the preset type `NftId`.
    NftId,
    /// Reference to the basic type `shortstring`.
    ShortString,
}

/// Reference to any type that can be used as array elements.
///
/// This type is a strict subset of [`InlineTypeReference`].
///
/// SNIP-12 specifies that for an array:
///
/// > The inner type could be any of the other types supported in this specification.
///
/// Note the use of "other" here, implying that only one-dimensional arrays are supported. While
/// SNIP-12 does not have the most precise technical language, interpreting this way has the benefit
/// of avoiding unlimited nesting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementTypeReference {
    /// Reference to a user-defined type. With an inline reference it's impossible to tell whether
    /// the pointee is a struct or enum.
    Custom(String),
    /// Reference to the basic type `felt`.
    Felt,
    /// Reference to the basic type `bool`.
    Bool,
    /// Reference to the basic type `string`.
    String,
    /// Reference to the basic type `selector`.
    Selector,
    /// Reference to the basic type `u128`.
    U128,
    /// Reference to the basic type `i128`.
    I128,
    /// Reference to the basic type `ContractAddress`.
    ContractAddress,
    /// Reference to the basic type `ClassHash`.
    ClassHash,
    /// Reference to the basic type `timestamp`.
    Timestamp,
    /// Reference to the preset type `u256`.
    U256,
    /// Reference to the preset type `TokenAmount`.
    TokenAmount,
    /// Reference to the preset type `NftId`.
    NftId,
    /// Reference to the basic type `shortstring`.
    ShortString,
}

/// An internal trait for working across the different type reference types defined above.
pub(crate) trait TypeReference {
    /// Creates a common type reference representation useful for type transversal.
    fn common(&self) -> CommonTypeReference<'_>;

    /// Gets the "canonical" string representation to be used in type signature encoding as field
    /// type references.
    fn signature_ref_repr(&self) -> String;

    /// Whether the referenced type must be a struct.
    fn must_be_struct(&self) -> bool;

    /// Whether the referenced type must be an enum.
    fn must_be_enum(&self) -> bool;
}

/// An internal type reference type that can be created from all of:
///
/// - [`FullTypeReference`]
/// - [`InlineTypeReference`]
/// - [`ElementTypeReference`]
///
/// This type exists instead of just using [`FullTypeReference`] as when traversing user-defined
/// type definitions, only a type's name matters, not whether it's a struct or enum.
///
/// It's *technically* possible to still use [`FullTypeReference`] as the common repr anyway, by
/// always using the [`FullTypeReference::Object`] variant. However, that would be far from ideal.
pub(crate) enum CommonTypeReference<'a> {
    Custom(&'a str),
    Array(&'a ElementTypeReference),
    Felt,
    Bool,
    String,
    Selector,
    MerkleTree(&'a InlineTypeReference),
    U128,
    I128,
    ContractAddress,
    ClassHash,
    Timestamp,
    U256,
    TokenAmount,
    NftId,
    ShortString,
}

impl FullTypeReference {
    pub(crate) fn from_parts(
        r#type: String,
        contains: Option<String>,
    ) -> Result<Self, TypedDataError> {
        Ok(match (r#type.as_str(), contains) {
            ("felt", None) => Self::Felt,
            ("bool", None) => Self::Bool,
            ("string", None) => Self::String,
            ("selector", None) => Self::Selector,
            ("merkletree", Some(item)) => Self::MerkleTree(InlineTypeReference::from_str(&item)?),
            ("u128", None) => Self::U128,
            ("i128", None) => Self::I128,
            ("ContractAddress", None) => Self::ContractAddress,
            ("ClassHash", None) => Self::ClassHash,
            ("timestamp", None) => Self::Timestamp,
            ("u256", None) => Self::U256,
            ("TokenAmount", None) => Self::TokenAmount,
            ("NftId", None) => Self::NftId,
            ("shortstring", None) => Self::ShortString,
            ("enum", Some(enum_type)) => Self::Enum(enum_type),
            (item, None) if item.ends_with('*') => Self::Array(ElementTypeReference::from_str(
                &r#type[..(r#type.len() - 1)],
            )?),
            (type_name, None) if is_valid_type_name(type_name) => Self::Object(r#type),
            (_, Some(_)) => {
                return Err(TypedDataError::UnexpectedContainsField);
            }
            (type_name, _) => {
                return Err(TypedDataError::InvalidTypeName(type_name.to_owned()));
            }
        })
    }
}

impl TypeReference for FullTypeReference {
    fn common(&self) -> CommonTypeReference<'_> {
        match self {
            Self::Object(name) | Self::Enum(name) => CommonTypeReference::Custom(name),
            Self::Array(element) => CommonTypeReference::Array(element),
            Self::Felt => CommonTypeReference::Felt,
            Self::Bool => CommonTypeReference::Bool,
            Self::String => CommonTypeReference::String,
            Self::Selector => CommonTypeReference::Selector,
            Self::MerkleTree(leaf) => CommonTypeReference::MerkleTree(leaf),
            Self::U128 => CommonTypeReference::U128,
            Self::I128 => CommonTypeReference::I128,
            Self::ContractAddress => CommonTypeReference::ContractAddress,
            Self::ClassHash => CommonTypeReference::ClassHash,
            Self::Timestamp => CommonTypeReference::Timestamp,
            Self::U256 => CommonTypeReference::U256,
            Self::TokenAmount => CommonTypeReference::TokenAmount,
            Self::NftId => CommonTypeReference::NftId,
            Self::ShortString => CommonTypeReference::ShortString,
        }
    }

    fn signature_ref_repr(&self) -> String {
        match self {
            Self::Object(name) | Self::Enum(name) => name.to_owned(),
            Self::Array(element) => format!("{}*", element.signature_ref_repr()),
            Self::Felt => "felt".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::String => "string".to_owned(),
            Self::Selector => "selector".to_owned(),
            Self::MerkleTree(_) => "merkletree".to_owned(),
            Self::U128 => "u128".to_owned(),
            Self::I128 => "i128".to_owned(),
            Self::ContractAddress => "ContractAddress".to_owned(),
            Self::ClassHash => "ClassHash".to_owned(),
            Self::Timestamp => "timestamp".to_owned(),
            Self::U256 => "u256".to_owned(),
            Self::TokenAmount => "TokenAmount".to_owned(),
            Self::NftId => "NftId".to_owned(),
            Self::ShortString => "shortstring".to_owned(),
        }
    }

    fn must_be_struct(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    fn must_be_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
}

impl TypeReference for InlineTypeReference {
    fn common(&self) -> CommonTypeReference<'_> {
        match self {
            Self::Custom(name) => CommonTypeReference::Custom(name),
            Self::Array(element) => CommonTypeReference::Array(element),
            Self::Felt => CommonTypeReference::Felt,
            Self::Bool => CommonTypeReference::Bool,
            Self::String => CommonTypeReference::String,
            Self::Selector => CommonTypeReference::Selector,
            Self::U128 => CommonTypeReference::U128,
            Self::I128 => CommonTypeReference::I128,
            Self::ContractAddress => CommonTypeReference::ContractAddress,
            Self::ClassHash => CommonTypeReference::ClassHash,
            Self::Timestamp => CommonTypeReference::Timestamp,
            Self::U256 => CommonTypeReference::U256,
            Self::TokenAmount => CommonTypeReference::TokenAmount,
            Self::NftId => CommonTypeReference::NftId,
            Self::ShortString => CommonTypeReference::ShortString,
        }
    }

    fn signature_ref_repr(&self) -> String {
        match self {
            Self::Custom(name) => name.to_owned(),
            Self::Array(element) => format!("{}*", element.signature_ref_repr()),
            Self::Felt => "felt".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::String => "string".to_owned(),
            Self::Selector => "selector".to_owned(),
            Self::U128 => "u128".to_owned(),
            Self::I128 => "i128".to_owned(),
            Self::ContractAddress => "ContractAddress".to_owned(),
            Self::ClassHash => "ClassHash".to_owned(),
            Self::Timestamp => "timestamp".to_owned(),
            Self::U256 => "u256".to_owned(),
            Self::TokenAmount => "TokenAmount".to_owned(),
            Self::NftId => "NftId".to_owned(),
            Self::ShortString => "shortstring".to_owned(),
        }
    }

    fn must_be_struct(&self) -> bool {
        false
    }

    fn must_be_enum(&self) -> bool {
        false
    }
}

impl TypeReference for ElementTypeReference {
    fn common(&self) -> CommonTypeReference<'_> {
        match self {
            Self::Custom(name) => CommonTypeReference::Custom(name),
            Self::Felt => CommonTypeReference::Felt,
            Self::Bool => CommonTypeReference::Bool,
            Self::String => CommonTypeReference::String,
            Self::Selector => CommonTypeReference::Selector,
            Self::U128 => CommonTypeReference::U128,
            Self::I128 => CommonTypeReference::I128,
            Self::ContractAddress => CommonTypeReference::ContractAddress,
            Self::ClassHash => CommonTypeReference::ClassHash,
            Self::Timestamp => CommonTypeReference::Timestamp,
            Self::U256 => CommonTypeReference::U256,
            Self::TokenAmount => CommonTypeReference::TokenAmount,
            Self::NftId => CommonTypeReference::NftId,
            Self::ShortString => CommonTypeReference::ShortString,
        }
    }

    fn signature_ref_repr(&self) -> String {
        match self {
            Self::Custom(name) => name.to_owned(),
            Self::Felt => "felt".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::String => "string".to_owned(),
            Self::Selector => "selector".to_owned(),
            Self::U128 => "u128".to_owned(),
            Self::I128 => "i128".to_owned(),
            Self::ContractAddress => "ContractAddress".to_owned(),
            Self::ClassHash => "ClassHash".to_owned(),
            Self::Timestamp => "timestamp".to_owned(),
            Self::U256 => "u256".to_owned(),
            Self::TokenAmount => "TokenAmount".to_owned(),
            Self::NftId => "NftId".to_owned(),
            Self::ShortString => "shortstring".to_owned(),
        }
    }

    fn must_be_struct(&self) -> bool {
        false
    }

    fn must_be_enum(&self) -> bool {
        false
    }
}

impl FromStr for InlineTypeReference {
    type Err = TypedDataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Same as `FullTypeReference::from_parts` except cases involving `contains`.
        Ok(match s {
            "felt" => Self::Felt,
            "bool" => Self::Bool,
            "string" => Self::String,
            "selector" => Self::Selector,
            "u128" => Self::U128,
            "i128" => Self::I128,
            "ContractAddress" => Self::ContractAddress,
            "ClassHash" => Self::ClassHash,
            "timestamp" => Self::Timestamp,
            "u256" => Self::U256,
            "TokenAmount" => Self::TokenAmount,
            "NftId" => Self::NftId,
            "shortstring" => Self::ShortString,
            item if item.ends_with('*') => {
                Self::Array(ElementTypeReference::from_str(&s[..(s.len() - 1)])?)
            }
            type_name if is_valid_type_name(type_name) => Self::Custom(s.to_owned()),
            type_name => {
                return Err(TypedDataError::InvalidTypeName(type_name.to_owned()));
            }
        })
    }
}

impl FromStr for ElementTypeReference {
    type Err = TypedDataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Same as `InlineTypeReference::from_parts` except the array case.
        Ok(match s {
            "felt" => Self::Felt,
            "bool" => Self::Bool,
            "string" => Self::String,
            "selector" => Self::Selector,
            "u128" => Self::U128,
            "i128" => Self::I128,
            "ContractAddress" => Self::ContractAddress,
            "ClassHash" => Self::ClassHash,
            "timestamp" => Self::Timestamp,
            "u256" => Self::U256,
            "TokenAmount" => Self::TokenAmount,
            "NftId" => Self::NftId,
            "shortstring" => Self::ShortString,
            type_name if is_valid_type_name(type_name) => Self::Custom(s.to_owned()),
            type_name => {
                return Err(TypedDataError::InvalidTypeName(type_name.to_owned()));
            }
        })
    }
}

struct InlineTypeReferenceVisitor;

impl Visitor<'_> for InlineTypeReferenceVisitor {
    type Value = InlineTypeReference;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        InlineTypeReference::from_str(v).map_err(|err| {
            serde::de::Error::custom(format!("invalid inline type reference: {}", err))
        })
    }
}

impl<'de> Deserialize<'de> for InlineTypeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(InlineTypeReferenceVisitor)
    }
}

fn is_valid_type_name(type_name: &str) -> bool {
    !(type_name.is_empty()
        || type_name.contains(',')
        || type_name.contains('(')
        || type_name.contains(')'))
}
