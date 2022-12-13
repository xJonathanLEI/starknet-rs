use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{DeserializeAs, SerializeAs};

use super::{SyncStatus, SyncStatusType};

pub(crate) struct NumAsHex;

impl SerializeAs<u64> for NumAsHex {
    fn serialize_as<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:#x}", value))
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
                "invalid hex string: {}",
                err
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
