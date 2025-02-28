use alloc::string::*;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::{Felt, SequencerTransactionStatus, UfeHex};

/// Handler transaction hash and status of an L1->L2 message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageWithStatus {
    /// Transaction hash of the `L1_HANDLER` transaction hash on Starknet.
    pub transaction_hash: Felt,
    /// Message finality status.
    pub status: MessageStatus,
}

/// Finality status of an L1->L2 message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStatus {
    /// The `L1_HANDLER` transaction has `RECEIVED` status.
    Received,
    /// The `L1_HANDLER` transaction has `REJECTED` status.
    Rejected {
        /// The reason that the message was rejected.
        reason: String,
    },
    /// The `L1_HANDLER` transaction has `ACCEPTED_ON_L2` status.
    AcceptedOnL2,
    /// The `L1_HANDLER` transaction has `ACCEPTED_ON_L1` status.
    AcceptedOnL1,
}

impl Serialize for MessageWithStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Raw<'a> {
            #[serde_as(as = "UfeHex")]
            transaction_hash: &'a Felt,
            finality_status: SequencerTransactionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<&'a str>,
        }

        let raw = match &self.status {
            MessageStatus::Received => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: SequencerTransactionStatus::Received,
                failure_reason: None,
            },
            MessageStatus::Rejected { reason } => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: SequencerTransactionStatus::Rejected,
                failure_reason: Some(reason),
            },
            MessageStatus::AcceptedOnL2 => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: SequencerTransactionStatus::AcceptedOnL2,
                failure_reason: None,
            },
            MessageStatus::AcceptedOnL1 => Raw {
                transaction_hash: &self.transaction_hash,
                finality_status: SequencerTransactionStatus::AcceptedOnL1,
                failure_reason: None,
            },
        };

        Raw::serialize(&raw, serializer)
    }
}

impl<'de> Deserialize<'de> for MessageWithStatus {
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
            finality_status: SequencerTransactionStatus,
            failure_reason: Option<String>,
        }

        let raw = Raw::deserialize(deserializer)?;

        let status =
            match (raw.finality_status, raw.failure_reason) {
                (SequencerTransactionStatus::Received, None) => MessageStatus::Received,
                (SequencerTransactionStatus::Rejected, Some(reason)) => {
                    MessageStatus::Rejected { reason }
                }
                (SequencerTransactionStatus::AcceptedOnL2, None) => MessageStatus::AcceptedOnL2,
                (SequencerTransactionStatus::AcceptedOnL1, None) => MessageStatus::AcceptedOnL1,
                (
                    SequencerTransactionStatus::Received
                    | SequencerTransactionStatus::AcceptedOnL2
                    | SequencerTransactionStatus::AcceptedOnL1,
                    Some(_),
                ) => return Err(serde::de::Error::custom(
                    "field `failure_reason` must not exist unless `finality_status` is `REJECTED`",
                )),
                (SequencerTransactionStatus::Rejected, None) => {
                    return Err(serde::de::Error::custom(
                        "field `failure_reason` must exist when `finality_status` is `REJECTED`",
                    ))
                }
            };

        Ok(Self {
            transaction_hash: raw.transaction_hash,
            status,
        })
    }
}
