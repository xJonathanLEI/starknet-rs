use std::collections::BTreeMap;

use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use starknet_crypto::{poseidon_hash_many, PoseidonHasher};

use crate::{
    serde::{json::to_string_pythonic, unsigned_field_element::UfeHex},
    types::FieldElement,
    utils::{cairo_short_string_to_felt, starknet_keccak, CairoShortStringToFeltError},
};

/// Module containing types related to artifacts of contracts compiled with a Cairo 0.x compiler.
pub mod legacy;

/// Cairo string for "COMPILED_CLASS_V1"
const PREFIX_COMPILED_CLASS_V1: FieldElement = FieldElement::from_mont([
    2291010424822318237,
    1609463842841646376,
    18446744073709549462,
    324306817650036332,
]);

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
#[derive(Debug, Serialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EntrypointList<E> {
    pub external: Vec<E>,
    pub l1_handler: Vec<E>,
    pub constructor: Vec<E>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Struct(AbiStruct),
    Enum(AbiEnum),
}

#[derive(Debug)]
pub struct Hint {
    pub id: u64,
    pub code: Vec<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub inputs: Vec<AbiNamedMember>,
    pub outputs: Vec<AbiOutput>,
    pub state_mutability: StateMutability,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEvent {
    pub name: String,
    pub inputs: Vec<AbiNamedMember>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiStruct {
    pub name: String,
    pub members: Vec<AbiNamedMember>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEnum {
    pub name: String,
    pub variants: Vec<AbiNamedMember>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiNamedMember {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiOutput {
    pub r#type: String,
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

impl CompiledClass {
    pub fn class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        let mut hasher = PoseidonHasher::new();
        hasher.update(PREFIX_COMPILED_CLASS_V1);

        // Hashes entry points
        hasher.update(
            Self::hash_entrypoints(&self.entry_points_by_type.external)
                .map_err(|_| ComputeClassHashError::InvalidBuiltinName)?,
        );
        hasher.update(
            Self::hash_entrypoints(&self.entry_points_by_type.l1_handler)
                .map_err(|_| ComputeClassHashError::InvalidBuiltinName)?,
        );
        hasher.update(
            Self::hash_entrypoints(&self.entry_points_by_type.constructor)
                .map_err(|_| ComputeClassHashError::InvalidBuiltinName)?,
        );

        // Hashes hinted_compiled_class_hash
        hasher.update(self.hinted_class_hash()?);

        // Hashes bytecode
        hasher.update(poseidon_hash_many(&self.bytecode));

        Ok(hasher.finalize())
    }

    pub fn hinted_class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        #[derive(Serialize)]
        struct ClassForHintedHash<'a> {
            program: ProgramForHintedHash<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        struct ProgramForHintedHash<'a> {
            // Hard-coded to be empty
            builtins: [(); 0],
            #[serde(skip_serializing_if = "Option::is_none")]
            compiler_version: Option<&'a String>,
            #[serde_as(as = "Vec<UfeHex>")]
            data: &'a Vec<FieldElement>,
            hints: BTreeMap<u64, Vec<HintForHintedHash<'a>>>,
            prime: &'a String,
        }

        #[derive(Serialize)]
        struct HintForHintedHash<'a> {
            // Hard-coded to be empty
            accessible_scopes: [(); 0],
            code: &'a String,
            flow_tracking_data: &'a EmptyFlowTrackingData<'a>,
        }

        #[derive(Serialize)]
        struct EmptyFlowTrackingData<'a> {
            ap_tracking: &'a EmptyApTrackingData,
            reference_ids: EmptyReferenceIds,
        }

        #[derive(Default, Serialize)]
        struct EmptyApTrackingData {
            group: u64,
            offset: u64,
        }

        #[derive(Default, Serialize)]
        struct EmptyReferenceIds {}

        let empty_ap_tracking_data = EmptyApTrackingData::default();
        let empty_flow_tracking_data = EmptyFlowTrackingData {
            ap_tracking: &empty_ap_tracking_data,
            reference_ids: EmptyReferenceIds {},
        };

        // We shouldn't need to this if artifacts are guaranteed to have sorted hints?
        // TODO: check compiler to see if it's guanranteed
        let mut hints = BTreeMap::<u64, Vec<HintForHintedHash>>::new();
        for hint in self.hints.iter() {
            let transformed_hints = hint
                .code
                .iter()
                .map(|code| HintForHintedHash {
                    accessible_scopes: [],
                    code,
                    flow_tracking_data: &empty_flow_tracking_data,
                })
                .collect::<Vec<_>>();
            hints.insert(hint.id, transformed_hints);
        }

        let serialized = to_string_pythonic(&ClassForHintedHash {
            program: ProgramForHintedHash {
                builtins: [],
                compiler_version: Some(&self.compiler_version),
                data: &self.bytecode,
                hints,
                prime: &self.prime,
            },
        })
        .map_err(ComputeClassHashError::Json)?;

        Ok(starknet_keccak(serialized.as_bytes()))
    }

    fn hash_entrypoints(
        entrypoints: &[CompiledClassEntrypoint],
    ) -> Result<FieldElement, CairoShortStringToFeltError> {
        let mut hasher = PoseidonHasher::new();

        for entry in entrypoints.iter() {
            hasher.update(entry.selector);
            hasher.update(entry.offset.into());

            let mut builtin_hasher = PoseidonHasher::new();
            for builtin in entry.builtins.iter() {
                builtin_hasher.update(cairo_short_string_to_felt(builtin)?)
            }

            hasher.update(builtin_hasher.finalize());
        }

        Ok(hasher.finalize())
    }
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

// Temporary workarond until this gets fixed, after which we drop the custom impl.
//   https://github.com/starkware-libs/cairo/issues/2350
//
// Since there's currently no officially supported way of generating valid artifacts, our only
// alternative is to ask users to manually patch the JSON files before loading them, which is far
// worse than what we do here.
impl<'de> Deserialize<'de> for CompiledClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Intermediate {
            prime: String,
            compiler_version: String,
            #[serde_as(as = "Vec<UfeHex>")]
            bytecode: Vec<FieldElement>,
            hints: Vec<Hint>,
            entry_points_by_type: EntrypointList<CompiledClassEntrypoint>,
        }

        let mut intermediate = Intermediate::deserialize(deserializer)?;
        intermediate
            .entry_points_by_type
            .external
            .sort_by_key(|entry| entry.selector);
        intermediate
            .entry_points_by_type
            .l1_handler
            .sort_by_key(|entry| entry.selector);
        intermediate
            .entry_points_by_type
            .constructor
            .sort_by_key(|entry| entry.selector);

        Ok(Self {
            prime: intermediate.prime,
            compiler_version: intermediate.compiler_version,
            bytecode: intermediate.bytecode,
            hints: intermediate.hints,
            entry_points_by_type: intermediate.entry_points_by_type,
        })
    }
}

impl Serialize for Hint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.id)?;
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

            let id = array.pop().unwrap();
            let id = match id {
                serde_json::Value::Number(id) => id
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("id value out of range"))?,
                _ => return Err(serde::de::Error::custom("unexpected value type")),
            };

            Ok(Self { id, code })
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
        // Artifacts generated from cairo v1.0.0-alpha.6
        for raw_artifact in [
            include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types_sierra.txt"),
            include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_sierra.txt"),
        ]
        .into_iter()
        {
            match serde_json::from_str::<ContractArtifact>(raw_artifact) {
                Ok(ContractArtifact::SierraClass(_)) => {}
                _ => panic!("Unexpected result"),
            }
        }
    }

    #[test]
    #[ignore = "Disabled until casm code rework is done"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compiled_class_deser() {
        // Artifacts generated from cairo v1.0.0-alpha.6
        for raw_artifact in [
            include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types_compiled.txt"),
            include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_compiled.txt"),
        ]
        .into_iter()
        {
            match serde_json::from_str::<ContractArtifact>(raw_artifact) {
                Ok(ContractArtifact::CompiledClass(_)) => {}
                _ => panic!("Unexpected result"),
            }
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_legacy_class_deser() {
        match serde_json::from_str::<ContractArtifact>(include_str!(
            "../../../test-data/contracts/cairo0/artifacts/oz_account.txt"
        )) {
            Ok(ContractArtifact::LegacyClass(_)) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    #[ignore = "Disabled until casm code rework is done"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compiled_class_hash() {
        // Hash obtained from sequencer in integration environment
        // TODO: automate class hash generation like for legacy contracts
        let expected_hash = FieldElement::from_hex_be(
            "0xb638af50c673869f62fd0079232a79a3f508532c7ec24af78cdf36b9dbbe6b",
        )
        .unwrap();

        let compiled_class = serde_json::from_str::<CompiledClass>(include_str!(
            "../../../test-data/contracts/cairo1/artifacts/erc20_compiled.txt"
        ))
        .unwrap();
        let computed_hash = compiled_class.class_hash().unwrap();

        assert_eq!(expected_hash, computed_hash);
    }
}
