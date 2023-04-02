use std::io::Write;

use flate2::{write::GzEncoder, Compression};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use starknet_crypto::{poseidon_hash_many, PoseidonHasher};

use crate::{
    serde::{
        byte_array::base64::serialize as base64_ser, json::to_string_pythonic,
        unsigned_field_element::UfeHex,
    },
    types::FieldElement,
    utils::{
        cairo_short_string_to_felt, normalize_address, starknet_keccak, CairoShortStringToFeltError,
    },
};

/// Module containing types related to artifacts of contracts compiled with a Cairo 0.x compiler.
pub mod legacy;

/// Cairo string for "CONTRACT_CLASS_V0.1.0"
const PREFIX_CONTRACT_CLASS_V0_1_0: FieldElement = FieldElement::from_mont([
    5800711240972404213,
    15539482671244488427,
    18446734822722598327,
    37302452645455172,
]);

/// Cairo string for "COMPILED_CLASS_V1"
const PREFIX_COMPILED_CLASS_V1: FieldElement = FieldElement::from_mont([
    2291010424822318237,
    1609463842841646376,
    18446744073709549462,
    324306817650036332,
]);

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ContractArtifact {
    SierraClass(SierraClass),
    CompiledClass(CompiledClass),
    LegacyClass(legacy::LegacyContractClass),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DeployedClass {
    SierraClass(FlattenedSierraClass),
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
    pub pythonic_hints: Option<Vec<PythonicHint>>,
    pub entry_points_by_type: EntrypointList<CompiledClassEntrypoint>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FlattenedSierraClass {
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<FieldElement>,
    pub contract_class_version: String,
    pub entry_points_by_type: EntrypointList<SierraClassEntrypoint>,
    pub abi: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedSierraClass {
    #[serde(serialize_with = "base64_ser")]
    pub sierra_program: Vec<u8>,
    pub contract_class_version: String,
    pub entry_points_by_type: EntrypointList<SierraClassEntrypoint>,
    pub abi: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Hint {
    pub id: u64,
    // For convenience we just treat it as an opaque JSON value here, unless a use case justifies
    // implementing the structure. (We no longer need the hints for the class hash anyways.)
    pub code: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub struct PythonicHint {
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

impl SierraClass {
    pub fn class_hash(&self) -> Result<FieldElement, ComputeClassHashError> {
        // Technically we don't have to use the Pythonic JSON style here. Doing this just to align
        // with the official `cairo-lang` CLI.
        //
        // TODO: add an `AbiFormatter` trait and let users choose which one to use.
        let abi_str = to_string_pythonic(&self.abi).map_err(ComputeClassHashError::Json)?;

        let mut hasher = PoseidonHasher::new();
        hasher.update(PREFIX_CONTRACT_CLASS_V0_1_0);

        // Hashes entry points
        hasher.update(hash_sierra_entrypoints(&self.entry_points_by_type.external));
        hasher.update(hash_sierra_entrypoints(
            &self.entry_points_by_type.l1_handler,
        ));
        hasher.update(hash_sierra_entrypoints(
            &self.entry_points_by_type.constructor,
        ));

        // Hashes ABI
        hasher.update(starknet_keccak(abi_str.as_bytes()));

        // Hashes Sierra program
        hasher.update(poseidon_hash_many(&self.sierra_program));

        Ok(normalize_address(hasher.finalize()))
    }

    pub fn flatten(self) -> Result<FlattenedSierraClass, serde_json::Error> {
        let abi = to_string_pythonic(&self.abi)?;

        Ok(FlattenedSierraClass {
            sierra_program: self.sierra_program,
            contract_class_version: self.contract_class_version,
            entry_points_by_type: self.entry_points_by_type,
            abi,
        })
    }
}

impl FlattenedSierraClass {
    pub fn class_hash(&self) -> FieldElement {
        let mut hasher = PoseidonHasher::new();
        hasher.update(PREFIX_CONTRACT_CLASS_V0_1_0);

        // Hashes entry points
        hasher.update(hash_sierra_entrypoints(&self.entry_points_by_type.external));
        hasher.update(hash_sierra_entrypoints(
            &self.entry_points_by_type.l1_handler,
        ));
        hasher.update(hash_sierra_entrypoints(
            &self.entry_points_by_type.constructor,
        ));

        // Hashes ABI
        hasher.update(starknet_keccak(self.abi.as_bytes()));

        // Hashes Sierra program
        hasher.update(poseidon_hash_many(&self.sierra_program));

        normalize_address(hasher.finalize())
    }

    pub fn compress(&self) -> Result<CompressedSierraClass, CompressProgramError> {
        #[serde_as]
        #[derive(Serialize)]
        struct SierraProgram<'a>(#[serde_as(as = "Vec<UfeHex>")] &'a Vec<FieldElement>);

        let program_json = serde_json::to_string(&SierraProgram(&self.sierra_program))
            .map_err(CompressProgramError::Json)?;

        // Use best compression level to optimize for payload size
        let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::best());
        gzip_encoder
            .write_all(program_json.as_bytes())
            .map_err(CompressProgramError::Io)?;

        let compressed_program = gzip_encoder.finish().map_err(CompressProgramError::Io)?;

        Ok(CompressedSierraClass {
            sierra_program: compressed_program,
            contract_class_version: self.contract_class_version.clone(),
            entry_points_by_type: self.entry_points_by_type.clone(),
            abi: self.abi.clone(),
        })
    }
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

        // Hashes bytecode
        hasher.update(poseidon_hash_many(&self.bytecode));

        Ok(hasher.finalize())
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
        if let Ok(value) = legacy::LegacyContractClass::deserialize(&temp_value) {
            return Ok(Self::LegacyClass(value));
        }
        Err(serde::de::Error::custom(
            "data did not match any variant of enum DeployedClass",
        ))
    }
}

impl Serialize for PythonicHint {
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

impl<'de> Deserialize<'de> for PythonicHint {
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

fn hash_sierra_entrypoints(entrypoints: &[SierraClassEntrypoint]) -> FieldElement {
    let mut hasher = PoseidonHasher::new();

    for entry in entrypoints.iter() {
        hasher.update(entry.selector);
        hasher.update(entry.function_idx.into());
    }

    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct ContractHashes {
        sierra_class_hash: String,
        compiled_class_hash: String,
    }

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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_sierra_class_hash() {
        for (raw_artifact, raw_hashes) in [
            (
                include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_sierra.txt"),
                include_str!("../../../test-data/contracts/cairo1/artifacts/erc20.hashes.json"),
            ),
            (
                include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types_sierra.txt"),
                include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types.hashes.json"),
            ),
        ]
        .into_iter()
        {
            let sierra_class = serde_json::from_str::<SierraClass>(raw_artifact).unwrap();
            let computed_hash = sierra_class.class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = FieldElement::from_hex_be(&hashes.sierra_class_hash).unwrap();

            assert_eq!(computed_hash, expected_hash);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compiled_class_hash() {
        for (raw_artifact, raw_hashes) in [
            (
                include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_compiled.txt"),
                include_str!("../../../test-data/contracts/cairo1/artifacts/erc20.hashes.json"),
            ),
            (
                include_str!(
                    "../../../test-data/contracts/cairo1/artifacts/abi_types_compiled.txt"
                ),
                include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types.hashes.json"),
            ),
        ]
        .into_iter()
        {
            let compiled_class = serde_json::from_str::<CompiledClass>(raw_artifact).unwrap();
            let computed_hash = compiled_class.class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = FieldElement::from_hex_be(&hashes.compiled_class_hash).unwrap();

            assert_eq!(computed_hash, expected_hash);
        }
    }
}
