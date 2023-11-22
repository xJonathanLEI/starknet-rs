// AUTO-GENERATED CODE. DO NOT EDIT
// To change the code generated, modify the codegen tool instead:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen

// Code generated with version:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen#3215ab04aee89aafa48973cdcf027219f662f49d

// Code generation requested but not implemented for these types:
// - `BLOCK_ID`
// - `BROADCASTED_DECLARE_TXN`
// - `BROADCASTED_TXN`
// - `CONTRACT_ABI_ENTRY`
// - `CONTRACT_CLASS`
// - `DECLARE_TXN`
// - `EXECUTE_INVOCATION`
// - `INVOKE_TXN`
// - `PENDING_TXN_RECEIPT`
// - `TRANSACTION_TRACE`
// - `TXN`
// - `TXN_RECEIPT`

use alloc::{format, string::String, vec::Vec};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

use crate::{
    serde::{byte_array::base64, unsigned_field_element::UfeHex},
    types::FieldElement,
};

use super::{serde_impls::NumAsHex, *};

#[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]
pub type OwnedPtr<T> = alloc::sync::Arc<T>;
#[cfg(not(all(not(no_rc), not(no_sync), target_has_atomic = "ptr")))]
pub type OwnedPtr<T> = alloc::boxed::Box<T>;

const QUERY_VERSION_OFFSET: FieldElement = FieldElement::from_mont([
    18446744073700081665,
    17407,
    18446744073709551584,
    576460752142434320,
]);

/// Block status.
///
/// The status of the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    #[serde(rename = "REJECTED")]
    Rejected,
}

/// Block tag.
///
/// A tag specifying a dynamic reference to a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
}

/// Block with transaction hashes.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxHashes {
    /// Status
    pub status: BlockStatus,
    /// Block hash
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
}

/// Block with transactions.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxs {
    /// Status
    pub status: BlockStatus,
    /// Block hash
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
}

/// Broadcasted declare transaction v1.
///
/// Mempool representation of a declare transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedDeclareTransactionV1 {
    /// The class to be declared
    pub contract_class: OwnedPtr<CompressedLegacyContractClass>,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

/// Broadcasted declare transaction v2.
///
/// Mempool representation of a declare transaction v2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedDeclareTransactionV2 {
    /// The class to be declared
    pub contract_class: OwnedPtr<FlattenedSierraClass>,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

/// Broadcasted deploy account transaction.
///
/// Mempool representation of a deploy account transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedDeployAccountTransaction {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

/// Invoke transaction v1.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedInvokeTransaction {
    /// Sender address
    pub sender_address: FieldElement,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<FieldElement>,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallType {
    #[serde(rename = "LIBRARY_CALL")]
    LibraryCall,
    #[serde(rename = "CALL")]
    Call,
}

/// Deprecated contract class.
///
/// The definition of a legacy (cairo 0) Starknet contract class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedLegacyContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: LegacyEntryPointsByType,
    /// Contract abi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<LegacyContractAbiEntry>>,
}

/// Contract storage diff item.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The changes in the storage of the contract
    pub storage_entries: Vec<StorageEntry>,
}

/// Declare transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Finality status
    pub finality_status: TransactionFinalityStatus,
    /// Block hash
    pub block_hash: FieldElement,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// The execution trace of a declare transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclareTransactionTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_invocation: Option<FunctionInvocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_transfer_invocation: Option<FunctionInvocation>,
}

/// Declare contract transaction v0.
///
/// Declare contract transaction v0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV0 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The hash of the declared class
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
}

/// Declare contract transaction v1.
///
/// Declare contract transaction v1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The hash of the declared class
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
}

/// Declare transaction v2.
///
/// Declare contract transaction v2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV2 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The hash of the declared class
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
}

/// The declared class hash and compiled class hash.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclaredClassItem {
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    /// The cairo assembly hash corresponding to the declared class
    #[serde_as(as = "UfeHex")]
    pub compiled_class_hash: FieldElement,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
}

/// Deploy account transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Finality status
    pub finality_status: TransactionFinalityStatus,
    /// Block hash
    pub block_hash: FieldElement,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

/// The execution trace of a deploy account transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployAccountTransactionTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_invocation: Option<FunctionInvocation>,
    /// The trace of the __execute__ call or constructor call, depending on the transaction type
    /// (none for declare transactions)
    pub constructor_invocation: FunctionInvocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_transfer_invocation: Option<FunctionInvocation>,
}

/// Deploy contract transaction.
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
}

/// Deploy transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Finality status
    pub finality_status: TransactionFinalityStatus,
    /// Block hash
    pub block_hash: FieldElement,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

/// Deployed contract item.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContractItem {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The hash of the contract code
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// Emitted event.
///
/// Event information decorated with metadata on where it was emitted / an event emitted as a result
/// of transaction execution.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EmittedEvent {
    /// From address
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
    /// The hash of the block in which the event was emitted
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The number of the block in which the event was emitted
    pub block_number: u64,
    /// The transaction that emitted the event
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryPointType {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "L1_HANDLER")]
    L1Handler,
    #[serde(rename = "CONSTRUCTOR")]
    Constructor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EntryPointsByType {
    /// Constructor
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<SierraEntryPoint>,
    /// External
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<SierraEntryPoint>,
    /// L1 handler
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<SierraEntryPoint>,
}

/// Event.
///
/// A Starknet event.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    /// From address
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

/// Event content.
///
/// The content of an event.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventContent {
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

/// Event filter.
///
/// An event filter/query.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventFilter {
    /// From block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    /// To block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockId>,
    /// From contract
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub address: Option<FieldElement>,
    /// The values used to filter the events
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<Vec<UfeHex>>>")]
    pub keys: Option<Vec<Vec<FieldElement>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventFilterWithPage {
    #[serde(flatten)]
    pub event_filter: EventFilter,
    #[serde(flatten)]
    pub result_page_request: ResultPageRequest,
}

/// Events chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventsChunk {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// Use this token in a subsequent query to obtain the next page. Should not appear if there are
    /// no more pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// Fee estimation.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FeeEstimate {
    /// The Ethereum gas cost of the transaction (see
    /// https://docs.starknet.io/docs/fees/fee-mechanism for more info)
    #[serde_as(as = "NumAsHex")]
    pub gas_consumed: u64,
    /// The gas price (in gwei) that was used in the cost estimation
    #[serde_as(as = "NumAsHex")]
    pub gas_price: u64,
    /// The estimated fee for the transaction (in gwei), product of gas_consumed and gas_price
    #[serde_as(as = "NumAsHex")]
    pub overall_fee: u64,
}

/// The definition of a sierra Starknet contract class.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FlattenedSierraClass {
    /// The list of sierra instructions of which the program consists
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<FieldElement>,
    /// The version of the contract class object. Currently, the Starknet os supports version 0.1.0
    pub contract_class_version: String,
    /// Entry points by type
    pub entry_points_by_type: EntryPointsByType,
    /// The class abi, as supplied by the user declaring the class
    pub abi: String,
}

/// Function call.
///
/// Function call information.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionCall {
    /// Contract address
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// Entry point selector
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionInvocation {
    /// Contract address
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// Entry point selector
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
    /// The address of the invoking contract. 0 for the root invocation
    #[serde_as(as = "UfeHex")]
    pub caller_address: FieldElement,
    /// The hash of the class being called
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    pub entry_point_type: EntryPointType,
    pub call_type: CallType,
    /// The value returned from the function invocation
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
    /// The calls made by this invocation
    pub calls: Vec<FunctionInvocation>,
    /// The events emitted in this invocation
    pub events: Vec<EventContent>,
    /// The messages sent by this invocation to L1
    pub messages: Vec<MsgToL1>,
}

/// Function state mutability type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionStateMutability {
    #[serde(rename = "view")]
    View,
}

/// Invoke transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Finality status
    pub finality_status: TransactionFinalityStatus,
    /// Block hash
    pub block_hash: FieldElement,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// The execution trace of an invoke transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InvokeTransactionTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_invocation: Option<FunctionInvocation>,
    pub execute_invocation: ExecuteInvocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_transfer_invocation: Option<FunctionInvocation>,
}

/// Invoke transaction v0.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV0 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Contract address
    pub contract_address: FieldElement,
    /// Entry point selector
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    pub calldata: Vec<FieldElement>,
}

/// Invoke transaction v1.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// Sender address
    pub sender_address: FieldElement,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<FieldElement>,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Signature
    pub signature: Vec<FieldElement>,
    /// Nonce
    pub nonce: FieldElement,
}

/// L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    /// The L1->L2 message nonce field of the sn core L1 contract at the time the transaction was
    /// sent
    pub nonce: u64,
    /// Contract address
    pub contract_address: FieldElement,
    /// Entry point selector
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    pub calldata: Vec<FieldElement>,
}

/// L1 handler transaction receipt.
///
/// Receipt for L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Finality status
    pub finality_status: TransactionFinalityStatus,
    /// Block hash
    pub block_hash: FieldElement,
    /// Block number
    pub block_number: u64,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// The execution trace of an L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1HandlerTransactionTrace {
    /// The trace of the __execute__ call or constructor call, depending on the transaction type
    /// (none for declare transactions)
    pub function_invocation: FunctionInvocation,
}

/// Deprecated cairo entry point.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractEntryPoint {
    /// The offset of the entry point in the program
    #[serde_as(as = "NumAsHex")]
    pub offset: u64,
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

/// Deprecated entry points by type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEntryPointsByType {
    /// Deprecated constructor
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<LegacyContractEntryPoint>,
    /// Deprecated external
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<LegacyContractEntryPoint>,
    /// Deprecated L1 handler
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<LegacyContractEntryPoint>,
}

/// Event abi entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEventAbiEntry {
    /// Event abi type
    pub r#type: LegacyEventAbiType,
    /// The event name
    pub name: String,
    /// Typed parameter
    pub keys: Vec<LegacyTypedParameter>,
    /// Typed parameter
    pub data: Vec<LegacyTypedParameter>,
}

/// Event abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyEventAbiType {
    #[serde(rename = "event")]
    Event,
}

/// Function abi entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyFunctionAbiEntry {
    /// Function abi type
    pub r#type: LegacyFunctionAbiType,
    /// The function name
    pub name: String,
    /// Typed parameter
    pub inputs: Vec<LegacyTypedParameter>,
    /// Typed parameter
    pub outputs: Vec<LegacyTypedParameter>,
    /// Function state mutability
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<FunctionStateMutability>,
}

/// Function abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyFunctionAbiType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "l1_handler")]
    L1Handler,
    #[serde(rename = "constructor")]
    Constructor,
}

/// Struct abi entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyStructAbiEntry {
    /// Struct abi type
    pub r#type: LegacyStructAbiType,
    /// The struct name
    pub name: String,
    /// Size
    pub size: u64,
    /// Members
    pub members: Vec<LegacyStructMember>,
}

/// Struct abi type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyStructAbiType {
    #[serde(rename = "struct")]
    Struct,
}

/// Struct member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyStructMember {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
    /// Offset of this property within the struct
    pub offset: u64,
}

/// Typed parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyTypedParameter {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
}

/// Message from L1.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct MsgFromL1 {
    /// The address of the L1 contract sending the message
    pub from_address: EthAddress,
    /// The target L2 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: FieldElement,
    /// The selector of the l1_handler in invoke in the target contract
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

/// Message to L1.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct MsgToL1 {
    /// The address of the L2 contract sending the message
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    /// The target L1 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: FieldElement,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

/// Pending block with transaction hashes.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingBlockWithTxHashes {
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
}

/// Pending block with transactions.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingBlockWithTxs {
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
}

/// Pending declare transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// Pending deploy account transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// Pending deploy transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

/// Pending invoke transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingInvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// Pending receipt for L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingL1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// Transaction execution result
    pub execution_result: ExecutionResult,
}

/// Pending state update.
///
/// Pending state update.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PendingStateUpdate {
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: FieldElement,
    /// The change in state applied in this block, given as a mapping of addresses to the new values
    /// and/or new contracts
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ReplacedClassItem {
    /// The address of the contract whose class was replaced
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The new class hash
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// Result page request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResultPageRequest {
    /// The token returned from the previous query. If no token is provided the first page is
    /// returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// Chunk size
    pub chunk_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct RevertedInvocation {
    /// The revert reason for the failed execution
    pub revert_reason: String,
}

/// Sierra entry point.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraEntryPoint {
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
    /// The index of the function in the program
    pub function_idx: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SimulatedTransaction {
    /// The transaction's trace
    pub transaction_trace: TransactionTrace,
    /// The transaction's resources and fee
    pub fee_estimation: FeeEstimate,
}

/// Flags that indicate how to simulate a given transaction. By default, the sequencer behavior is
/// replicated locally (enough funds are expected to be in the account, and fee will be deducted
/// from the balance before the simulation of the next transaction). To skip the fee charge, use the
/// skip_fee_charge flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationFlag {
    #[serde(rename = "SKIP_VALIDATE")]
    SkipValidate,
    #[serde(rename = "SKIP_FEE_CHARGE")]
    SkipFeeCharge,
}

/// JSON-RPC error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarknetError {
    /// Failed to write transaction
    FailedToReceiveTransaction,
    /// Contract not found
    ContractNotFound,
    /// Block not found
    BlockNotFound,
    /// Invalid transaction index in a block
    InvalidTransactionIndex,
    /// Class hash not found
    ClassHashNotFound,
    /// Transaction hash not found
    TransactionHashNotFound,
    /// Requested page size is too big
    PageSizeTooBig,
    /// There are no blocks
    NoBlocks,
    /// The supplied continuation token is invalid or unknown
    InvalidContinuationToken,
    /// Too many keys provided in a filter
    TooManyKeysInFilter,
    /// Contract error
    ContractError,
    /// Class already declared
    ClassAlreadyDeclared,
    /// Invalid transaction nonce
    InvalidTransactionNonce,
    /// Max fee is smaller than the minimal transaction cost (validation plus fee transfer)
    InsufficientMaxFee,
    /// Account balance is smaller than the transaction's max_fee
    InsufficientAccountBalance,
    /// Account validation failed
    ValidationFailure,
    /// Compilation failed
    CompilationFailed,
    /// Contract class size it too large
    ContractClassSizeIsTooLarge,
    /// Sender address in not an account contract
    NonAccount,
    /// A transaction with the same hash already exists in the mempool
    DuplicateTx,
    /// the compiled class hash did not match the one supplied in the transaction
    CompiledClassHashMismatch,
    /// the transaction version is not supported
    UnsupportedTxVersion,
    /// the contract class version is not supported
    UnsupportedContractClassVersion,
    /// An unexpected error occured
    UnexpectedError,
    /// No trace available for transaction
    NoTraceAvailable,
    /// Invalid transaction hash
    InvalidTransactionHash,
    /// Invalid block hash
    InvalidBlockHash,
}

#[cfg(feature = "std")]
impl std::error::Error for StarknetError {}

impl core::fmt::Display for StarknetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FailedToReceiveTransaction => write!(f, "FailedToReceiveTransaction"),
            Self::ContractNotFound => write!(f, "ContractNotFound"),
            Self::BlockNotFound => write!(f, "BlockNotFound"),
            Self::InvalidTransactionIndex => write!(f, "InvalidTransactionIndex"),
            Self::ClassHashNotFound => write!(f, "ClassHashNotFound"),
            Self::TransactionHashNotFound => write!(f, "TransactionHashNotFound"),
            Self::PageSizeTooBig => write!(f, "PageSizeTooBig"),
            Self::NoBlocks => write!(f, "NoBlocks"),
            Self::InvalidContinuationToken => write!(f, "InvalidContinuationToken"),
            Self::TooManyKeysInFilter => write!(f, "TooManyKeysInFilter"),
            Self::ContractError => write!(f, "ContractError"),
            Self::ClassAlreadyDeclared => write!(f, "ClassAlreadyDeclared"),
            Self::InvalidTransactionNonce => write!(f, "InvalidTransactionNonce"),
            Self::InsufficientMaxFee => write!(f, "InsufficientMaxFee"),
            Self::InsufficientAccountBalance => write!(f, "InsufficientAccountBalance"),
            Self::ValidationFailure => write!(f, "ValidationFailure"),
            Self::CompilationFailed => write!(f, "CompilationFailed"),
            Self::ContractClassSizeIsTooLarge => write!(f, "ContractClassSizeIsTooLarge"),
            Self::NonAccount => write!(f, "NonAccount"),
            Self::DuplicateTx => write!(f, "DuplicateTx"),
            Self::CompiledClassHashMismatch => write!(f, "CompiledClassHashMismatch"),
            Self::UnsupportedTxVersion => write!(f, "UnsupportedTxVersion"),
            Self::UnsupportedContractClassVersion => write!(f, "UnsupportedContractClassVersion"),
            Self::UnexpectedError => write!(f, "UnexpectedError"),
            Self::NoTraceAvailable => write!(f, "NoTraceAvailable"),
            Self::InvalidTransactionHash => write!(f, "InvalidTransactionHash"),
            Self::InvalidBlockHash => write!(f, "InvalidBlockHash"),
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    /// Storage diffs
    pub storage_diffs: Vec<ContractStorageDiffItem>,
    /// Deprecated declared classes
    #[serde_as(as = "Vec<UfeHex>")]
    pub deprecated_declared_classes: Vec<FieldElement>,
    /// Declared classes
    pub declared_classes: Vec<DeclaredClassItem>,
    /// Deployed contracts
    pub deployed_contracts: Vec<DeployedContractItem>,
    /// Replaced classes
    pub replaced_classes: Vec<ReplacedClassItem>,
    /// Nonces
    pub nonces: Vec<NonceUpdate>,
}

/// State update.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
    /// Block hash
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: FieldElement,
    /// The change in state applied in this block, given as a mapping of addresses to the new values
    /// and/or new contracts
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageEntry {
    /// The key of the changed value
    #[serde_as(as = "UfeHex")]
    pub key: FieldElement,
    /// The new value applied to the given address
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
}

/// Sync status.
///
/// An object describing the node synchronization status.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SyncStatus {
    /// The hash of the block from which the sync started
    #[serde_as(as = "UfeHex")]
    pub starting_block_hash: FieldElement,
    /// The number (height) of the block from which the sync started
    pub starting_block_num: u64,
    /// The hash of the current block being synchronized
    #[serde_as(as = "UfeHex")]
    pub current_block_hash: FieldElement,
    /// The number (height) of the current block being synchronized
    pub current_block_num: u64,
    /// The hash of the estimated highest block to be synchronized
    #[serde_as(as = "UfeHex")]
    pub highest_block_hash: FieldElement,
    /// The number (height) of the estimated highest block to be synchronized
    pub highest_block_num: u64,
}

/// Execution status.
///
/// The execution status of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionExecutionStatus {
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "REVERTED")]
    Reverted,
}

/// Finality status.
///
/// The finality status of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionFinalityStatus {
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
}

/// A single pair of transaction hash and corresponding trace.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTraceWithHash {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    pub trace_root: TransactionTrace,
}

/// Request for method starknet_addDeclareTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDeclareTransactionRequest {
    /// Declare transaction required to declare a new class on Starknet
    pub declare_transaction: BroadcastedDeclareTransaction,
}

/// Reference version of [AddDeclareTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDeclareTransactionRequestRef<'a> {
    pub declare_transaction: &'a BroadcastedDeclareTransaction,
}

/// Request for method starknet_addDeployAccountTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDeployAccountTransactionRequest {
    /// The deploy account transaction
    pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
}

/// Reference version of [AddDeployAccountTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDeployAccountTransactionRequestRef<'a> {
    pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
}

/// Request for method starknet_addInvokeTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddInvokeTransactionRequest {
    /// The information needed to invoke the function (or account, for version 1 transactions)
    pub invoke_transaction: BroadcastedInvokeTransaction,
}

/// Reference version of [AddInvokeTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddInvokeTransactionRequestRef<'a> {
    pub invoke_transaction: &'a BroadcastedInvokeTransaction,
}

/// Request for method starknet_blockHashAndNumber
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHashAndNumberRequest;

/// Request for method starknet_blockNumber
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockNumberRequest;

/// Request for method starknet_call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRequest {
    pub request: FunctionCall,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [CallRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRequestRef<'a> {
    pub request: &'a FunctionCall,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_chainId
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainIdRequest;

/// Request for method starknet_estimateFee
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateFeeRequest {
    pub request: Vec<BroadcastedTransaction>,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [EstimateFeeRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateFeeRequestRef<'a> {
    pub request: &'a [BroadcastedTransaction],
    pub block_id: &'a BlockId,
}

/// Request for method starknet_estimateMessageFee
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateMessageFeeRequest {
    /// the message's parameters
    pub message: MsgFromL1,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [EstimateMessageFeeRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateMessageFeeRequestRef<'a> {
    pub message: &'a MsgFromL1,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockTransactionCount
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockTransactionCountRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockTransactionCountRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockTransactionCountRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxHashes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithTxHashesRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxHashesRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithTxHashesRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithTxsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithTxsRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getClassAt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class definition will be returned
    pub contract_address: FieldElement,
}

/// Reference version of [GetClassAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getClassHashAt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassHashAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class hash will be returned
    pub contract_address: FieldElement,
}

/// Reference version of [GetClassHashAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassHashAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getClass
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The hash of the requested contract class
    pub class_hash: FieldElement,
}

/// Reference version of [GetClassRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub class_hash: &'a FieldElement,
}

/// Request for method starknet_getEvents
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetEventsRequest {
    pub filter: EventFilterWithPage,
}

/// Reference version of [GetEventsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetEventsRequestRef<'a> {
    pub filter: &'a EventFilterWithPage,
}

/// Request for method starknet_getNonce
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNonceRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose nonce we're seeking
    pub contract_address: FieldElement,
}

/// Reference version of [GetNonceRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNonceRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getStateUpdate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStateUpdateRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStateUpdateRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStateUpdateRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getStorageAt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStorageAtRequest {
    /// The address of the contract to read from
    pub contract_address: FieldElement,
    /// The key to the storage value for the given contract
    pub key: FieldElement,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStorageAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStorageAtRequestRef<'a> {
    pub contract_address: &'a FieldElement,
    pub key: &'a FieldElement,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getTransactionByBlockIdAndIndex
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionByBlockIdAndIndexRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    pub index: u64,
}

/// Reference version of [GetTransactionByBlockIdAndIndexRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionByBlockIdAndIndexRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub index: &'a u64,
}

/// Request for method starknet_getTransactionByHash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionByHashRequest {
    pub transaction_hash: FieldElement,
}

/// Reference version of [GetTransactionByHashRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionByHashRequestRef<'a> {
    pub transaction_hash: &'a FieldElement,
}

/// Request for method starknet_getTransactionReceipt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionReceiptRequest {
    pub transaction_hash: FieldElement,
}

/// Reference version of [GetTransactionReceiptRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionReceiptRequestRef<'a> {
    pub transaction_hash: &'a FieldElement,
}

/// Request for method starknet_pendingTransactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTransactionsRequest;

/// Request for method starknet_simulateTransactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulateTransactionsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
    /// The transactions to simulate
    pub transactions: Vec<BroadcastedTransaction>,
    /// describes what parts of the transaction should be executed
    pub simulation_flags: Vec<SimulationFlag>,
}

/// Reference version of [SimulateTransactionsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulateTransactionsRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub transactions: &'a [BroadcastedTransaction],
    pub simulation_flags: &'a [SimulationFlag],
}

/// Request for method starknet_syncing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncingRequest;

/// Request for method starknet_traceBlockTransactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceBlockTransactionsRequest {
    pub block_hash: FieldElement,
}

/// Reference version of [TraceBlockTransactionsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceBlockTransactionsRequestRef<'a> {
    pub block_hash: &'a FieldElement,
}

/// Request for method starknet_traceTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceTransactionRequest {
    pub transaction_hash: FieldElement,
}

/// Reference version of [TraceTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceTransactionRequestRef<'a> {
    pub transaction_hash: &'a FieldElement,
}

impl Serialize for BroadcastedDeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            pub contract_class: &'a CompressedLegacyContractClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "DECLARE";

        let version = &(if self.is_query {
            FieldElement::ONE + QUERY_VERSION_OFFSET
        } else {
            FieldElement::ONE
        });

        let tagged = Tagged {
            r#type,
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeclareTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            pub contract_class: CompressedLegacyContractClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == FieldElement::ONE {
            false
        } else if tagged.version == FieldElement::ONE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            contract_class: OwnedPtr::new(tagged.contract_class),
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            is_query,
        })
    }
}

impl Serialize for BroadcastedDeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            pub contract_class: &'a FlattenedSierraClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "DECLARE";

        let version = &(if self.is_query {
            FieldElement::TWO + QUERY_VERSION_OFFSET
        } else {
            FieldElement::TWO
        });

        let tagged = Tagged {
            r#type,
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeclareTransactionV2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            pub contract_class: FlattenedSierraClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == FieldElement::TWO {
            false
        } else if tagged.version == FieldElement::TWO + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            contract_class: OwnedPtr::new(tagged.contract_class),
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            is_query,
        })
    }
}

impl Serialize for BroadcastedDeployAccountTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &(if self.is_query {
            FieldElement::ONE + QUERY_VERSION_OFFSET
        } else {
            FieldElement::ONE
        });

        let tagged = Tagged {
            r#type,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeployAccountTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == FieldElement::ONE {
            false
        } else if tagged.version == FieldElement::ONE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
            is_query,
        })
    }
}

impl Serialize for BroadcastedInvokeTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "INVOKE";

        let version = &(if self.is_query {
            FieldElement::ONE + QUERY_VERSION_OFFSET
        } else {
            FieldElement::ONE
        });

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub version: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == FieldElement::ONE {
            false
        } else if tagged.version == FieldElement::ONE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            is_query,
        })
    }
}

impl Serialize for DeclareTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "DECLARE";

        let tagged = Tagged {
            r#type,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub finality_status: TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for DeclareTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
        }

        let r#type = "DECLARE";

        let version = &0;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV0 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
        })
    }
}

impl Serialize for DeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "DECLARE";

        let version = &1;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
        })
    }
}

impl Serialize for DeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "DECLARE";

        let version = &2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &2 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
        })
    }
}

impl Serialize for DeployAccountTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &1;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeployAccountTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_result: &self.execution_result,
            r#type,
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub finality_status: TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for DeployTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
        }

        let r#type = "DEPLOY";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            version: &self.version,
            r#type,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            version: tagged.version,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeployTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let r#type = "DEPLOY";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_result: &self.execution_result,
            r#type,
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub finality_status: TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for InvokeTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "INVOKE";

        let tagged = Tagged {
            r#type,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub finality_status: TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for InvokeTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let r#type = "INVOKE";

        let version = &0;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV0 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<FieldElement>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for InvokeTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
        }

        let r#type = "INVOKE";

        let version = &1;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
        })
    }
}

impl Serialize for L1HandlerTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub nonce: &'a u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let r#type = "L1_HANDLER";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            version: &self.version,
            r#type,
            nonce: &self.nonce,
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            pub r#type: Option<String>,
            #[serde_as(as = "NumAsHex")]
            pub nonce: u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<FieldElement>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            version: tagged.version,
            nonce: tagged.nonce,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for L1HandlerTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "L1_HANDLER";

        let tagged = Tagged {
            r#type,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub finality_status: TransactionFinalityStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for PendingDeclareTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "DECLARE";

        let finality_status = &TransactionFinalityStatus::AcceptedOnL2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type,
            messages_sent: &self.messages_sent,
            events: &self.events,
            finality_status,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeclareTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub finality_status: TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if tagged.finality_status != TransactionFinalityStatus::AcceptedOnL2 {
            return Err(serde::de::Error::custom("invalid `finality_status` value"));
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for PendingDeployAccountTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let finality_status = &TransactionFinalityStatus::AcceptedOnL2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type,
            messages_sent: &self.messages_sent,
            events: &self.events,
            finality_status,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeployAccountTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub finality_status: TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if tagged.finality_status != TransactionFinalityStatus::AcceptedOnL2 {
            return Err(serde::de::Error::custom("invalid `finality_status` value"));
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for PendingDeployTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let r#type = "DEPLOY";

        let finality_status = &TransactionFinalityStatus::AcceptedOnL2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type,
            messages_sent: &self.messages_sent,
            events: &self.events,
            finality_status,
            execution_result: &self.execution_result,
            contract_address: &self.contract_address,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingDeployTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub finality_status: TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if tagged.finality_status != TransactionFinalityStatus::AcceptedOnL2 {
            return Err(serde::de::Error::custom("invalid `finality_status` value"));
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for PendingInvokeTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "INVOKE";

        let finality_status = &TransactionFinalityStatus::AcceptedOnL2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type,
            messages_sent: &self.messages_sent,
            events: &self.events,
            finality_status,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingInvokeTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub finality_status: TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if tagged.finality_status != TransactionFinalityStatus::AcceptedOnL2 {
            return Err(serde::de::Error::custom("invalid `finality_status` value"));
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for PendingL1HandlerTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub finality_status: &'a TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "L1_HANDLER";

        let finality_status = &TransactionFinalityStatus::AcceptedOnL2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type,
            messages_sent: &self.messages_sent,
            events: &self.events,
            finality_status,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for PendingL1HandlerTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub finality_status: TransactionFinalityStatus,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if tagged.finality_status != TransactionFinalityStatus::AcceptedOnL2 {
            return Err(serde::de::Error::custom("invalid `finality_status` value"));
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for AddDeclareTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub declare_transaction: &'a BroadcastedDeclareTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            declare_transaction: &self.declare_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddDeclareTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub declare_transaction: &'a BroadcastedDeclareTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            declare_transaction: self.declare_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddDeclareTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub declare_transaction: BroadcastedDeclareTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub declare_transaction: BroadcastedDeclareTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                declare_transaction: field0.declare_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                declare_transaction: object.declare_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddDeployAccountTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_account_transaction: &self.deploy_account_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddDeployAccountTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_account_transaction: self.deploy_account_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddDeployAccountTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                deploy_account_transaction: field0.deploy_account_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                deploy_account_transaction: object.deploy_account_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddInvokeTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub invoke_transaction: &'a BroadcastedInvokeTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            invoke_transaction: &self.invoke_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddInvokeTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub invoke_transaction: &'a BroadcastedInvokeTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            invoke_transaction: self.invoke_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddInvokeTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub invoke_transaction: BroadcastedInvokeTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub invoke_transaction: BroadcastedInvokeTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                invoke_transaction: field0.invoke_transaction,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                invoke_transaction: object.invoke_transaction,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for BlockHashAndNumberRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlockHashAndNumberRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for BlockNumberRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BlockNumberRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for CallRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: &self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for CallRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for CallRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub request: FunctionCall,
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub request: FunctionCall,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                request: field0.request,
                block_id: field1.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for ChainIdRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ChainIdRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for EstimateFeeRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: &self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for EstimateFeeRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub request: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            request: self.request,
        })?;
        seq.serialize_element(&Field1 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for EstimateFeeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub request: Vec<BroadcastedTransaction>,
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub request: Vec<BroadcastedTransaction>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                request: field0.request,
                block_id: field1.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for EstimateMessageFeeRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub message: &'a MsgFromL1,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            message: &self.message,
        })?;
        seq.serialize_element(&Field1 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for EstimateMessageFeeRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub message: &'a MsgFromL1,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            message: self.message,
        })?;
        seq.serialize_element(&Field1 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for EstimateMessageFeeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub message: MsgFromL1,
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub message: MsgFromL1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                message: field0.message,
                block_id: field1.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                message: object.message,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockTransactionCountRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockTransactionCountRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockTransactionCountRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxHashesRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockWithTxHashesRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxHashesRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetBlockWithTxsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassHashAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassHashAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassHashAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            class_hash: &self.class_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetClassRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            class_hash: self.class_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetClassRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                class_hash: field1.class_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                class_hash: object.class_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetEventsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub filter: &'a EventFilterWithPage,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            filter: &self.filter,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetEventsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub filter: &'a EventFilterWithPage,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            filter: self.filter,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetEventsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub filter: EventFilterWithPage,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub filter: EventFilterWithPage,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                filter: field0.filter,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                filter: object.filter,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetNonceRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: &self.contract_address,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetNonceRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            contract_address: self.contract_address,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetNonceRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                contract_address: field1.contract_address,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                contract_address: object.contract_address,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStateUpdateRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetStateUpdateRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetStateUpdateRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStorageAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub key: &'a FieldElement,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            contract_address: &self.contract_address,
        })?;
        seq.serialize_element(&Field1 { key: &self.key })?;
        seq.serialize_element(&Field2 {
            block_id: &self.block_id,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetStorageAtRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub key: &'a FieldElement,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub block_id: &'a BlockId,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            contract_address: self.contract_address,
        })?;
        seq.serialize_element(&Field1 { key: self.key })?;
        seq.serialize_element(&Field2 {
            block_id: self.block_id,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetStorageAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub key: FieldElement,
            pub block_id: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub contract_address: FieldElement,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub key: FieldElement,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub block_id: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                contract_address: field0.contract_address,
                key: field1.key,
                block_id: field2.block_id,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                contract_address: object.contract_address,
                key: object.key,
                block_id: object.block_id,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByBlockIdAndIndexRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub index: &'a u64,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 { index: &self.index })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionByBlockIdAndIndexRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub index: &'a u64,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 { index: self.index })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionByBlockIdAndIndexRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            pub index: u64,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub index: u64,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                index: field1.index,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                index: object.index,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByHashRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: &self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionByHashRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionByHashRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                transaction_hash: field0.transaction_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionReceiptRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: &self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for GetTransactionReceiptRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for GetTransactionReceiptRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                transaction_hash: field0.transaction_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for PendingTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for PendingTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for SimulateTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub transactions: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub simulation_flags: &'a [SimulationFlag],
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: &self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            transactions: &self.transactions,
        })?;
        seq.serialize_element(&Field2 {
            simulation_flags: &self.simulation_flags,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for SimulateTransactionsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub block_id: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub transactions: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub simulation_flags: &'a [SimulationFlag],
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_id: self.block_id,
        })?;
        seq.serialize_element(&Field1 {
            transactions: self.transactions,
        })?;
        seq.serialize_element(&Field2 {
            simulation_flags: self.simulation_flags,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for SimulateTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            pub block_id: BlockId,
            pub transactions: Vec<BroadcastedTransaction>,
            pub simulation_flags: Vec<SimulationFlag>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub transactions: Vec<BroadcastedTransaction>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub simulation_flags: Vec<SimulationFlag>,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_id: field0.block_id,
                transactions: field1.transactions,
                simulation_flags: field2.simulation_flags,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id,
                transactions: object.transactions,
                simulation_flags: object.simulation_flags,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SyncingRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SyncingRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for TraceBlockTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_hash: &self.block_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for TraceBlockTransactionsRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            block_hash: self.block_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for TraceBlockTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                block_hash: field0.block_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_hash: object.block_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for TraceTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: &self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for TraceTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            transaction_hash: self.transaction_hash,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for TraceTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct AsObject {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

            Ok(Self {
                transaction_hash: field0.transaction_hash,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}
