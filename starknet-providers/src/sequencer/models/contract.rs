use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::{byte_array::base64::serialize as base64_ser, unsigned_field_element::UfeHex},
    types::{
        contract::{
            legacy::{LegacyContractClass, RawLegacyAbiEntry, RawLegacyEntryPoints},
            CompressProgramError,
        },
        EntryPointsByType, FieldElement, FlattenedSierraClass,
    },
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DeployedClass {
    SierraClass(FlattenedSierraClass),
    LegacyClass(LegacyContractClass),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedSierraClass {
    #[serde(serialize_with = "base64_ser")]
    pub sierra_program: Vec<u8>,
    pub contract_class_version: String,
    pub entry_points_by_type: EntryPointsByType,
    pub abi: String,
}

/// This type exists because of an `offset` issue. Without this type declaration of pre 0.11.0
/// contracts against the sequencer gateway won't function properly.
#[derive(Debug, Serialize, Clone)]
pub struct CompressedLegacyContractClass {
    #[serde(serialize_with = "base64_ser")]
    pub program: Vec<u8>,
    pub entry_points_by_type: RawLegacyEntryPoints,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<RawLegacyAbiEntry>>,
}

// We need to manually implement this because `raw_value` doesn't work with `untagged`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de> Deserialize<'de> for DeployedClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        if let Ok(value) = FlattenedSierraClass::deserialize(&temp_value) {
            return Ok(Self::SierraClass(value));
        }
        if let Ok(value) = LegacyContractClass::deserialize(&temp_value) {
            return Ok(Self::LegacyClass(value));
        }
        Err(serde::de::Error::custom(
            "data did not match any variant of enum DeployedClass",
        ))
    }
}

impl CompressedSierraClass {
    pub fn from_flattened(
        flattened_class: &FlattenedSierraClass,
    ) -> Result<Self, CompressProgramError> {
        #[serde_as]
        #[derive(Serialize)]
        struct SierraProgram<'a>(#[serde_as(as = "Vec<UfeHex>")] &'a Vec<FieldElement>);

        let program_json = serde_json::to_string(&SierraProgram(&flattened_class.sierra_program))
            .map_err(CompressProgramError::Json)?;

        // Use best compression level to optimize for payload size
        let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gzip_encoder
            .write_all(program_json.as_bytes())
            .map_err(CompressProgramError::Io)?;

        let compressed_program = gzip_encoder.finish().map_err(CompressProgramError::Io)?;

        Ok(CompressedSierraClass {
            sierra_program: compressed_program,
            contract_class_version: flattened_class.contract_class_version.clone(),
            entry_points_by_type: flattened_class.entry_points_by_type.clone(),
            abi: flattened_class.abi.clone(),
        })
    }
}
