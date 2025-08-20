use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;
use starknet_types_core::felt::Felt;

/// Block identifier used in
/// [`TransactionReceiptWithBlockInfo`](super::TransactionReceiptWithBlockInfo).
///
/// Instead of directly exposing the `block_hash` and `block_number` fields as [`Option<Felt>`],
/// this struct captures the fact that these fields are always [`Some`](Option::Some) or
/// [`None`](Option::None) together, allowing idiomatic access without unnecessary unwraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptBlock {
    /// The receipt is attached to a pre-confirmed block.
    PreConfirmed {
        /// Block number (height).
        block_number: u64,
    },
    /// The receipt is attached to a confirmed block.
    Block {
        /// Block hash.
        block_hash: Felt,
        /// Block number (height).
        block_number: u64,
    },
}

impl ReceiptBlock {
    /// Returns `true` if and only if it's the `PreConfirmed` variant.
    pub const fn is_pre_confirmed(&self) -> bool {
        match self {
            Self::PreConfirmed { .. } => true,
            Self::Block { .. } => false,
        }
    }

    /// Returns `true` if and only if it's the `Block` variant.
    pub const fn is_block(&self) -> bool {
        match self {
            Self::PreConfirmed { .. } => false,
            Self::Block { .. } => true,
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block hash is to match the `Block` enum variant.
    pub const fn block_hash(&self) -> Option<Felt> {
        match self {
            Self::PreConfirmed { .. } => None,
            Self::Block { block_hash, .. } => Some(*block_hash),
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block number is to match the `Block` enum variant.
    pub const fn block_number(&self) -> Option<u64> {
        match self {
            Self::PreConfirmed { block_number } => Some(*block_number),
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
            Self::PreConfirmed { block_number } => Raw {
                block_hash: None,
                block_number: Some(*block_number),
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
            (None, Some(block_number)) => Ok(Self::PreConfirmed { block_number }),
            (Some(_), None) => Err(serde::de::Error::custom(
                "field `block_number` must exist if `block_hash` exists",
            )),
            (None, None) => Err(serde::de::Error::custom(
                "at least `block_number` must exist",
            )),
        }
    }
}
