use core::{hash::BuildHasher, marker::PhantomData};

use alloc::{fmt::Formatter, format};

use indexmap::IndexMap;
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DeserializeAs, SerializeAs};

use super::{
    codegen::OwnedPtr, ContractExecutionError, Felt, MerkleNode, SyncStatus, SyncStatusType, UfeHex,
};

pub(crate) struct NumAsHex;

struct NumAsHexVisitorU64;
struct NumAsHexVisitorU128;

impl SerializeAs<u64> for NumAsHex {
    fn serialize_as<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }
}

impl SerializeAs<&u64> for NumAsHex {
    fn serialize_as<S>(value: &&u64, serializer: S) -> Result<S::Ok, S::Error>
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
        deserializer.deserialize_any(NumAsHexVisitorU64)
    }
}

impl SerializeAs<u128> for NumAsHex {
    fn serialize_as<S>(value: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }
}

impl<'de> DeserializeAs<'de, u128> for NumAsHex {
    fn deserialize_as<D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumAsHexVisitorU128)
    }
}

impl Visitor<'_> for NumAsHexVisitorU64 {
    type Value = u64;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> alloc::fmt::Result {
        write!(formatter, "string or number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match u64::from_str_radix(v.trim_start_matches("0x"), 16) {
            Ok(value) => Ok(value),
            Err(err) => Err(serde::de::Error::custom(format!(
                "invalid hex string: {err}"
            ))),
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.try_into() {
            Ok(value) => self.visit_u64(value),
            Err(_) => Err(serde::de::Error::custom(format!(
                "value cannot be negative: {}",
                v
            ))),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl Visitor<'_> for NumAsHexVisitorU128 {
    type Value = u128;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> alloc::fmt::Result {
        write!(formatter, "string or number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match u128::from_str_radix(v.trim_start_matches("0x"), 16) {
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
            Self::NotSyncing => serializer.serialize_bool(false),
            Self::Syncing(sync_status) => SyncStatus::serialize(sync_status, serializer),
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
            SyncStatusTypeDe::SyncStatus(value) => Ok(Self::Syncing(value)),
        }
    }
}

pub(crate) struct OwnedContractExecutionError;

impl SerializeAs<OwnedPtr<ContractExecutionError>> for OwnedContractExecutionError {
    fn serialize_as<S>(
        value: &OwnedPtr<ContractExecutionError>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ContractExecutionError::serialize(value.as_ref(), serializer)
    }
}

impl<'de> DeserializeAs<'de, OwnedPtr<ContractExecutionError>> for OwnedContractExecutionError {
    fn deserialize_as<D>(deserializer: D) -> Result<OwnedPtr<ContractExecutionError>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OwnedPtr::new(ContractExecutionError::deserialize(
            deserializer,
        )?))
    }
}

pub(crate) struct MerkleNodeMap;

struct MerkleNodeMapVisitor<R> {
    phantom: PhantomData<R>,
}

impl<R> SerializeAs<IndexMap<Felt, MerkleNode, R>> for MerkleNodeMap {
    fn serialize_as<S>(
        value: &IndexMap<Felt, MerkleNode, R>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Raw<'a> {
            node_hash: &'a Felt,
            node: &'a MerkleNode,
        }

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for (key, node) in value {
            seq.serialize_element(&Raw {
                node_hash: key,
                node,
            })?;
        }
        seq.end()
    }
}

impl<'de, R> DeserializeAs<'de, IndexMap<Felt, MerkleNode, R>> for MerkleNodeMap
where
    R: BuildHasher + Default,
{
    fn deserialize_as<D>(deserializer: D) -> Result<IndexMap<Felt, MerkleNode, R>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MerkleNodeMapVisitor::<R> {
            phantom: PhantomData,
        })
    }
}

impl<'de, R> Visitor<'de> for MerkleNodeMapVisitor<R>
where
    R: BuildHasher + Default,
{
    type Value = IndexMap<Felt, MerkleNode, R>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        struct Raw {
            #[serde_as(as = "UfeHex")]
            node_hash: Felt,
            node: MerkleNode,
        }

        let mut map = match seq.size_hint() {
            Some(hint) => {
                IndexMap::<Felt, MerkleNode, R>::with_capacity_and_hasher(hint, R::default())
            }
            None => IndexMap::with_hasher(R::default()),
        };

        while let Some(element) = seq.next_element::<Raw>()? {
            map.insert(element.node_hash, element.node);
        }

        Ok(map)
    }
}

mod block_id {
    use crate::serde::unsigned_field_element::UfeHex;
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_with::serde_as;
    use starknet_types_core::felt::Felt;

    use crate::types::{BlockId, BlockTag, ConfirmedBlockId};

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
        block_hash: Felt,
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

    impl Serialize for ConfirmedBlockId {
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
                Self::Latest => BlockTag::serialize(&BlockTag::Latest, serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for ConfirmedBlockId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match BlockIdDe::deserialize(deserializer)? {
                BlockIdDe::Hash(hash) => Self::Hash(hash.block_hash),
                BlockIdDe::Number(number) => Self::Number(number.block_number),
                BlockIdDe::Tag(BlockTag::Latest) => Self::Latest,
                BlockIdDe::Tag(BlockTag::Pending) => {
                    return Err(serde::de::Error::custom(
                        "confirmed block id must not be `pending`",
                    ))
                }
            })
        }
    }
}

mod transaction_status {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::types::{
        ExecutionResult, SequencerTransactionStatus, TransactionExecutionStatus, TransactionStatus,
    };

    #[derive(Deserialize)]
    #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
    struct Raw {
        finality_status: SequencerTransactionStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        execution_status: Option<TransactionExecutionStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_reason: Option<String>,
    }

    #[derive(Serialize)]
    #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
    struct RawRef<'a> {
        finality_status: SequencerTransactionStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        execution_status: Option<TransactionExecutionStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_reason: Option<&'a str>,
    }

    impl Serialize for TransactionStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let raw = match self {
                Self::Received => RawRef {
                    finality_status: SequencerTransactionStatus::Received,
                    execution_status: None,
                    failure_reason: None,
                },
                Self::Rejected => RawRef {
                    finality_status: SequencerTransactionStatus::Rejected,
                    execution_status: None,
                    failure_reason: None,
                },
                Self::AcceptedOnL2(exe) => RawRef {
                    finality_status: SequencerTransactionStatus::AcceptedOnL2,
                    execution_status: Some(exe.status()),
                    failure_reason: match &exe {
                        ExecutionResult::Succeeded => None,
                        ExecutionResult::Reverted { reason } => Some(reason),
                    },
                },
                Self::AcceptedOnL1(exe) => RawRef {
                    finality_status: SequencerTransactionStatus::AcceptedOnL1,
                    execution_status: Some(exe.status()),
                    failure_reason: match &exe {
                        ExecutionResult::Succeeded => None,
                        ExecutionResult::Reverted { reason } => Some(reason),
                    },
                },
            };

            raw.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for TransactionStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let raw = Raw::deserialize(deserializer)?;

            match (
                raw.finality_status,
                raw.execution_status,
                raw.failure_reason,
            ) {
                (SequencerTransactionStatus::Received, None, None) => Ok(Self::Received),
                (SequencerTransactionStatus::Rejected, None, None) => Ok(Self::Rejected),
                (SequencerTransactionStatus::AcceptedOnL2, Some(exe), reason) => {
                    Ok(Self::AcceptedOnL2(parse_exe::<D>(exe, reason)?))
                }
                (SequencerTransactionStatus::AcceptedOnL1, Some(exe), reason) => {
                    Ok(Self::AcceptedOnL1(parse_exe::<D>(exe, reason)?))
                }
                _ => Err(serde::de::Error::custom(
                    "invalid combination of finality_status, execution_status, and failure_reason",
                )),
            }
        }
    }

    fn parse_exe<'de, D>(
        exe: TransactionExecutionStatus,
        reason: Option<String>,
    ) -> Result<ExecutionResult, D::Error>
    where
        D: Deserializer<'de>,
    {
        match (exe, reason) {
            (TransactionExecutionStatus::Succeeded, None) => Ok(ExecutionResult::Succeeded),
            (TransactionExecutionStatus::Reverted, Some(reason)) => {
                Ok(ExecutionResult::Reverted { reason })
            }
            _ => Err(serde::de::Error::custom(
                "inconsistent execution_status and failure_reason",
            )),
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

    impl Serialize for TransactionContent {
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
                Self::DeployAccount(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for InvokeTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for InvokeTransactionContent {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for DeclareTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
                Self::V2(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for DeclareTransactionContent {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V0(variant) => variant.serialize(serializer),
                Self::V1(variant) => variant.serialize(serializer),
                Self::V2(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for DeployAccountTransaction {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V1(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for DeployAccountTransactionContent {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::V1(variant) => variant.serialize(serializer),
                Self::V3(variant) => variant.serialize(serializer),
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

    impl Serialize for TransactionTrace {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Invoke(variant) => variant.serialize(serializer),
                Self::DeployAccount(variant) => variant.serialize(serializer),
                Self::L1Handler(variant) => variant.serialize(serializer),
                Self::Declare(variant) => variant.serialize(serializer),
            }
        }
    }

    impl Serialize for ExecuteInvocation {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Self::Success(variant) => variant.serialize(serializer),
                Self::Reverted(variant) => variant.serialize(serializer),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_with::serde_as;
    use starknet_types_core::felt::Felt;

    use super::{
        super::{BlockId, BlockTag, ConfirmedBlockId},
        NumAsHex,
    };

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_blockid_serde() {
        for (block_id, json) in [
            (
                BlockId::Hash(Felt::from_hex("0x1234").unwrap()),
                "{\"block_hash\":\"0x1234\"}",
            ),
            (BlockId::Number(1234), "{\"block_number\":1234}"),
            (BlockId::Tag(BlockTag::Latest), "\"latest\""),
            (BlockId::Tag(BlockTag::Pending), "\"pending\""),
        ] {
            assert_eq!(serde_json::to_string(&block_id).unwrap(), json);
            assert_eq!(serde_json::from_str::<BlockId>(json).unwrap(), block_id);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_confirmed_blockid_serde() {
        for (block_id, json) in [
            (
                ConfirmedBlockId::Hash(Felt::from_hex("0x1234").unwrap()),
                "{\"block_hash\":\"0x1234\"}",
            ),
            (ConfirmedBlockId::Number(1234), "{\"block_number\":1234}"),
            (ConfirmedBlockId::Latest, "\"latest\""),
        ] {
            assert_eq!(serde_json::to_string(&block_id).unwrap(), json);
            assert_eq!(
                serde_json::from_str::<ConfirmedBlockId>(json).unwrap(),
                block_id
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_num_as_hex_deser() {
        #[serde_as]
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct Value(#[serde_as(as = "NumAsHex")] u64);

        for (num, json) in [(Value(100), "\"0x64\""), (Value(100), "100")] {
            assert_eq!(serde_json::from_str::<Value>(json).unwrap(), num);
        }
    }
}
