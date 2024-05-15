use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use starknet_types_core::felt::Felt;

/// A more idiomatic way to access `execution_status` and `revert_reason`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptBlock {
    Pending,
    Block { block_hash: Felt, block_number: u64 },
}

impl ReceiptBlock {
    /// Returns `true` if and only if it's the `Pending` variant.
    pub fn is_pending(&self) -> bool {
        match self {
            ReceiptBlock::Pending => true,
            ReceiptBlock::Block { .. } => false,
        }
    }

    /// Returns `true` if and only if it's the `Block` variant.
    pub fn is_block(&self) -> bool {
        match self {
            ReceiptBlock::Pending => false,
            ReceiptBlock::Block { .. } => true,
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block hash is to match the `Block` enum variant.
    pub fn block_hash(&self) -> Option<Felt> {
        match self {
            ReceiptBlock::Pending => None,
            ReceiptBlock::Block { block_hash, .. } => Some(*block_hash),
        }
    }

    /// Returns `None` if block is not `Block`.
    ///
    /// A more idiomatic way of accessing the block number is to match the `Block` enum variant.
    pub fn block_number(&self) -> Option<u64> {
        match self {
            ReceiptBlock::Pending => None,
            ReceiptBlock::Block { block_number, .. } => Some(*block_number),
        }
    }
}

impl Serialize for ReceiptBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde_as]
        struct Raw<'a> {
            #[serde_as(as = "Option<UfeHex>")]
            #[serde(skip_serializing_if = "Option::is_none")]
            block_hash: Option<&'a Felt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_number: Option<&'a u64>,
        }

        let raw = match self {
            Self::Pending => Raw {
                block_hash: None,
                block_number: None,
            },
            Self::Block {
                block_hash,
                block_number,
            } => Raw {
                block_hash: Some(block_hash),
                block_number: Some(block_number),
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
        #[derive(Deserialize)]
        #[serde_as]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Raw {
            #[serde_as(as = "Option<UfeHex>")]
            #[serde(skip_serializing_if = "Option::is_none")]
            block_hash: Option<Felt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_number: Option<u64>,
        }

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
