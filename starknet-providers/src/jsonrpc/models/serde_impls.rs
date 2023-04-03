use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DeserializeAs, SerializeAs};

use super::{SyncStatus, SyncStatusType};

pub(crate) struct NumAsHex;

impl SerializeAs<u64> for NumAsHex {
    fn serialize_as<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }
}

impl<'de> DeserializeAs<'de, u64> for NumAsHex {
    fn deserialize_as<D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match u64::from_str_radix(&value[2..], 16) {
            Ok(value) => Ok(value),
            Err(err) => Err(serde::de::Error::custom(format!(
                "invalid hex string: {err}"
            ))),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SyncStatusTypeDe {
    Boolean(bool),
    SyncStatus(SyncStatus),
}

impl Serialize for SyncStatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SyncStatusType::NotSyncing => serializer.serialize_bool(false),
            SyncStatusType::Syncing(sync_status) => SyncStatus::serialize(sync_status, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SyncStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SyncStatusTypeDe::deserialize(deserializer)? {
            SyncStatusTypeDe::Boolean(value) => match value {
                true => Err(serde::de::Error::custom("invalid boolean value")),

                false => Ok(Self::NotSyncing),
            },
            SyncStatusTypeDe::SyncStatus(value) => Ok(SyncStatusType::Syncing(value)),
        }
    }
}

mod block_id {
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_with::serde_as;
    use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};

    use crate::jsonrpc::models::{BlockId, BlockTag};

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BlockIdDe {
        Hash(BlockHash),
        Number(BlockNumber),
        Tag(BlockTag),
    }

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
    struct BlockHash {
        #[serde_as(as = "UfeHex")]
        block_hash: FieldElement,
    }

    #[derive(Serialize, Deserialize)]
    #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
    struct BlockNumber {
        block_number: u64,
    }

    impl Serialize for BlockId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Self::Hash(hash) => {
                    BlockHash::serialize(&BlockHash { block_hash: *hash }, serializer)
                }
                Self::Number(number) => BlockNumber::serialize(
                    &BlockNumber {
                        block_number: *number,
                    },
                    serializer,
                ),
                Self::Tag(tag) => BlockTag::serialize(tag, serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for BlockId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match BlockIdDe::deserialize(deserializer)? {
                BlockIdDe::Hash(hash) => Self::Hash(hash.block_hash),
                BlockIdDe::Number(number) => Self::Number(number.block_number),
                BlockIdDe::Tag(tag) => Self::Tag(tag),
            })
        }
    }
}

// Deriving the Serialize trait directly results in duplicate fields since the variants also write
// the tag fields when individually serialized.
mod enum_ser_impls {
    use super::super::*;

    impl Serialize for Transaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Invoke(variant) => variant.serialize(serializer),
                Self::L1Handler(variant) => variant.serialize(serializer),
                Self::Declare(variant) => variant.serialize(serializer),
                Self::Deploy(variant) => variant.serialize(serializer),
                Self::DeployAccount(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for BroadcastedTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Invoke(variant) => variant.serialize(serializer),
                Self::Declare(variant) => variant.serialize(serializer),
                Self::Deploy(variant) => variant.serialize(serializer),
                Self::DeployAccount(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for InvokeTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for BroadcastedInvokeTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for DeclareTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V1(variant) => variant.serialize(serializer),
                Self::V2(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for BroadcastedDeclareTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V1(variant) => variant.serialize(serializer),
                Self::V2(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for TransactionReceipt {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Invoke(variant) => variant.serialize(serializer),
                Self::L1Handler(variant) => variant.serialize(serializer),
                Self::Declare(variant) => variant.serialize(serializer),
                Self::Deploy(variant) => variant.serialize(serializer),
                Self::DeployAccount(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for PendingTransactionReceipt {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Invoke(variant) => variant.serialize(serializer),
                Self::L1Handler(variant) => variant.serialize(serializer),
                Self::Declare(variant) => variant.serialize(serializer),
                Self::Deploy(variant) => variant.serialize(serializer),
                Self::DeployAccount(variant) => variant.serialize(serializer),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use starknet_core::types::FieldElement;

    use super::super::{BlockId, BlockTag};

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_blockid_serde() {
        for (block_id, json) in [
            (
                BlockId::Hash(FieldElement::from_hex_be("0x1234").unwrap()),
                "{\"block_hash\":\"0x1234\"}",
            ),
            (BlockId::Number(1234), "{\"block_number\":1234}"),
            (BlockId::Tag(BlockTag::Latest), "\"latest\""),
            (BlockId::Tag(BlockTag::Pending), "\"pending\""),
        ]
        .into_iter()
        {
            assert_eq!(serde_json::to_string(&block_id).unwrap(), json);
            assert_eq!(serde_json::from_str::<BlockId>(json).unwrap(), block_id);
        }
    }
}
