use super::{
    super::serde::unsigned_field_element::UfeHex, AbiEntry, EntryPointsByType, FieldElement,
};

use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    pub builtins: Vec<String>,
    // This field was introduced in Cairo 0.10.0. By making it optional we're keeping compatibility
    // with older artifacts. This decision should be reviewd in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_version: Option<String>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<DebugInfo>,
    pub hints: BTreeMap<u64, Vec<Hint>>,
    pub identifiers: BTreeMap<String, Identifier>,
    pub main_scope: String,
    // Impossible to use [FieldElement] here as by definition field elements are smaller
    // than prime
    pub prime: String,
    pub reference_manager: ReferenceManager,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    pub accessible_scopes: Vec<String>,
    pub end_pc: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_tracking_data: Option<FlowTrackingData>,
    pub name: String,
    pub start_pc: u64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebugInfo {
    /// A partial map from file name to its content. Files that are not in the map, are assumed to
    /// exist in the file system.
    pub file_contents: BTreeMap<String, String>,
    /// A map from (relative) PC to the location of the instruction
    pub instruction_locations: BTreeMap<u64, InstructionLocation>,
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
pub struct InstructionLocation {
    pub accessible_scopes: Vec<String>,
    // This field is serialized as `null` instead of skipped
    pub flow_tracking_data: Option<FlowTrackingData>,
    pub hints: Vec<HintLocation>,
    pub inst: Location,
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
pub struct HintLocation {
    pub location: Location,
    /// The number of new lines following the "%{" symbol
    pub n_prefix_newlines: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub end_col: u64,
    pub end_line: u64,
    pub input_file: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_location: Option<ParentLocation>,
    pub start_col: u64,
    pub start_line: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApTrackingData {
    pub group: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct ParentLocation {
    pub location: Box<Location>,
    pub remark: String,
}

impl Serialize for ParentLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.location)?;
        seq.serialize_element(&self.remark)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ParentLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::Array(mut array) = temp_value {
            if array.len() != 2 {
                return Err(serde::de::Error::custom("length mismatch"));
            }

            let remark = array.pop().unwrap();
            let remark = match remark {
                serde_json::Value::String(remark) => remark,
                _ => return Err(serde::de::Error::custom("unexpected value type")),
            };

            let location = array.pop().unwrap();
            let location = Location::deserialize(location).map_err(|err| {
                serde::de::Error::custom(format!("unable to deserialize Location: {}", err))
            })?;

            Ok(Self {
                location: Box::new(location),
                remark,
            })
        } else {
            Err(serde::de::Error::custom("expected sequencer"))
        }
    }
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

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_artifact_json_equivalence() {
        // Removes '\n' at the end
        let original_text = include_str!("../../test-data/contracts/artifacts/oz_account.txt");
        let original_text = &original_text[..(original_text.len() - 1)];

        let artifact = serde_json::from_str::<ContractArtifact>(original_text).unwrap();
        let serialized = serde_json::to_string(&artifact).unwrap();

        assert_eq!(original_text, serialized);
    }
}
