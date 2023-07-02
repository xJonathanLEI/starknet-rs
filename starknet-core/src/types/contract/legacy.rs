use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec::Vec};

use crate::{
    crypto::compute_hash_on_elements,
    serde::{num_hex::u64 as u64_hex, unsigned_field_element::UfeHex},
    types::{
        contract::{ComputeClassHashError, JsonError},
        FieldElement, FunctionStateMutability, LegacyContractAbiEntry, LegacyContractEntryPoint,
        LegacyEntryPointsByType, LegacyEventAbiEntry, LegacyEventAbiType, LegacyFunctionAbiEntry,
        LegacyFunctionAbiType, LegacyStructAbiEntry, LegacyStructAbiType, LegacyStructMember,
        LegacyTypedParameter,
    },
    utils::{cairo_short_string_to_felt, starknet_keccak},
};

use serde::{
    de::Error as DeError, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json_pythonic::to_string_pythonic;
use serde_with::{serde_as, SerializeAs};

#[cfg(feature = "std")]
use crate::types::{contract::CompressProgramError, CompressedLegacyContractClass};
#[cfg(feature = "std")]
use flate2::{write::GzEncoder, Compression};

const API_VERSION: FieldElement = FieldElement::ZERO;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractClass {
    pub abi: Vec<RawLegacyAbiEntry>,
    pub entry_points_by_type: RawLegacyEntryPoints,
    pub program: LegacyProgram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct RawLegacyEntryPoints {
    pub constructor: Vec<RawLegacyEntryPoint>,
    pub external: Vec<RawLegacyEntryPoint>,
    pub l1_handler: Vec<RawLegacyEntryPoint>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct RawLegacyEntryPoint {
    pub offset: LegacyEntrypointOffset,
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyDebugInfo {
    /// A partial map from file name to its content. Files that are not in the map, are assumed to
    /// exist in the file system.
    pub file_contents: BTreeMap<String, String>,
    /// A map from (relative) PC to the location of the instruction
    pub instruction_locations: BTreeMap<u64, LegacyInstructionLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyHint {
    pub accessible_scopes: Vec<String>,
    pub code: String,
    pub flow_tracking_data: LegacyFlowTrackingData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub value: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyInstructionLocation {
    pub accessible_scopes: Vec<String>,
    // This field is serialized as `null` instead of skipped
    pub flow_tracking_data: Option<LegacyFlowTrackingData>,
    pub hints: Vec<LegacyHintLocation>,
    pub inst: LegacyLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyIdentifierMember {
    pub cairo_type: String,
    pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyReference {
    pub ap_tracking_data: LegacyApTrackingData,
    pub pc: u64,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyFlowTrackingData {
    pub ap_tracking: LegacyApTrackingData,
    pub reference_ids: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyHintLocation {
    pub location: LegacyLocation,
    /// The number of new lines following the "%{" symbol
    pub n_prefix_newlines: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyApTrackingData {
    pub group: u64,
    pub offset: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyInputFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LegacyParentLocation {
    pub location: Box<LegacyLocation>,
    pub remark: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractCode {
    #[serde_as(as = "Vec<UfeHex>")]
    pub bytecode: Vec<FieldElement>,
    pub abi: Option<Vec<RawLegacyAbiEntry>>,
}

#[derive(Debug, Clone)]
pub enum RawLegacyAbiEntry {
    Constructor(RawLegacyConstructor),
    Function(RawLegacyFunction),
    Struct(RawLegacyStruct),
    L1Handler(RawLegacyL1Handler),
    Event(RawLegacyEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLegacyConstructor {
    pub inputs: Vec<LegacyTypedParameter>,
    pub name: String,
    pub outputs: Vec<LegacyTypedParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawLegacyFunction {
    pub inputs: Vec<LegacyTypedParameter>,
    pub name: String,
    pub outputs: Vec<LegacyTypedParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<FunctionStateMutability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLegacyStruct {
    pub members: Vec<RawLegacyMember>,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLegacyL1Handler {
    pub inputs: Vec<LegacyTypedParameter>,
    pub name: String,
    pub outputs: Vec<LegacyTypedParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawLegacyEvent {
    pub data: Vec<LegacyTypedParameter>,
    pub keys: Vec<LegacyTypedParameter>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct RawLegacyMember {
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
impl Serialize for RawLegacyAbiEntry {
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

// We need to manually implement this because `raw_value` doesn't work with `tag`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de> Deserialize<'de> for RawLegacyAbiEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        match &temp_value["type"] {
            serde_json::Value::String(type_str) => match &type_str[..] {
                "constructor" => Ok(RawLegacyAbiEntry::Constructor(
                    RawLegacyConstructor::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid constructor variant: {err}"))
                    })?,
                )),
                "function" => Ok(RawLegacyAbiEntry::Function(
                    RawLegacyFunction::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid function variant: {err}"))
                    })?,
                )),
                "struct" => Ok(RawLegacyAbiEntry::Struct(
                    RawLegacyStruct::deserialize(temp_value)
                        .map_err(|err| DeError::custom(format!("invalid struct variant: {err}")))?,
                )),
                "l1_handler" => Ok(RawLegacyAbiEntry::L1Handler(
                    RawLegacyL1Handler::deserialize(temp_value).map_err(|err| {
                        DeError::custom(format!("invalid l1_handler variant: {err}"))
                    })?,
                )),
                "event" => Ok(RawLegacyAbiEntry::Event(
                    RawLegacyEvent::deserialize(temp_value)
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
        let mut elements = Vec::new();

        elements.push(API_VERSION);

        // Hashes external entry points
        elements.push({
            let mut buffer = Vec::new();
            for entrypoint in self.entry_points_by_type.external.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset.into());
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes L1 handler entry points
        elements.push({
            let mut buffer = Vec::new();
            for entrypoint in self.entry_points_by_type.l1_handler.iter() {
                buffer.push(entrypoint.selector);
                buffer.push(entrypoint.offset.into());
            }
            compute_hash_on_elements(&buffer)
        });

        // Hashes constructor entry points
        elements.push({
            let mut buffer = Vec::new();
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
            abi: &'a Vec<RawLegacyAbiEntry>,
            #[serde_as(as = "ProgramForHintedHash")]
            program: &'a LegacyProgram,
        }

        // TODO: handle adding extra whitespaces in pre-0.10.0 artifacts for backward compatibility

        let serialized = to_string_pythonic(&ContractArtifactForHash {
            abi: &self.abi,
            program: &self.program,
        })
        .map_err(|err| {
            ComputeClassHashError::Json(JsonError {
                message: format!("{}", err),
            })
        })?;

        Ok(starknet_keccak(serialized.as_bytes()))
    }

    #[cfg(feature = "std")]
    pub fn compress(&self) -> Result<CompressedLegacyContractClass, CompressProgramError> {
        Ok(CompressedLegacyContractClass {
            program: self.program.compress()?,
            entry_points_by_type: self.entry_points_by_type.clone().into(),
            abi: Some(
                self.abi
                    .clone()
                    .into_iter()
                    .map(|item| item.into())
                    .collect(),
            ),
        })
    }
}

impl LegacyProgram {
    #[cfg(feature = "std")]
    pub fn compress(&self) -> Result<Vec<u8>, CompressProgramError> {
        use std::io::Write;

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
        .map_err(|err| {
            CompressProgramError::Json(JsonError {
                message: format!("{}", err),
            })
        })?;

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

impl From<RawLegacyEntryPoints> for LegacyEntryPointsByType {
    fn from(value: RawLegacyEntryPoints) -> Self {
        Self {
            constructor: value
                .constructor
                .into_iter()
                .map(|item| item.into())
                .collect(),
            external: value.external.into_iter().map(|item| item.into()).collect(),
            l1_handler: value
                .l1_handler
                .into_iter()
                .map(|item| item.into())
                .collect(),
        }
    }
}

impl From<RawLegacyEntryPoint> for LegacyContractEntryPoint {
    fn from(value: RawLegacyEntryPoint) -> Self {
        Self {
            offset: value.offset.into(),
            selector: value.selector,
        }
    }
}

impl From<RawLegacyAbiEntry> for LegacyContractAbiEntry {
    fn from(value: RawLegacyAbiEntry) -> Self {
        match value {
            RawLegacyAbiEntry::Constructor(entry) => Self::Function(entry.into()),
            RawLegacyAbiEntry::Function(entry) => Self::Function(entry.into()),
            RawLegacyAbiEntry::Struct(entry) => Self::Struct(entry.into()),
            RawLegacyAbiEntry::L1Handler(entry) => Self::Function(entry.into()),
            RawLegacyAbiEntry::Event(entry) => Self::Event(entry.into()),
        }
    }
}

impl From<RawLegacyConstructor> for LegacyFunctionAbiEntry {
    fn from(value: RawLegacyConstructor) -> Self {
        Self {
            r#type: LegacyFunctionAbiType::Constructor,
            name: value.name,
            inputs: value.inputs,
            outputs: value.outputs,
            state_mutability: None,
        }
    }
}

impl From<RawLegacyFunction> for LegacyFunctionAbiEntry {
    fn from(value: RawLegacyFunction) -> Self {
        Self {
            r#type: LegacyFunctionAbiType::Function,
            name: value.name,
            inputs: value.inputs,
            outputs: value.outputs,
            state_mutability: value.state_mutability,
        }
    }
}

impl From<RawLegacyStruct> for LegacyStructAbiEntry {
    fn from(value: RawLegacyStruct) -> Self {
        Self {
            r#type: LegacyStructAbiType::Struct,
            name: value.name,
            size: value.size,
            members: value.members.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<RawLegacyL1Handler> for LegacyFunctionAbiEntry {
    fn from(value: RawLegacyL1Handler) -> Self {
        Self {
            r#type: LegacyFunctionAbiType::L1Handler,
            name: value.name,
            inputs: value.inputs,
            outputs: value.outputs,
            state_mutability: None,
        }
    }
}

impl From<RawLegacyEvent> for LegacyEventAbiEntry {
    fn from(value: RawLegacyEvent) -> Self {
        Self {
            r#type: LegacyEventAbiType::Event,
            name: value.name,
            keys: value.keys,
            data: value.data,
        }
    }
}

impl From<RawLegacyMember> for LegacyStructMember {
    fn from(value: RawLegacyMember) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
            offset: value.offset,
        }
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
    fn test_contract_class_hash() {
        for (raw_artifact, raw_hashes) in [
            (
                include_str!("../../../test-data/contracts/cairo0/artifacts/oz_account.txt"),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/oz_account.hashes.json"
                ),
            ),
            (
                include_str!("../../../test-data/contracts/cairo0/artifacts/emoji.txt"),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/emoji.hashes.json"
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
                include_str!("../../../test-data/contracts/cairo0/artifacts/emoji.txt"),
                include_str!(
                    "../../../test-data/contracts/cairo0/artifacts/emoji.hashes.json"
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
        if let RawLegacyAbiEntry::Constructor(c) = &abi[0] {
            assert_eq!(c.name, "constructor");
            assert_eq!(c.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly")
        }

        if let RawLegacyAbiEntry::Function(f) = &abi[1] {
            assert_eq!(f.name, "execute");
            assert_eq!(f.inputs.len(), 5);
            assert_eq!(f.state_mutability, None);
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let RawLegacyAbiEntry::Function(f) = &abi[9] {
            assert_eq!(f.name, "is_valid_signature");
            assert_eq!(f.inputs.len(), 3);
            assert_eq!(f.state_mutability, Some(FunctionStateMutability::View));
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

        if let RawLegacyAbiEntry::Struct(s) = &abi[0] {
            assert_eq!(s.name, "ExternalStruct3");
            assert_eq!(s.size, 1);
        } else {
            panic!("Did not deserialize AbiEntry::Struct properly");
        }

        if let RawLegacyAbiEntry::Constructor(c) = &abi[3] {
            assert_eq!(c.name, "constructor");
        } else {
            panic!("Did not deserialize AbiEntry::Constructor properly");
        }

        if let RawLegacyAbiEntry::Function(f) = &abi[5] {
            assert_eq!(f.name, "g");
            assert_eq!(f.outputs.len(), 1);
            assert_eq!(f.state_mutability, Some(FunctionStateMutability::View));
        } else {
            panic!("Did not deserialize AbiEntry::Function properly");
        }

        if let RawLegacyAbiEntry::L1Handler(h) = &abi[6] {
            assert_eq!(h.name, "handler");
            assert_eq!(h.inputs.len(), 2);
        } else {
            panic!("Did not deserialize AbiEntry::L1Handler properly");
        }
    }
}
