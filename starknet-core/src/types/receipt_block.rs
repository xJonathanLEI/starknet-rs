use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;
use starknet_types_core::felt::Felt;

/// A more idiomatic way to access `execution_status` and `revert_reason`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptBlock {
    Pending,
    Block { block_hash: Felt, block_number: u64 },
}

impl ReceiptBlock {
    /// Returns `true` if and only if it's the `Pending` variant.
    pub const fn is_pending(&self) -> bool {
        match self {
            Self::Pending => true,
            Self::Block { .. } => false,
        }
    }

    /// Returns `true` if and only if it's the `Block` variant.
    pub const fn is_block(&self) -> bool {
        match self {
            Self::Pending => false,
            Self::Block { .. } => true,
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block hash is to match the `Block` enum variant.
    pub const fn block_hash(&self) -> Option<Felt> {
        match self {
            Self::Pending => None,
            Self::Block { block_hash, .. } => Some(*block_hash),
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block number is to match the `Block` enum variant.
    pub const fn block_number(&self) -> Option<u64> {
        match self {
            Self::Pending => None,
            Self::Block { block_number, .. } => Some(*block_number),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
struct Raw {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    block_hash: Option<Felt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_number: Option<u64>,
}

impl Serialize for ReceiptBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let raw = match self {
            Self::Pending => Raw {
                block_hash: None,
                block_number: None,
            },
            Self::Block {
                block_hash,
                block_number,
            } => Raw {
                block_hash: Some(*block_hash),
                block_number: Some(*block_number),
            },
        };

        Raw::serialize(&raw, serializer)
    }
}

impl<'de> Deserialize<'de> for ReceiptBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = Raw::deserialize(deserializer)?;

        match (raw.block_hash, raw.block_number) {
            (Some(block_hash), Some(block_number)) => Ok(Self::Block {
                block_hash,
                block_number,
            }),
            (None, None) => Ok(Self::Pending),
            (Some(_), None) => Err(serde::de::Error::custom(
                "field `block_hash` must not exist when `block_number` is missing",
            )),
            (None, Some(_)) => Err(serde::de::Error::custom(
                "field `block_number` must not exist when `block_hash` is missing",
            )),
        }
    }
}
