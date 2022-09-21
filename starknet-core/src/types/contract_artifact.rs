use super::{
    super::{
        crypto::compute_hash_on_elements,
        serde::{json::to_string_pythonic, unsigned_field_element::UfeHex},
        utils::{cairo_short_string_to_felt, starknet_keccak},
    },
    AbiEntry, ContractDefinition, EntryPointsByType, FieldElement,
};

use flate2::{write::GzEncoder, Compression};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, SerializeAs};
use std::{collections::BTreeMap, io::Write};

const API_VERSION: FieldElement = FieldElement::ZERO;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractArtifact {
    pub abi: Vec<AbiEntry>,
    pub entry_points_by_type: EntryPointsByType,
    pub program: Program,
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeClassHashError {
    #[error("invalid builtin name")]
    InvalidBuiltinName,
    #[error("json serialization error: {0}")]
    Json(serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CompressProgramError {
    #[error("json serialization error: {0}")]
    Json(serde_json::Error),
    #[error("compression io error: {0}")]
    Io(std::io::Error),
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

struct ProgramForHintedHash;
struct AttributeForHintedHash;

impl ContractArtifact {
    pub fn class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        let mut elements = vec![];

        elements.push(API_VERSION);

        // Hashes external entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.external.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset);
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes L1 handler entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.l1_handler.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset);
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes constructor entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.constructor.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset);
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes builtins
        elements.push(compute_hash_on_elements(
            &self
                .program
                .builtins
                .iter()
                .map(|item| cairo_short_string_to_felt(item))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| ComputeClassHashError::InvalidBuiltinName)?,
        ));

        // Hashes hinted_class_hash
        elements.push(self.hinted_class_hash()?);

        // Hashes bytecode
        elements.push(compute_hash_on_elements(&self.program.data));

        Ok(compute_hash_on_elements(&elements))
    }

    pub fn hinted_class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        #[serde_as]
        #[derive(Serialize)]
        struct ContractArtifactForHash<'a> {
            abi: &'a Vec<AbiEntry>,
            #[serde_as(as = "ProgramForHintedHash")]
            program: &'a Program,
        }

        // TODO: handle adding extra whitespaces in pre-0.10.0 artifacts for backward compatibility

        let serialized = to_string_pythonic(&ContractArtifactForHash {
            abi: &self.abi,
            program: &self.program,
        })
        .map_err(ComputeClassHashError::Json)?;

        Ok(starknet_keccak(serialized.as_bytes()))
    }

    pub fn compress(&self) -> Result<ContractDefinition, CompressProgramError> {
        Ok(ContractDefinition {
            program: self.program.compress()?,
            entry_points_by_type: self.entry_points_by_type.clone(),
            abi: Some(self.abi.clone()),
        })
    }
}

impl Program {
    pub fn compress(&self) -> Result<Vec<u8>, CompressProgramError> {
        #[serde_as]
        #[derive(Serialize)]
        pub struct ProgramWithoutDebugInfo<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            attributes: &'a Option<Vec<Attribute>>,
            builtins: &'a Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compiler_version: &'a Option<String>,
            #[serde_as(as = "Vec<UfeHex>")]
            data: &'a Vec<FieldElement>,
            hints: &'a BTreeMap<u64, Vec<Hint>>,
            identifiers: &'a BTreeMap<String, Identifier>,
            main_scope: &'a String,
            prime: &'a String,
            reference_manager: &'a ReferenceManager,
        }

        let program_json = serde_json::to_string(&ProgramWithoutDebugInfo {
            attributes: &self.attributes,
            builtins: &self.builtins,
            compiler_version: &self.compiler_version,
            data: &self.data,
            hints: &self.hints,
            identifiers: &self.identifiers,
            main_scope: &self.main_scope,
            prime: &self.prime,
            reference_manager: &self.reference_manager,
        })
        .map_err(CompressProgramError::Json)?;

        // Use best compression level to optimize for payload size
        let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gzip_encoder
            .write_all(program_json.as_bytes())
            .map_err(CompressProgramError::Io)?;

        gzip_encoder.finish().map_err(CompressProgramError::Io)
    }
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

impl SerializeAs<Program> for ProgramForHintedHash {
    fn serialize_as<S>(source: &Program, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct HashVo<'a> {
            #[serde(skip_serializing_if = "should_skip_attributes_for_hinted_hash")]
            #[serde_as(as = "Option<Vec<AttributeForHintedHash>>")]
            attributes: &'a Option<Vec<Attribute>>,
            builtins: &'a Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compiler_version: &'a Option<String>,
            #[serde_as(as = "Vec<UfeHex>")]
            data: &'a Vec<FieldElement>,
            debug_info: &'a Option<DebugInfo>,
            hints: &'a BTreeMap<u64, Vec<Hint>>,
            identifiers: &'a BTreeMap<String, Identifier>,
            main_scope: &'a String,
            prime: &'a String,
            reference_manager: &'a ReferenceManager,
        }

        HashVo::serialize(
            &HashVo {
                attributes: &source.attributes,
                builtins: &source.builtins,
                compiler_version: &source.compiler_version,
                data: &source.data,
                debug_info: &None,
                hints: &source.hints,
                identifiers: &source.identifiers,
                main_scope: &source.main_scope,
                prime: &source.prime,
                reference_manager: &source.reference_manager,
            },
            serializer,
        )
    }
}

impl SerializeAs<Attribute> for AttributeForHintedHash {
    fn serialize_as<S>(source: &Attribute, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct HashVo<'a> {
            #[serde(skip_serializing_if = "Vec::is_empty")]
            accessible_scopes: &'a Vec<String>,
            end_pc: &'a u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            flow_tracking_data: &'a Option<FlowTrackingData>,
            name: &'a String,
            start_pc: &'a u64,
            value: &'a String,
        }

        HashVo::serialize(
            &HashVo {
                accessible_scopes: &source.accessible_scopes,
                end_pc: &source.end_pc,
                flow_tracking_data: &source.flow_tracking_data,
                name: &source.name,
                start_pc: &source.start_pc,
                value: &source.value,
            },
            serializer,
        )
    }
}

fn should_skip_attributes_for_hinted_hash(value: &Option<Vec<Attribute>>) -> bool {
    match value {
        Some(value) => value.is_empty(),
        None => true,
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
    fn test_contract_class_hash() {
        let artifact = serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/contracts/artifacts/oz_account.txt"
        ))
        .unwrap();
        let computed_hash = artifact.class_hash().unwrap();

        // Generated with `cairo-lang` v0.10.0
        // TODO: generate this inside Docker
        let expected_hash = FieldElement::from_hex_be(
            "0x0045d788d040561528a1ac1c05f8b1606dc726d88dfa857b66a98ff7d2fbe72a",
        )
        .unwrap();

        assert_eq!(computed_hash, expected_hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_hinted_class_hash() {
        let artifact = serde_json::from_str::<ContractArtifact>(include_str!(
            "../../test-data/contracts/artifacts/oz_account.txt"
        ))
        .unwrap();
        let computed_hash = artifact.hinted_class_hash().unwrap();

        // Generated with `cairo-lang` v0.10.0
        // TODO: generate this inside Docker
        let expected_hash = FieldElement::from_hex_be(
            "0x015b7a3ec0098a3b60fa098dcb966892ad2531459f4910502d44c3adb5a4160e",
        )
        .unwrap();

        assert_eq!(computed_hash, expected_hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_artifact_json_equivalence() {
        // Removes '\n' or "\r\n" at the end
        let original_text = include_str!("../../test-data/contracts/artifacts/oz_account.txt");
        let original_text = original_text
            .trim_end_matches("\r\n")
            .trim_end_matches('\n');

        let artifact = serde_json::from_str::<ContractArtifact>(original_text).unwrap();
        let serialized = serde_json::to_string(&artifact).unwrap();

        assert_eq!(original_text, serialized);
    }
}
