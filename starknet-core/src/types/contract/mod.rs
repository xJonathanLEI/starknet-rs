use crate::{serde::unsigned_field_element::UfeHex, types::FieldElement};

use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

/// Module containing types related to artifacts of contracts compiled with a Cairo 0.x compiler.
pub mod legacy;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContractArtifact {
    SierraClass(SierraClass),
    CompiledClass(CompiledClass),
    LegacyClass(legacy::LegacyContractClass),
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraClass {
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<FieldElement>,
    pub sierra_program_debug_info: SierraClassDebugInfo,
    pub contract_class_version: String,
    pub entry_points_by_type: EntrypointList<SierraClassEntrypoint>,
    pub abi: Vec<AbiEntry>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompiledClass {
    pub prime: String,
    pub compiler_version: String,
    #[serde_as(as = "Vec<UfeHex>")]
    pub bytecode: Vec<FieldElement>,
    pub hints: Vec<Hint>,
    pub entry_points_by_type: EntrypointList<CompiledClassEntrypoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraClassDebugInfo {
    pub type_names: Vec<(u64, String)>,
    pub libfunc_names: Vec<(u64, String)>,
    pub user_func_names: Vec<(u64, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EntrypointList<E> {
    external: Vec<E>,
    l1_handler: Vec<E>,
    constructor: Vec<E>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraClassEntrypoint {
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
    pub function_idx: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum AbiEntry {
    Function(AbiFunction),
    Event(AbiEvent),
}

#[derive(Debug)]
pub struct Hint {
    offset: u64,
    code: Vec<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompiledClassEntrypoint {
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
    pub offset: u64,
    pub builtins: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiFunction {
    pub name: String,
    pub inputs: Vec<AbiInput>,
    pub output_ty: String,
    pub state_mutability: StateMutability,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEvent {
    pub name: String,
    pub inputs: Vec<AbiInput>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiInput {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMutability {
    External,
    View,
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

// We need to manually implement this because `arbitrary_precision` doesn't work with `untagged`:
//   https://github.com/serde-rs/serde/issues/1183
impl<'de> Deserialize<'de> for ContractArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        if let Ok(value) = SierraClass::deserialize(&temp_value) {
            return Ok(Self::SierraClass(value));
        }
        if let Ok(value) = CompiledClass::deserialize(&temp_value) {
            return Ok(Self::CompiledClass(value));
        }
        if let Ok(value) = legacy::LegacyContractClass::deserialize(&temp_value) {
            return Ok(Self::LegacyClass(value));
        }
        Err(serde::de::Error::custom(
            "data did not match any variant of enum ContractArtifact",
        ))
    }
}

impl Serialize for Hint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.offset)?;
        seq.serialize_element(&self.code)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Hint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp_value = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::Array(mut array) = temp_value {
            if array.len() != 2 {
                return Err(serde::de::Error::custom("length mismatch"));
            }

            let code = array.pop().unwrap();
            let code = Vec::<String>::deserialize(code).map_err(|err| {
                serde::de::Error::custom(format!("unable to deserialize Location: {err}"))
            })?;

            let offset = array.pop().unwrap();
            let offset = match offset {
                serde_json::Value::Number(offset) => offset
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("offset value out of range"))?,
                _ => return Err(serde::de::Error::custom("unexpected value type")),
            };

            Ok(Self { offset, code })
        } else {
            Err(serde::de::Error::custom("expected sequence"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_sierra_class_deser() {
        // Artifact generated from cairo v1.0.0-alpha.3
        match serde_json::from_str::<ContractArtifact>(include_str!(
            "../../../test-data/contracts/artifacts/erc20_sierra.txt"
        )) {
            Ok(ContractArtifact::SierraClass(_)) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compiled_class_deser() {
        // Artifact generated from cairo v1.0.0-alpha.3
        match serde_json::from_str::<ContractArtifact>(include_str!(
            "../../../test-data/contracts/artifacts/erc20_compiled.txt"
        )) {
            Ok(ContractArtifact::CompiledClass(_)) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_legacy_class_deser() {
        match serde_json::from_str::<ContractArtifact>(include_str!(
            "../../../test-data/contracts/artifacts/legacy/oz_account.txt"
        )) {
            Ok(ContractArtifact::LegacyClass(_)) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
