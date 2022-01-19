use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use starknet_core::{
    serde::{deserialize_h256_from_hex, serialize_u8_slice_into_hex_without_leading_zeros},
    types::{AbiEntry, EntryPointsByType, H256, U256},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub abi: Vec<AbiEntry>,
    pub entry_points_by_type: EntryPointsByType,
    pub program: Program,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    #[serde(default, skip_serializing)]
    pub attributes: serde::de::IgnoredAny, // Skipped since it's not used in deployment
    pub builtins: Vec<String>,
    pub data: Vec<U256>,
    #[serde(default, skip_serializing)]
    pub debug_info: serde::de::IgnoredAny, // Skipped since it's not used in deployment
    pub hints: BTreeMap<u64, Vec<Hint>>,
    pub identifiers: BTreeMap<String, Identifier>,
    pub main_scope: String,
    #[serde(
        serialize_with = "serialize_u8_slice_into_hex_without_leading_zeros",
        deserialize_with = "deserialize_h256_from_hex"
    )]
    pub prime: H256,
    pub reference_manager: ReferenceManager,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hint {
    pub accessible_scopes: Vec<String>,
    pub code: String,
    pub flow_tracking_data: FlowTrackingData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorators: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cairo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<BTreeMap<String, IdentifierMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Number>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceManager {
    pub references: Vec<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentifierMember {
    pub cairo_type: String,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub ap_tracking_data: ApTrackingData,
    pub pc: u64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowTrackingData {
    pub ap_tracking: ApTrackingData,
    pub reference_ids: BTreeMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApTrackingData {
    pub group: u64,
    pub offset: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_deser_oz_account() {
        serde_json::from_str::<Artifact>(include_str!("../test-data/artifacts/oz_account.txt"))
            .unwrap();
    }

    #[test]
    fn test_artifact_deser_event_example() {
        serde_json::from_str::<Artifact>(include_str!("../test-data/artifacts/event_example.txt"))
            .unwrap();
    }
}
