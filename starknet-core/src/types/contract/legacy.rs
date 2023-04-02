use crate::{
    crypto::compute_hash_on_elements,
    serde::{
        byte_array::base64::serialize as base64_ser, json::to_string_pythonic,
        num_hex::u64 as u64_hex, unsigned_field_element::UfeHex,
    },
    types::{
        contract::{CompressProgramError, ComputeClassHashError},
        FieldElement,
    },
    utils::{cairo_short_string_to_felt, starknet_keccak},
};

use flate2::{write::GzEncoder, Compression};
use serde::{
    de::Error as DeError, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer,
};
use serde_with::{serde_as, SerializeAs};
use std::{collections::BTreeMap, io::Write};

const API_VERSION: FieldElement = FieldElement::ZERO;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractClass {
    pub abi: Vec<LegacyAbiEntry>,
    pub entry_points_by_type: LegacyEntryPoints,
    pub program: LegacyProgram,
}

#[derive(Debug, Serialize, Clone)]
pub struct CompressedLegacyContractClass {
    #[serde(serialize_with = "base64_ser")]
    pub program: Vec<u8>,
    pub entry_points_by_type: LegacyEntryPoints,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<LegacyAbiEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEntryPoints {
    pub constructor: Vec<LegacyEntryPoint>,
    pub external: Vec<LegacyEntryPoint>,
    pub l1_handler: Vec<LegacyEntryPoint>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyProgram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<LegacyAttribute>>,
    pub builtins: Vec<String>,
    // This field was introduced in Cairo 0.10.0. By making it optional we're keeping compatibility
    // with older artifacts. This decision should be reviewd in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_version: Option<String>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
    pub debug_info: Option<LegacyDebugInfo>,
    pub hints: BTreeMap<u64, Vec<LegacyHint>>,
    pub identifiers: BTreeMap<String, LegacyIdentifier>,
    pub main_scope: String,
    // Impossible to use [FieldElement] here as by definition field elements are smaller
    // than prime
    pub prime: String,
    pub reference_manager: LegacyReferenceManager,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEntryPoint {
    pub offset: LegacyEntrypointOffset,
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyAttribute {
    pub accessible_scopes: Vec<String>,
    pub end_pc: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_tracking_data: Option<LegacyFlowTrackingData>,
    pub name: String,
    pub start_pc: u64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyDebugInfo {
    /// A partial map from file name to its content. Files that are not in the map, are assumed to
    /// exist in the file system.
    pub file_contents: BTreeMap<String, String>,
    /// A map from (relative) PC to the location of the instruction
    pub instruction_locations: BTreeMap<u64, LegacyInstructionLocation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyHint {
    pub accessible_scopes: Vec<String>,
    pub code: String,
    pub flow_tracking_data: LegacyFlowTrackingData,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorators: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cairo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<BTreeMap<String, LegacyIdentifierMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<LegacyReference>>,
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
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyReferenceManager {
    pub references: Vec<LegacyReference>,
}

/// This field changed from hex string to number on 0.11.0.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum LegacyEntrypointOffset {
    U64AsHex(#[serde(with = "u64_hex")] u64),
    U64AsInt(u64),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyInstructionLocation {
    pub accessible_scopes: Vec<String>,
    // This field is serialized as `null` instead of skipped
    pub flow_tracking_data: Option<LegacyFlowTrackingData>,
    pub hints: Vec<LegacyHintLocation>,
    pub inst: LegacyLocation,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyIdentifierMember {
    pub cairo_type: String,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyReference {
    pub ap_tracking_data: LegacyApTrackingData,
    pub pc: u64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyFlowTrackingData {
    pub ap_tracking: LegacyApTrackingData,
    pub reference_ids: BTreeMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyHintLocation {
    pub location: LegacyLocation,
    /// The number of new lines following the "%{" symbol
    pub n_prefix_newlines: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyLocation {
    pub end_col: u64,
    pub end_line: u64,
    pub input_file: LegacyInputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_location: Option<LegacyParentLocation>,
    pub start_col: u64,
    pub start_line: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyApTrackingData {
    pub group: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyInputFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct LegacyParentLocation {
    pub location: Box<LegacyLocation>,
    pub remark: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractCode {
    #[serde_as(as = "Vec<UfeHex>")]
    pub bytecode: Vec<FieldElement>,
    pub abi: Option<Vec<LegacyAbiEntry>>,
}

#[derive(Debug, Clone)]
pub enum LegacyAbiEntry {
    Constructor(LegacyConstructor),
    Function(LegacyFunction),
    Struct(LegacyStruct),
    L1Handler(LegacyL1Handler),
    Event(LegacyEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyConstructor {
    pub inputs: Vec<LegacyInput>,
    pub name: String,
    pub outputs: Vec<LegacyOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyFunction {
    pub inputs: Vec<LegacyInput>,
    pub name: String,
    pub outputs: Vec<LegacyOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyStruct {
    pub members: Vec<LegacyMember>,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyL1Handler {
    pub inputs: Vec<LegacyInput>,
    pub name: String,
    pub outputs: Vec<LegacyOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyEvent {
    pub data: Vec<LegacyEventData>,
    pub keys: Vec<LegacyEventData>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyInput {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyOutput {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEventData {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyMember {
    pub name: String,
    pub offset: u64,
    pub r#type: String,
}

struct ProgramForHintedHash;
struct AttributeForHintedHash;

impl From<LegacyEntrypointOffset> for u64 {
    fn from(value: LegacyEntrypointOffset) -> Self {
        match value {
            LegacyEntrypointOffset::U64AsHex(inner) => inner,
            LegacyEntrypointOffset::U64AsInt(inner) => inner,
        }
    }
}

impl From<LegacyEntrypointOffset> for FieldElement {
    fn from(value: LegacyEntrypointOffset) -> Self {
        match value {
            LegacyEntrypointOffset::U64AsHex(inner) => inner.into(),
            LegacyEntrypointOffset::U64AsInt(inner) => inner.into(),
        }
    }
}

// Manually implementing this so we can put `type` at the end:
// https://github.com/xJonathanLEI/starknet-rs/issues/216
impl Serialize for LegacyAbiEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct TypedValue<'a, T> {
            #[serde(flatten)]
            value: &'a T,
            r#type: &'static str,
        }

        match self {
            Self::Constructor(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "constructor",
                },
                serializer,
            ),
            Self::Function(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "function",
                },
                serializer,
            ),
            Self::Struct(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "struct",
                },
                serializer,
            ),
            Self::L1Handler(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "l1_handler",
                },
                serializer,
            ),
            Self::Event(value) => TypedValue::serialize(
                &TypedValue {
                    value: &value,
                    r#type: "event",
                },
                serializer,
            ),
        }
    }
}

// We need to manually implement this because `arbitrary_precision` doesn't work with `tag`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de> Deserialize<'de> for LegacyAbiEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        match &temp_value["type"] {
            serde_json::Value::String(type_str) => match &type_str[..] {
                "constructor" => Ok(LegacyAbiEntry::Constructor(
                    LegacyConstructor::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid constructor variant: {err}"))
                    })?,
                )),
                "function" => Ok(LegacyAbiEntry::Function(
                    LegacyFunction::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid function variant: {err}"))
                    })?,
                )),
                "struct" => Ok(LegacyAbiEntry::Struct(
                    LegacyStruct::deserialize(temp_value)
                        .map_err(|err| DeError::custom(format!("invalid struct variant: {err}")))?,
                )),
                "l1_handler" => Ok(LegacyAbiEntry::L1Handler(
                    LegacyL1Handler::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid l1_handler variant: {err}"))
                    })?,
                )),
                "event" => Ok(LegacyAbiEntry::Event(
                    LegacyEvent::deserialize(temp_value)
                        .map_err(|err| DeError::custom(format!("invalid event variant: {err}")))?,
                )),
                _ => Err(DeError::custom(format!(
                    "unknown ABI entry type: {type_str}"
                ))),
            },
            _ => Err(DeError::custom("invalid type field")),
        }
    }
}

impl LegacyContractClass {
    pub fn class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        let mut elements = vec![];

        elements.push(API_VERSION);

        // Hashes external entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.external.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset.into());
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes L1 handler entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.l1_handler.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset.into());
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes constructor entry points
        elements.push({
            let mut buffer = vec![];
            for entrypoint in self.entry_points_by_type.constructor.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset.into());
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
            abi: &'a Vec<LegacyAbiEntry>,
            #[serde_as(as = "ProgramForHintedHash")]
            program: &'a LegacyProgram,
        }

        // TODO: handle adding extra whitespaces in pre-0.10.0 artifacts for backward compatibility

        let serialized = to_string_pythonic(&ContractArtifactForHash {
            abi: &self.abi,
            program: &self.program,
        })
        .map_err(ComputeClassHashError::Json)?;

        Ok(starknet_keccak(serialized.as_bytes()))
    }

    pub fn compress(&self) -> Result<CompressedLegacyContractClass, CompressProgramError> {
        Ok(CompressedLegacyContractClass {
            program: self.program.compress()?,
            entry_points_by_type: self.entry_points_by_type.clone(),
            abi: Some(self.abi.clone()),
        })
    }
}

impl LegacyProgram {
    pub fn compress(&self) -> Result<Vec<u8>, CompressProgramError> {
        #[serde_as]
        #[derive(Serialize)]
        pub struct ProgramWithoutDebugInfo<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            attributes: &'a Option<Vec<LegacyAttribute>>,
            builtins: &'a Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compiler_version: &'a Option<String>,
            #[serde_as(as = "Vec<UfeHex>")]
            data: &'a Vec<FieldElement>,
            hints: &'a BTreeMap<u64, Vec<LegacyHint>>,
            identifiers: &'a BTreeMap<String, LegacyIdentifier>,
            main_scope: &'a String,
            prime: &'a String,
            reference_manager: &'a LegacyReferenceManager,
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

impl Serialize for LegacyParentLocation {
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

impl<'de> Deserialize<'de> for LegacyParentLocation {
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
            let location = LegacyLocation::deserialize(location).map_err(|err| {
                serde::de::Error::custom(format!("unable to deserialize Location: {err}"))
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

impl SerializeAs<LegacyProgram> for ProgramForHintedHash {
    fn serialize_as<S>(source: &LegacyProgram, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct HashVo<'a> {
            #[serde(skip_serializing_if = "should_skip_attributes_for_hinted_hash")]
            #[serde_as(as = "Option<Vec<AttributeForHintedHash>>")]
            attributes: &'a Option<Vec<LegacyAttribute>>,
            builtins: &'a Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            compiler_version: &'a Option<String>,
            #[serde_as(as = "Vec<UfeHex>")]
            data: &'a Vec<FieldElement>,
            debug_info: &'a Option<LegacyDebugInfo>,
            hints: &'a BTreeMap<u64, Vec<LegacyHint>>,
            identifiers: &'a BTreeMap<String, LegacyIdentifier>,
            main_scope: &'a String,
            prime: &'a String,
            reference_manager: &'a LegacyReferenceManager,
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

impl SerializeAs<LegacyAttribute> for AttributeForHintedHash {
    fn serialize_as<S>(source: &LegacyAttribute, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct HashVo<'a> {
            #[serde(skip_serializing_if = "Vec::is_empty")]
            accessible_scopes: &'a Vec<String>,
            end_pc: &'a u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            flow_tracking_data: &'a Option<LegacyFlowTrackingData>,
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

fn should_skip_attributes_for_hinted_hash(value: &Option<Vec<LegacyAttribute>>) -> bool {
    match value {
        Some(value) => value.is_empty(),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::contract::DeployedClass;

    use super::*;

    #[derive(serde::Deserialize)]
    struct ContractHashes {
        hinted_class_hash: String,
        class_hash: String,
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_legacy_artifact_deser() {
        for raw_artifact in [
            include_str!("../../../test-data/contracts/cairo0/artifacts/oz_account.txt"),
            include_str!("../../../test-data/contracts/cairo0/artifacts/event_example.txt"),
            include_str!("../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/oz_account.txt"),
            include_str!(
                "../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/event_example.txt"
            ),
        ]
        .into_iter()
        {
            serde_json::from_str::<LegacyContractClass>(raw_artifact).unwrap();
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_full_contract_deser_cairo_0() {
        let class = serde_json::from_str::<DeployedClass>(include_str!(
            "../../../test-data/raw_gateway_responses/get_full_contract/1_cairo_0.txt"
        ))
        .unwrap();
        assert!(matches!(class, DeployedClass::LegacyClass(_)));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_full_contract_deser_cairo_1() {
        let class = serde_json::from_str::<DeployedClass>(include_str!(
            "../../../test-data/raw_gateway_responses/get_full_contract/2_cairo_1.txt"
        ))
        .unwrap();
        assert!(matches!(class, DeployedClass::SierraClass(_)));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_cairo_0() {
        let class = serde_json::from_str::<DeployedClass>(include_str!(
            "../../../test-data/raw_gateway_responses/get_class_by_hash/1_cairo_0.txt"
        ))
        .unwrap();
        assert!(matches!(class, DeployedClass::LegacyClass(_)));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_class_by_hash_deser_cairo_1() {
        let class = serde_json::from_str::<DeployedClass>(include_str!(
            "../../../test-data/raw_gateway_responses/get_class_by_hash/3_cairo_1.txt"
        ))
        .unwrap();
        assert!(matches!(class, DeployedClass::SierraClass(_)));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_class_hash() {
        for (raw_artifact, raw_hashes) in [
            (
                include_str!("../../../test-data/contracts/cairo0/artifacts/oz_account.txt"),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/oz_account.hashes.json"
                ),
            ),
            (
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/oz_account.txt"
                ),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/oz_account.hashes.json"
                ),
            ),
        ]
        .into_iter()
        {
            let artifact = serde_json::from_str::<LegacyContractClass>(raw_artifact).unwrap();
            let computed_hash = artifact.class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = FieldElement::from_hex_be(&hashes.class_hash).unwrap();

            assert_eq!(computed_hash, expected_hash);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_hinted_class_hash() {
        for (raw_artifact, raw_hashes) in [
            (
                include_str!("../../../test-data/contracts/cairo0/artifacts/oz_account.txt"),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/oz_account.hashes.json"
                ),
            ),
            (
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/oz_account.txt"
                ),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/pre-0.11.0/oz_account.hashes.json"
                ),
            ),
        ]
        .into_iter()
        {
            let artifact = serde_json::from_str::<LegacyContractClass>(raw_artifact).unwrap();
            let computed_hash = artifact.hinted_class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = FieldElement::from_hex_be(&hashes.hinted_class_hash).unwrap();

            assert_eq!(computed_hash, expected_hash);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_artifact_json_equivalence() {
        // Removes '\n' or "\r\n" at the end
        let original_text =
            include_str!("../../../test-data/contracts/cairo0/artifacts/oz_account.txt");
        let original_text = original_text
            .trim_end_matches("\r\n")
            .trim_end_matches('\n');

        let artifact = serde_json::from_str::<LegacyContractClass>(original_text).unwrap();
        let serialized = serde_json::to_string(&artifact).unwrap();

        assert_eq!(original_text, serialized);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_code_deser() {
        let raw = include_str!("../../../test-data/raw_gateway_responses/get_code/1_code.txt");

        let cc: LegacyContractCode = serde_json::from_str(raw).unwrap();
        let abi = cc.abi.unwrap();

        assert_eq!(cc.bytecode.len(), 1347);
        if let LegacyAbiEntry::Constructor(c) = &abi[0] {
            assert_eq!(c.name, "constructor");
            assert_eq!(c.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly")
        }

        if let LegacyAbiEntry::Function(f) = &abi[1] {
            assert_eq!(f.name, "execute");
            assert_eq!(f.inputs.len(), 5);
            assert_eq!(f.state_mutability, None);
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let LegacyAbiEntry::Function(f) = &abi[9] {
            assert_eq!(f.name, "is_valid_signature");
            assert_eq!(f.inputs.len(), 3);
            assert_eq!(f.state_mutability, Some(String::from("view")));
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_code_deser_all_abi_types() {
        // $ curl "https://alpha4.starknet.io/feeder_gateway/get_code?contractAddress=0x06ef97a90be1c0458f6e7bd1faf05021f2d81211f658155df0c5c97a39eb2d12"
        // Contract built from: https://github.com/starkware-libs/cairo-lang/blob/3d33c4e829a87bc3d88cf04ed6a489e788918b8b/src/starkware/starknet/compiler/starknet_preprocessor_test.py#L143
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_code/2_all_abi_types.txt");
        let cc: LegacyContractCode = serde_json::from_str(raw).unwrap();
        let abi = cc.abi.unwrap();

        if let LegacyAbiEntry::Struct(s) = &abi[0] {
            assert_eq!(s.name, "ExternalStruct3");
            assert_eq!(s.size, 1);
        } else {
            panic!("Did not deserialize AbiEntry::Struct properly");
        }

        if let LegacyAbiEntry::Constructor(c) = &abi[3] {
            assert_eq!(c.name, "constructor");
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly");
        }

        if let LegacyAbiEntry::Function(f) = &abi[5] {
            assert_eq!(f.name, "g");
            assert_eq!(f.outputs.len(), 1);
            assert_eq!(f.state_mutability, Some(String::from("view")));
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let LegacyAbiEntry::L1Handler(h) = &abi[6] {
            assert_eq!(h.name, "handler");
            assert_eq!(h.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::L1Handler properly");
        }
    }
}
