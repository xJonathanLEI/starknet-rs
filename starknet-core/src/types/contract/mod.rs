use alloc::{format, string::*, vec::*};

use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use serde_json_pythonic::to_string_pythonic;
use serde_with::serde_as;
use starknet_crypto::{poseidon_hash_many, PoseidonHasher};

use crate::{
    serde::unsigned_field_element::UfeHex,
    types::{EntryPointsByType, Felt, FlattenedSierraClass, SierraEntryPoint},
    utils::{
        cairo_short_string_to_felt, normalize_address, starknet_keccak, CairoShortStringToFeltError,
    },
};

/// Module containing types related to artifacts of contracts compiled with a Cairo 0.x compiler.
pub mod legacy;

/// Cairo string for `CONTRACT_CLASS_V0.1.0`
const PREFIX_CONTRACT_CLASS_V0_1_0: Felt = Felt::from_raw([
    37302452645455172,
    18446734822722598327,
    15539482671244488427,
    5800711240972404213,
]);

/// Cairo string for `COMPILED_CLASS_V1`
const PREFIX_COMPILED_CLASS_V1: Felt = Felt::from_raw([
    324306817650036332,
    18446744073709549462,
    1609463842841646376,
    2291010424822318237,
]);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ContractArtifact {
    SierraClass(SierraClass),
    CompiledClass(CompiledClass),
    LegacyClass(legacy::LegacyContractClass),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraClass {
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<Felt>,
    pub sierra_program_debug_info: SierraClassDebugInfo,
    pub contract_class_version: String,
    pub entry_points_by_type: EntryPointsByType,
    pub abi: Vec<AbiEntry>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompiledClass {
    pub prime: String,
    pub compiler_version: String,
    #[serde_as(as = "Vec<UfeHex>")]
    pub bytecode: Vec<Felt>,
    /// Represents the structure of the bytecode segments, using a nested list of segment lengths.
    /// For example, [2, [3, 4]] represents a bytecode with 2 segments, the first is a leaf of
    /// length 2 and the second is a node with 2 children of lengths 3 and 4.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bytecode_segment_lengths: Vec<IntOrList>,
    pub hints: Vec<Hint>,
    pub pythonic_hints: Option<Vec<PythonicHint>>,
    pub entry_points_by_type: CompiledClassEntrypointList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraClassDebugInfo {
    pub type_names: Vec<(u64, String)>,
    pub libfunc_names: Vec<(u64, String)>,
    pub user_func_names: Vec<(u64, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompiledClassEntrypointList {
    pub external: Vec<CompiledClassEntrypoint>,
    pub l1_handler: Vec<CompiledClassEntrypoint>,
    pub constructor: Vec<CompiledClassEntrypoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum AbiEntry {
    Function(AbiFunction),
    Event(AbiEvent),
    Struct(AbiStruct),
    Enum(AbiEnum),
    Constructor(AbiConstructor),
    Impl(AbiImpl),
    Interface(AbiInterface),
    L1Handler(AbiFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub id: u64,
    // For convenience we just treat it as an opaque JSON value here, unless a use case justifies
    // implementing the structure. (We no longer need the hints for the class hash anyways.)
    pub code: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct PythonicHint {
    pub id: u64,
    pub code: Vec<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompiledClassEntrypoint {
    #[serde_as(as = "UfeHex")]
    pub selector: Felt,
    pub offset: u64,
    pub builtins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiFunction {
    pub name: String,
    pub inputs: Vec<AbiNamedMember>,
    pub outputs: Vec<AbiOutput>,
    pub state_mutability: StateMutability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
pub enum AbiEvent {
    /// Cairo 2.x ABI event entry
    Typed(TypedAbiEvent),
    /// Cairo 1.x ABI event entry
    Untyped(UntypedAbiEvent),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TypedAbiEvent {
    Struct(AbiEventStruct),
    Enum(AbiEventEnum),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct UntypedAbiEvent {
    pub name: String,
    pub inputs: Vec<AbiNamedMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEventStruct {
    pub name: String,
    pub members: Vec<EventField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEventEnum {
    pub name: String,
    pub variants: Vec<EventField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiStruct {
    pub name: String,
    pub members: Vec<AbiNamedMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiConstructor {
    pub name: String,
    pub inputs: Vec<AbiNamedMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiImpl {
    pub name: String,
    pub interface_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiInterface {
    pub name: String,
    pub items: Vec<AbiEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiEnum {
    pub name: String,
    pub variants: Vec<AbiNamedMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiNamedMember {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct AbiOutput {
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventField {
    pub name: String,
    pub r#type: String,
    pub kind: EventFieldKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateMutability {
    External,
    View,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventFieldKind {
    Key,
    Data,
    Nested,
    Flat,
}

#[derive(Debug, Clone)]
pub enum IntOrList {
    Int(u64),
    List(Vec<IntOrList>),
}

struct IntOrListVisitor;

/// Internal structure used for post-Sierra-1.5.0 CASM hash calculation.
enum BytecodeSegmentStructure {
    BytecodeLeaf(BytecodeLeaf),
    BytecodeSegmentedNode(BytecodeSegmentedNode),
}

/// Internal structure used for post-Sierra-1.5.0 CASM hash calculation.
///
/// Represents a leaf in the bytecode segment tree.
struct BytecodeLeaf {
    // NOTE: change this to a slice?
    data: Vec<Felt>,
}

/// Internal structure used for post-Sierra-1.5.0 CASM hash calculation.
///
/// Represents an internal node in the bytecode segment tree. Each child can be loaded into memory
/// or skipped.
struct BytecodeSegmentedNode {
    segments: Vec<BytecodeSegment>,
}

/// Internal structure used for post-Sierra-1.5.0 CASM hash calculation.
///
/// Represents a child of [`BytecodeSegmentedNode`].
struct BytecodeSegment {
    segment_length: u64,
    #[allow(unused)]
    is_used: bool,
    inner_structure: alloc::boxed::Box<BytecodeSegmentStructure>,
}

mod errors {
    use alloc::string::*;
    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub enum ComputeClassHashError {
        InvalidBuiltinName,
        BytecodeSegmentLengthMismatch(BytecodeSegmentLengthMismatchError),
        InvalidBytecodeSegment(InvalidBytecodeSegmentError),
        PcOutOfRange(PcOutOfRangeError),
        Json(JsonError),
    }

    #[cfg(feature = "std")]
    #[derive(Debug)]
    pub enum CompressProgramError {
        Json(JsonError),
        Io(std::io::Error),
    }

    #[derive(Debug)]
    pub struct JsonError {
        pub(crate) message: String,
    }

    #[derive(Debug)]
    pub struct BytecodeSegmentLengthMismatchError {
        pub segment_length: usize,
        pub bytecode_length: usize,
    }

    #[derive(Debug)]
    pub struct InvalidBytecodeSegmentError {
        pub visited_pc: u64,
        pub segment_start: u64,
    }

    #[derive(Debug)]
    pub struct PcOutOfRangeError {
        pub pc: u64,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for ComputeClassHashError {}

    impl Display for ComputeClassHashError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::InvalidBuiltinName => write!(f, "invalid builtin name"),
                Self::BytecodeSegmentLengthMismatch(inner) => write!(f, "{}", inner),
                Self::InvalidBytecodeSegment(inner) => write!(f, "{}", inner),
                Self::PcOutOfRange(inner) => write!(f, "{}", inner),
                Self::Json(inner) => write!(f, "json serialization error: {}", inner),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for CompressProgramError {}

    #[cfg(feature = "std")]
    impl Display for CompressProgramError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Json(inner) => write!(f, "json serialization error: {}", inner),
                Self::Io(inner) => write!(f, "compression io error: {}", inner),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for JsonError {}

    impl Display for JsonError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", self.message)
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for BytecodeSegmentLengthMismatchError {}

    impl Display for BytecodeSegmentLengthMismatchError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "invalid bytecode segment structure length: {}, bytecode length: {}.",
                self.segment_length, self.bytecode_length,
            )
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for InvalidBytecodeSegmentError {}

    impl Display for InvalidBytecodeSegmentError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "invalid segment structure: PC {} was visited, \
                but the beginning of the segment ({}) was not",
                self.visited_pc, self.segment_start
            )
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for PcOutOfRangeError {}

    impl Display for PcOutOfRangeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "PC {} is out of range", self.pc)
        }
    }
}
pub use errors::{
    BytecodeSegmentLengthMismatchError, ComputeClassHashError, InvalidBytecodeSegmentError,
    JsonError, PcOutOfRangeError,
};

#[cfg(feature = "std")]
pub use errors::CompressProgramError;

impl SierraClass {
    pub fn class_hash(&self) -> Result<Felt, ComputeClassHashError> {
        // Technically we don't have to use the Pythonic JSON style here. Doing this just to align
        // with the official `cairo-lang` CLI.
        //
        // TODO: add an `AbiFormatter` trait and let users choose which one to use.
        let abi_str = to_string_pythonic(&self.abi).map_err(|err| {
            ComputeClassHashError::Json(JsonError {
                message: format!("{}", err),
            })
        })?;

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

    pub fn flatten(self) -> Result<FlattenedSierraClass, JsonError> {
        let abi = to_string_pythonic(&self.abi).map_err(|err| JsonError {
            message: format!("{}", err),
        })?;

        Ok(FlattenedSierraClass {
            sierra_program: self.sierra_program,
            entry_points_by_type: self.entry_points_by_type,
            abi,
            contract_class_version: self.contract_class_version,
        })
    }
}

impl FlattenedSierraClass {
    pub fn class_hash(&self) -> Felt {
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
}

impl CompiledClass {
    pub fn class_hash(&self) -> Result<Felt, ComputeClassHashError> {
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
        hasher.update(if self.bytecode_segment_lengths.is_empty() {
            // Pre-Sierra-1.5.0 compiled classes
            poseidon_hash_many(&self.bytecode)
        } else {
            // `bytecode_segment_lengths` was added since Sierra 1.5.0 and changed hash calculation.
            // This implementation here is basically a direct translation of the Python code from
            // `cairo-lang` v0.13.1. The goal was simply to have a working implementation as quickly
            // as possible. There should be some optimizations to be made here.
            // TODO: review how this can be optimized

            // NOTE: this looks extremely inefficient. Maybe just use a number for tracking instead?
            let mut rev_visited_pcs: Vec<u64> = (0..(self.bytecode.len() as u64)).rev().collect();

            let (res, total_len) = Self::create_bytecode_segment_structure_inner(
                &self.bytecode,
                &IntOrList::List(self.bytecode_segment_lengths.clone()),
                &mut rev_visited_pcs,
                &mut 0,
            )?;

            if total_len != self.bytecode.len() as u64 {
                return Err(ComputeClassHashError::BytecodeSegmentLengthMismatch(
                    BytecodeSegmentLengthMismatchError {
                        segment_length: total_len as usize,
                        bytecode_length: self.bytecode.len(),
                    },
                ));
            }
            if !rev_visited_pcs.is_empty() {
                return Err(ComputeClassHashError::PcOutOfRange(PcOutOfRangeError {
                    pc: rev_visited_pcs[rev_visited_pcs.len() - 1],
                }));
            }

            res.hash()
        });

        Ok(hasher.finalize())
    }

    fn hash_entrypoints(
        entrypoints: &[CompiledClassEntrypoint],
    ) -> Result<Felt, CairoShortStringToFeltError> {
        let mut hasher = PoseidonHasher::new();

        for entry in entrypoints {
            hasher.update(entry.selector);
            hasher.update(entry.offset.into());

            let mut builtin_hasher = PoseidonHasher::new();
            for builtin in &entry.builtins {
                builtin_hasher.update(cairo_short_string_to_felt(builtin)?)
            }

            hasher.update(builtin_hasher.finalize());
        }

        Ok(hasher.finalize())
    }

    // Direct translation of `_create_bytecode_segment_structure_inner` from `cairo-lang` v0.13.1.
    //
    // `visited_pcs` should be given in reverse order, and is consumed by the function. Returns the
    // BytecodeSegmentStructure and the total length of the processed segment.
    fn create_bytecode_segment_structure_inner(
        bytecode: &[Felt],
        bytecode_segment_lengths: &IntOrList,
        visited_pcs: &mut Vec<u64>,
        bytecode_offset: &mut u64,
    ) -> Result<(BytecodeSegmentStructure, u64), ComputeClassHashError> {
        match bytecode_segment_lengths {
            IntOrList::Int(bytecode_segment_lengths) => {
                let segment_end = *bytecode_offset + bytecode_segment_lengths;

                // Remove all the visited PCs that are in the segment.
                while !visited_pcs.is_empty()
                    && *bytecode_offset <= visited_pcs[visited_pcs.len() - 1]
                    && visited_pcs[visited_pcs.len() - 1] < segment_end
                {
                    visited_pcs.pop();
                }

                Ok((
                    BytecodeSegmentStructure::BytecodeLeaf(BytecodeLeaf {
                        data: bytecode[(*bytecode_offset as usize)..(segment_end as usize)]
                            .to_vec(),
                    }),
                    *bytecode_segment_lengths,
                ))
            }
            IntOrList::List(bytecode_segment_lengths) => {
                let mut res = Vec::new();
                let mut total_len = 0;

                for item in bytecode_segment_lengths {
                    let visited_pc_before = if !visited_pcs.is_empty() {
                        Some(visited_pcs[visited_pcs.len() - 1])
                    } else {
                        None
                    };

                    let (current_structure, item_len) =
                        Self::create_bytecode_segment_structure_inner(
                            bytecode,
                            item,
                            visited_pcs,
                            bytecode_offset,
                        )?;

                    let visited_pc_after = if !visited_pcs.is_empty() {
                        Some(visited_pcs[visited_pcs.len() - 1])
                    } else {
                        None
                    };
                    let is_used = visited_pc_after != visited_pc_before;

                    if let Some(visited_pc_before) = visited_pc_before {
                        if is_used && visited_pc_before != *bytecode_offset {
                            return Err(ComputeClassHashError::InvalidBytecodeSegment(
                                InvalidBytecodeSegmentError {
                                    visited_pc: visited_pc_before,
                                    segment_start: *bytecode_offset,
                                },
                            ));
                        }
                    }

                    res.push(BytecodeSegment {
                        segment_length: item_len,
                        is_used,
                        inner_structure: alloc::boxed::Box::new(current_structure),
                    });

                    *bytecode_offset += item_len;
                    total_len += item_len;
                }

                Ok((
                    BytecodeSegmentStructure::BytecodeSegmentedNode(BytecodeSegmentedNode {
                        segments: res,
                    }),
                    total_len,
                ))
            }
        }
    }
}

impl BytecodeSegmentStructure {
    fn hash(&self) -> Felt {
        match self {
            Self::BytecodeLeaf(inner) => inner.hash(),
            Self::BytecodeSegmentedNode(inner) => inner.hash(),
        }
    }
}

impl BytecodeLeaf {
    fn hash(&self) -> Felt {
        poseidon_hash_many(&self.data)
    }
}

impl BytecodeSegmentedNode {
    fn hash(&self) -> Felt {
        let mut hasher = PoseidonHasher::new();
        for node in &self.segments {
            hasher.update(node.segment_length.into());
            hasher.update(node.inner_structure.hash());
        }
        hasher.finalize() + Felt::ONE
    }
}

// We need to manually implement this because `raw_value` doesn't work with `untagged`:
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

// Manually implementing this so we can put `kind` in the middle:
impl Serialize for TypedAbiEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct StructRef<'a> {
            name: &'a str,
            kind: &'static str,
            members: &'a [EventField],
        }

        #[derive(Serialize)]
        struct EnumRef<'a> {
            name: &'a str,
            kind: &'static str,
            variants: &'a [EventField],
        }

        match self {
            Self::Struct(inner) => StructRef::serialize(
                &StructRef {
                    name: &inner.name,
                    kind: "struct",
                    members: &inner.members,
                },
                serializer,
            ),
            Self::Enum(inner) => EnumRef::serialize(
                &EnumRef {
                    name: &inner.name,
                    kind: "enum",
                    variants: &inner.variants,
                },
                serializer,
            ),
        }
    }
}

impl Serialize for IntOrList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Int(int) => serializer.serialize_u64(*int),
            Self::List(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for item in list {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Visitor<'de> for IntOrListVisitor {
    type Value = IntOrList;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "number or list")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntOrList::Int(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(element) = seq.next_element::<IntOrList>()? {
            items.push(element);
        }
        Ok(IntOrList::List(items))
    }
}

impl<'de> Deserialize<'de> for IntOrList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IntOrListVisitor)
    }
}

fn hash_sierra_entrypoints(entrypoints: &[SierraEntryPoint]) -> Felt {
    let mut hasher = PoseidonHasher::new();

    for entry in entrypoints {
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
        for raw_artifact in [
            include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types_sierra.txt"),
            include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_sierra.txt"),
            include_str!("../../../test-data/contracts/cairo2/artifacts/abi_types_sierra.txt"),
            include_str!("../../../test-data/contracts/cairo2/artifacts/erc20_sierra.txt"),
            include_str!("../../../test-data/contracts/cairo2.6/artifacts/erc20_sierra.txt"),
        ] {
            match serde_json::from_str::<ContractArtifact>(raw_artifact) {
                Ok(ContractArtifact::SierraClass(_)) => {}
                _ => panic!("Unexpected result"),
            }
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_compiled_class_deser() {
        for raw_artifact in [
            include_str!("../../../test-data/contracts/cairo1/artifacts/abi_types_compiled.txt"),
            include_str!("../../../test-data/contracts/cairo1/artifacts/erc20_compiled.txt"),
            include_str!("../../../test-data/contracts/cairo2/artifacts/abi_types_compiled.txt"),
            include_str!("../../../test-data/contracts/cairo2/artifacts/erc20_compiled.txt"),
            include_str!("../../../test-data/contracts/cairo2.6/artifacts/erc20_compiled.txt"),
        ] {
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
            (
                include_str!("../../../test-data/contracts/cairo2/artifacts/erc20_sierra.txt"),
                include_str!("../../../test-data/contracts/cairo2/artifacts/erc20.hashes.json"),
            ),
            (
                include_str!("../../../test-data/contracts/cairo2/artifacts/abi_types_sierra.txt"),
                include_str!("../../../test-data/contracts/cairo2/artifacts/abi_types.hashes.json"),
            ),
        ] {
            let sierra_class = serde_json::from_str::<SierraClass>(raw_artifact).unwrap();
            let computed_hash = sierra_class.class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = Felt::from_hex(&hashes.sierra_class_hash).unwrap();

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
            (
                include_str!("../../../test-data/contracts/cairo2/artifacts/erc20_compiled.txt"),
                include_str!("../../../test-data/contracts/cairo2/artifacts/erc20.hashes.json"),
            ),
            (
                include_str!(
                    "../../../test-data/contracts/cairo2/artifacts/abi_types_compiled.txt"
                ),
                include_str!("../../../test-data/contracts/cairo2/artifacts/abi_types.hashes.json"),
            ),
            (
                include_str!("../../../test-data/contracts/cairo2.6/artifacts/erc20_compiled.txt"),
                include_str!("../../../test-data/contracts/cairo2.6/artifacts/erc20.hashes.json"),
            ),
        ] {
            let compiled_class = serde_json::from_str::<CompiledClass>(raw_artifact).unwrap();
            let computed_hash = compiled_class.class_hash().unwrap();

            let hashes: ContractHashes = serde_json::from_str(raw_hashes).unwrap();
            let expected_hash = Felt::from_hex(&hashes.compiled_class_hash).unwrap();

            assert_eq!(computed_hash, expected_hash);
        }
    }
}
