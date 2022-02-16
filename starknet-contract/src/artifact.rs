use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::UfeHex,
    types::{AbiEntry, EntryPointsByType, UnsignedFieldElement},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub abi: Vec<AbiEntry>,
    pub entry_points_by_type: EntryPointsByType,
    pub program: Program,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    #[serde(skip_serializing)]
    pub attributes: serde::de::IgnoredAny, // Skipped since it's not used in deployment
    pub builtins: Vec<String>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<UnsignedFieldElement>,
    #[serde(skip_serializing)]
    pub debug_info: serde::de::IgnoredAny, // Skipped since it's not used in deployment
    pub hints: BTreeMap<u64, Vec<Hint>>,
    pub identifiers: BTreeMap<String, Identifier>,
    pub main_scope: String,
    // Impossible to use [UnsignedFieldElement] here as by definition field elements are smaller
    // than prime
    pub prime: String,
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
