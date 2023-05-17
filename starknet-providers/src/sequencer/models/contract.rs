use serde::{Deserialize, Deserializer, Serialize};
use starknet_core::{
    serde::byte_array::base64::serialize as base64_ser,
    types::{
        contract::legacy::{LegacyContractClass, RawLegacyAbiEntry, RawLegacyEntryPoints},
        FlattenedSierraClass,
    },
};

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DeployedClass {
    SierraClass(FlattenedSierraClass),
    LegacyClass(LegacyContractClass),
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

// We need to manually implement this because `arbitrary_precision` doesn't work with `untagged`:
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
