use alloc::string::*;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::{
    ExecutionResult, Felt, TransactionExecutionStatus, TransactionFinalityStatus, UfeHex,
};

/// Handler transaction hash and status of an L1->L2 message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageStatus {
    /// Transaction hash of the `L1_HANDLER` transaction hash on Starknet.
    pub transaction_hash: Felt,
    /// Message finality status.
    pub finality_status: TransactionFinalityStatus,
    /// Message handler execution result.
    pub execution_result: ExecutionResult,
}

impl Serialize for MessageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Raw<'a> {
            #[serde_as(as = "UfeHex")]
            transaction_hash: &'a Felt,
            finality_status: TransactionFinalityStatus,
            execution_status: TransactionExecutionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<&'a str>,
        }

        let raw = match &self.execution_result {
            ExecutionResult::Succeeded => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: self.finality_status,
                execution_status: TransactionExecutionStatus::Succeeded,
                failure_reason: None,
            },
            ExecutionResult::Reverted { reason } => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: self.finality_status,
                execution_status: TransactionExecutionStatus::Reverted,
                failure_reason: Some(reason),
            },
        };

        Raw::serialize(&raw, serializer)
    }
}

impl<'de> Deserialize<'de> for MessageStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Raw {
            #[serde_as(as = "UfeHex")]
            transaction_hash: Felt,
            finality_status: TransactionFinalityStatus,
            execution_status: TransactionExecutionStatus,
            failure_reason: Option<String>,
        }

        let raw = Raw::deserialize(deserializer)?;

        let execution_result = match (raw.execution_status, raw.failure_reason) {
            (TransactionExecutionStatus::Succeeded, None) => ExecutionResult::Succeeded,
            (TransactionExecutionStatus::Reverted, Some(reason)) => {
                ExecutionResult::Reverted { reason }
            }
            (TransactionExecutionStatus::Succeeded, Some(_)) => {
                return Err(serde::de::Error::custom(
                    "field `failure_reason` must not exist unless `execution_status` is `REVERTED`",
                ))
            }
            (TransactionExecutionStatus::Reverted, None) => {
                return Err(serde::de::Error::custom(
                    "field `failure_reason` must exist when `execution_status` is `REVERTED`",
                ))
            }
        };

        Ok(Self {
            transaction_hash: raw.transaction_hash,
            finality_status: raw.finality_status,
            execution_result,
        })
    }
}
