use super::{
    super::serde::unsigned_field_element::UfeHex, AbiEntry, EntryPointsByType, FieldElement,
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractArtifact {
    pub abi: Vec<AbiEntry>,
    pub entry_points_by_type: EntryPointsByType,
    pub program: Program,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    #[serde(skip_serializing)]
    pub attributes: Option<serde::de::IgnoredAny>, // Skipped since it's not used in deployment
    pub builtins: Vec<String>,
    // This field was introduced in Cairo 0.10.0. By making it optional we're keeping compatibility
    // with older artifacts. This decision should be reviewd in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_version: Option<String>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
    #[serde(skip_serializing)]
    pub debug_info: Option<serde::de::IgnoredAny>, // Skipped since it's not used in deployment
    pub hints: BTreeMap<String, Vec<Hint>>,
    pub identifiers: BTreeMap<String, Identifier>,
    pub main_scope: String,
    // Impossible to use [FieldElement] here as by definition field elements are smaller
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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_artifact_deser_oz_account() {
        serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/contracts/artifacts/oz_account.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_artifact_deser_event_example() {
        serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/contracts/artifacts/event_example.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_full_contract_deser_code() {
        serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/raw_gateway_responses/get_full_contract/1_code.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_full_contract_deser_all_abi_types() {
        serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/raw_gateway_responses/get_full_contract/2_all_abi_types.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser() {
        serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/raw_gateway_responses/get_class_by_hash/1_success.txt"
        ))
        .unwrap();
    }
}
