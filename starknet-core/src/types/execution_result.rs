use alloc::string::*;

use serde::{Deserialize, Serialize};

use super::TransactionExecutionStatus;

/// Execution result of a transaction.
///
/// This struct ccorresponds to the `execution_status` and `revert_reason` fields of a transaction
/// receipt, capturing the fact that the presence of `revert_reason` depends on `execution_status`,
/// allowing more idiomatic access to `revert_reason`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionResult {
    /// The execution succeeded.
    Succeeded,
    /// The execution reverted.
    Reverted {
        /// The reason that the execution was reverted.    
        reason: String,
    },
}

impl ExecutionResult {
    /// Gets the [`TransactionExecutionStatus`].
    pub const fn status(&self) -> TransactionExecutionStatus {
        match self {
            Self::Succeeded => TransactionExecutionStatus::Succeeded,
            Self::Reverted { .. } => TransactionExecutionStatus::Reverted,
        }
    }

    /// Returns `None` if execution status is not `Reverted`.
    ///
    /// A more idiomatic way of accessing the revert reason is to match the `Reverted` enum
    /// variant.
    pub fn revert_reason(&self) -> Option<&str> {
        match self {
            Self::Succeeded => None,
            Self::Reverted { reason } => Some(reason),
        }
    }
}

impl Serialize for ExecutionResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Raw<'a> {
            execution_status: &'a TransactionExecutionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            revert_reason: &'a Option<&'a str>,
        }

        let raw = Raw {
            execution_status: &self.status(),
            revert_reason: &self.revert_reason(),
        };

        Raw::serialize(&raw, serializer)
    }
}

impl<'de> Deserialize<'de> for ExecutionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Raw {
            execution_status: TransactionExecutionStatus,
            revert_reason: Option<String>,
        }

        let raw = Raw::deserialize(deserializer)?;

        match (raw.execution_status, raw.revert_reason) {
            (TransactionExecutionStatus::Succeeded, None) => Ok(Self::Succeeded),
            (TransactionExecutionStatus::Reverted, reason) => Ok(Self::Reverted {
                reason: reason.unwrap_or_default(),
            }),
            (TransactionExecutionStatus::Succeeded, Some(_)) => Err(serde::de::Error::custom(
                "field `revert_reason` must not exist when `execution_status` is `SUCCEEDED`",
            )),
        }
    }
}
