// AUTO-GENERATED CODE. DO NOT EDIT
// To change the code generated, modify the codegen tool instead:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen

// Code generated with version:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen#84c2cdcfa5276039a8294722df871ce6c97d7cdc

// Code generation requested but not implemented for these types:
// - `BLOCK_ID`
// - `BROADCASTED_DECLARE_TXN`
// - `BROADCASTED_INVOKE_TXN`
// - `BROADCASTED_TXN`
// - `CONTRACT_ABI_ENTRY`
// - `CONTRACT_CLASS`
// - `DECLARE_TXN`
// - `INVOKE_TXN`
// - `PENDING_TXN_RECEIPT`
// - `TXN`
// - `TXN_RECEIPT`

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use starknet_core::{
    serde::{byte_array::base64, unsigned_field_element::UfeHex},
    types::FieldElement,
};

use super::{serde_impls::NumAsHex, *};

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

/// A tag specifying a dynamic reference to a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
}

/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxHashes {
    pub status: BlockStatus,
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

/// The block object.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithTxs {
    pub status: BlockStatus,
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

/// Mempool representation of a version 1 declare transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeclareTransactionV1 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The class to be declared
    pub contract_class: CompressedLegacyContractClass,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
}

/// Mempool representation of a version 2 declare transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeclareTransactionV2 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The class to be declared
    pub contract_class: FlattenedSierraClass,
    /// The hash of the compiled class
    pub compiled_class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
}

/// Mempool representation of a deploy account transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeployAccountTransaction {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
}

/// Mempool representation of a deploy transaction.
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
#[derive(Debug, Clone)]
pub struct BroadcastedDeployTransaction {
    /// The class of the contract that will be deployed
    pub contract_class: ContractClass,
    /// Version of the transaction scheme
    pub version: u64,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
}

/// Version 0 invoke transaction.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone)]
pub struct BroadcastedInvokeTransactionV0 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub contract_address: FieldElement,
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    pub calldata: Vec<FieldElement>,
}

/// Version 1 invoke transaction.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone)]
pub struct BroadcastedInvokeTransactionV1 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub sender_address: FieldElement,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<FieldElement>,
}

/// The definition of a legacy (cairo 0) Starknet contract class.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedLegacyContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: LegacyEntryPointsByType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<ContractAbiEntry>>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractEntryPoint {
    pub function_idx: u64,
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The changes in the storage of the contract
    pub storage_entries: Vec<StorageEntry>,
}

/// Declare transaction receipt.
#[derive(Debug, Clone)]
pub struct DeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Version 1 declare contract transaction.
#[derive(Debug, Clone)]
pub struct DeclareTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The hash of the declared class
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
}

/// Version 2 declare contract transaction.
#[derive(Debug, Clone)]
pub struct DeclareTransactionV2 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The hash of the declared sierra class
    pub class_hash: FieldElement,
    /// The hash of the compiled class
    pub compiled_class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: FieldElement,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone)]
pub struct DeployAccountTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
}

/// Deploy account transaction receipt.
#[derive(Debug, Clone)]
pub struct DeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

/// Deploy contract transaction.
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
#[derive(Debug, Clone)]
pub struct DeployTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The hash of the deployed contract's class
    pub class_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<FieldElement>,
}

/// Deploy transaction receipt.
#[derive(Debug, Clone)]
pub struct DeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContractItem {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The hash of the contract code
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// An event emitted as a result of transaction execution.
///
/// Event information decorated with metadata on where it was emitted.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EmittedEvent {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EntryPointsByType {
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<ContractEntryPoint>,
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<ContractEntryPoint>,
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<ContractEntryPoint>,
}

/// JSON-RPC error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
pub enum ErrorCode {
    /// Failed to write transaction
    #[error("Failed to write transaction")]
    FailedToReceiveTransaction,
    /// Contract not found
    #[error("Contract not found")]
    ContractNotFound,
    /// Invalid message selector
    #[error("Invalid message selector")]
    InvalidMessageSelector,
    /// Invalid call data
    #[error("Invalid call data")]
    InvalidCallData,
    /// Block not found
    #[error("Block not found")]
    BlockNotFound,
    /// Transaction hash not found
    #[error("Transaction hash not found")]
    TransactionHashNotFound,
    /// Invalid transaction index in a block
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndex,
    /// Class hash not found
    #[error("Class hash not found")]
    ClassHashNotFound,
    /// Requested page size is too big
    #[error("Requested page size is too big")]
    PageSizeTooBig,
    /// There are no blocks
    #[error("There are no blocks")]
    NoBlocks,
    /// The supplied continuation token is invalid or unknown
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken,
    /// Contract error
    #[error("Contract error")]
    ContractError,
    /// Invalid contract class
    #[error("Invalid contract class")]
    InvalidContractClass,
}

/// A Starknet event.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventAbiEntry {
    pub r#type: EventAbiType,
    /// The event name
    pub name: String,
    pub keys: Vec<TypedParameter>,
    pub data: Vec<TypedParameter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventAbiType {
    #[serde(rename = "event")]
    Event,
}

/// An event filter/query.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub keys: Option<Vec<FieldElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventFilterWithPage {
    #[serde(flatten)]
    pub event_filter: EventFilter,
    #[serde(flatten)]
    pub result_page_request: ResultPageRequest,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FlattenedSierraClass {
    /// Sierra program bytecode
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<FieldElement>,
    pub entry_points_by_type: EntryPointsByType,
    /// String representation of the abi, uploaded by the declarer
    pub abi: String,
    /// Sierra contract class version
    pub contract_class_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionAbiEntry {
    pub r#type: FunctionAbiType,
    /// The function name
    pub name: String,
    pub inputs: Vec<TypedParameter>,
    pub outputs: Vec<TypedParameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionAbiType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "l1_handler")]
    L1Handler,
    #[serde(rename = "constructor")]
    Constructor,
}

/// Function call information.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionCall {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
}

/// Invoke transaction receipt.
#[derive(Debug, Clone)]
pub struct InvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Version 0 invoke transaction.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone)]
pub struct InvokeTransactionV0 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub contract_address: FieldElement,
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    pub calldata: Vec<FieldElement>,
}

/// Version 1 invoke transaction.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone)]
pub struct InvokeTransactionV1 {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub sender_address: FieldElement,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<FieldElement>,
}

#[derive(Debug, Clone)]
pub struct L1HandlerTransaction {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: u64,
    /// The L1->L2 message nonce field of the sn core L1 contract at the time the transaction was
    /// sent
    pub nonce: u64,
    pub contract_address: FieldElement,
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    pub calldata: Vec<FieldElement>,
}

/// Receipt for L1 handler transaction.
#[derive(Debug, Clone)]
pub struct L1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractEntryPoint {
    /// The offset of the entry point in the program
    #[serde_as(as = "NumAsHex")]
    pub offset: u64,
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyEntryPointsByType {
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<LegacyContractEntryPoint>,
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<LegacyContractEntryPoint>,
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<LegacyContractEntryPoint>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct MsgToL1 {
    /// The target L1 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: FieldElement,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone)]
pub struct PendingDeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending deploy account transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingDeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending deploy transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingDeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    pub contract_address: FieldElement,
}

/// Pending invoke transaction receipt.
#[derive(Debug, Clone)]
pub struct PendingInvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending receipt for L1 handler transaction.
#[derive(Debug, Clone)]
pub struct PendingL1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    pub actual_fee: FieldElement,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResultPageRequest {
    /// A pointer to the last element of the delivered page, use this token in a subsequent query to
    /// obtain the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    pub chunk_size: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    pub storage_diffs: Vec<ContractStorageDiffItem>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub declared_contract_hashes: Vec<FieldElement>,
    pub deployed_contracts: Vec<DeployedContractItem>,
    pub nonces: Vec<NonceUpdate>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageEntry {
    /// The key of the changed value
    #[serde_as(as = "UfeHex")]
    pub key: FieldElement,
    /// The new value applied to the given address
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StructAbiEntry {
    pub r#type: StructAbiType,
    /// The struct name
    pub name: String,
    pub size: u64,
    pub members: Vec<StructMember>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructAbiType {
    #[serde(rename = "struct")]
    Struct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StructMember {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
    /// Offset of this property within the struct
    pub offset: u64,
}

/// An object describing the node synchronization status.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SyncStatus {
    /// The hash of the block from which the sync started
    #[serde_as(as = "UfeHex")]
    pub starting_block_hash: FieldElement,
    /// The number (height) of the block from which the sync started
    #[serde_as(as = "NumAsHex")]
    pub starting_block_num: u64,
    /// The hash of the current block being synchronized
    #[serde_as(as = "UfeHex")]
    pub current_block_hash: FieldElement,
    /// The number (height) of the current block being synchronized
    #[serde_as(as = "NumAsHex")]
    pub current_block_num: u64,
    /// The hash of the estimated highest block to be synchronized
    #[serde_as(as = "UfeHex")]
    pub highest_block_hash: FieldElement,
    /// The number (height) of the estimated highest block to be synchronized
    #[serde_as(as = "NumAsHex")]
    pub highest_block_num: u64,
}

/// The status of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    #[serde(rename = "REJECTED")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TypedParameter {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
}

/// Request for method starknet_addDeclareTransaction
#[derive(Debug, Clone)]
pub struct AddDeclareTransactionRequest {
    pub declare_transaction: BroadcastedDeclareTransaction,
}

/// Reference version of [AddDeclareTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddDeclareTransactionRequestRef<'a> {
    pub declare_transaction: &'a BroadcastedDeclareTransaction,
}

/// Request for method starknet_addDeployAccountTransaction
#[derive(Debug, Clone)]
pub struct AddDeployAccountTransactionRequest {
    /// The deploy account transaction
    pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
}

/// Reference version of [AddDeployAccountTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddDeployAccountTransactionRequestRef<'a> {
    pub deploy_account_transaction: &'a BroadcastedDeployAccountTransaction,
}

/// Request for method starknet_addDeployTransaction
#[derive(Debug, Clone)]
pub struct AddDeployTransactionRequest {
    /// The deploy transaction
    pub deploy_transaction: BroadcastedDeployTransaction,
}

/// Reference version of [AddDeployTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddDeployTransactionRequestRef<'a> {
    pub deploy_transaction: &'a BroadcastedDeployTransaction,
}

/// Request for method starknet_addInvokeTransaction
#[derive(Debug, Clone)]
pub struct AddInvokeTransactionRequest {
    /// The information needed to invoke the function (or account, for version 1 transactions)
    pub invoke_transaction: BroadcastedInvokeTransaction,
}

/// Reference version of [AddInvokeTransactionRequest].
#[derive(Debug, Clone)]
pub struct AddInvokeTransactionRequestRef<'a> {
    pub invoke_transaction: &'a BroadcastedInvokeTransaction,
}

/// Request for method starknet_blockHashAndNumber
#[derive(Debug, Clone)]
pub struct BlockHashAndNumberRequest;

/// Request for method starknet_blockNumber
#[derive(Debug, Clone)]
pub struct BlockNumberRequest;

/// Request for method starknet_call
#[derive(Debug, Clone)]
pub struct CallRequest {
    pub request: FunctionCall,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [CallRequest].
#[derive(Debug, Clone)]
pub struct CallRequestRef<'a> {
    pub request: &'a FunctionCall,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_chainId
#[derive(Debug, Clone)]
pub struct ChainIdRequest;

/// Request for method starknet_estimateFee
#[derive(Debug, Clone)]
pub struct EstimateFeeRequest {
    pub request: BroadcastedTransaction,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [EstimateFeeRequest].
#[derive(Debug, Clone)]
pub struct EstimateFeeRequestRef<'a> {
    pub request: &'a BroadcastedTransaction,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockTransactionCount
#[derive(Debug, Clone)]
pub struct GetBlockTransactionCountRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockTransactionCountRequest].
#[derive(Debug, Clone)]
pub struct GetBlockTransactionCountRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxHashes
#[derive(Debug, Clone)]
pub struct GetBlockWithTxHashesRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxHashesRequest].
#[derive(Debug, Clone)]
pub struct GetBlockWithTxHashesRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getBlockWithTxs
#[derive(Debug, Clone)]
pub struct GetBlockWithTxsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithTxsRequest].
#[derive(Debug, Clone)]
pub struct GetBlockWithTxsRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getClassAt
#[derive(Debug, Clone)]
pub struct GetClassAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class definition will be returned
    pub contract_address: FieldElement,
}

/// Reference version of [GetClassAtRequest].
#[derive(Debug, Clone)]
pub struct GetClassAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getClassHashAt
#[derive(Debug, Clone)]
pub struct GetClassHashAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class hash will be returned
    pub contract_address: FieldElement,
}

/// Reference version of [GetClassHashAtRequest].
#[derive(Debug, Clone)]
pub struct GetClassHashAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getClass
#[derive(Debug, Clone)]
pub struct GetClassRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The hash of the requested contract class
    pub class_hash: FieldElement,
}

/// Reference version of [GetClassRequest].
#[derive(Debug, Clone)]
pub struct GetClassRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub class_hash: &'a FieldElement,
}

/// Request for method starknet_getEvents
#[derive(Debug, Clone)]
pub struct GetEventsRequest {
    pub filter: EventFilterWithPage,
}

/// Reference version of [GetEventsRequest].
#[derive(Debug, Clone)]
pub struct GetEventsRequestRef<'a> {
    pub filter: &'a EventFilterWithPage,
}

/// Request for method starknet_getNonce
#[derive(Debug, Clone)]
pub struct GetNonceRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose nonce we're seeking
    pub contract_address: FieldElement,
}

/// Reference version of [GetNonceRequest].
#[derive(Debug, Clone)]
pub struct GetNonceRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a FieldElement,
}

/// Request for method starknet_getStateUpdate
#[derive(Debug, Clone)]
pub struct GetStateUpdateRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStateUpdateRequest].
#[derive(Debug, Clone)]
pub struct GetStateUpdateRequestRef<'a> {
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getStorageAt
#[derive(Debug, Clone)]
pub struct GetStorageAtRequest {
    /// The address of the contract to read from
    pub contract_address: FieldElement,
    /// The key to the storage value for the given contract
    pub key: FieldElement,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStorageAtRequest].
#[derive(Debug, Clone)]
pub struct GetStorageAtRequestRef<'a> {
    pub contract_address: &'a FieldElement,
    pub key: &'a FieldElement,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getTransactionByBlockIdAndIndex
#[derive(Debug, Clone)]
pub struct GetTransactionByBlockIdAndIndexRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    pub index: u64,
}

/// Reference version of [GetTransactionByBlockIdAndIndexRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionByBlockIdAndIndexRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub index: &'a u64,
}

/// Request for method starknet_getTransactionByHash
#[derive(Debug, Clone)]
pub struct GetTransactionByHashRequest {
    pub transaction_hash: FieldElement,
}

/// Reference version of [GetTransactionByHashRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionByHashRequestRef<'a> {
    pub transaction_hash: &'a FieldElement,
}

/// Request for method starknet_getTransactionReceipt
#[derive(Debug, Clone)]
pub struct GetTransactionReceiptRequest {
    pub transaction_hash: FieldElement,
}

/// Reference version of [GetTransactionReceiptRequest].
#[derive(Debug, Clone)]
pub struct GetTransactionReceiptRequestRef<'a> {
    pub transaction_hash: &'a FieldElement,
}

/// Request for method starknet_pendingTransactions
#[derive(Debug, Clone)]
pub struct PendingTransactionsRequest;

/// Request for method starknet_syncing
#[derive(Debug, Clone)]
pub struct SyncingRequest;

impl Serialize for BroadcastedDeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            pub contract_class: &'a CompressedLegacyContractClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
        }

        let tagged = Tagged {
            r#type: "DECLARE",
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            pub contract_class: CompressedLegacyContractClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_class: tagged.contract_class,
            sender_address: tagged.sender_address,
        })
    }
}

impl Serialize for BroadcastedDeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            pub contract_class: &'a FlattenedSierraClass,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
        }

        let tagged = Tagged {
            r#type: "DECLARE",
            max_fee: &self.max_fee,
            version: &2,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_class: &self.contract_class,
            compiled_class_hash: &self.compiled_class_hash,
            sender_address: &self.sender_address,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            pub contract_class: FlattenedSierraClass,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_class: tagged.contract_class,
            compiled_class_hash: tagged.compiled_class_hash,
            sender_address: tagged.sender_address,
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

        let tagged = Tagged {
            r#type: "DEPLOY_ACCOUNT",
            max_fee: &self.max_fee,
            version: &self.version,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
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

        Ok(Self {
            max_fee: tagged.max_fee,
            version: tagged.version,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for BroadcastedDeployTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub contract_class: &'a ContractClass,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            contract_class: &self.contract_class,
            version: &self.version,
            r#type: "DEPLOY",
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeployTransaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub contract_class: ContractClass,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<FieldElement>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            contract_class: tagged.contract_class,
            version: tagged.version,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
        })
    }
}

impl Serialize for BroadcastedInvokeTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
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
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            r#type: "INVOKE",
            max_fee: &self.max_fee,
            version: &0,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address: &self.contract_address,
            entry_point_selector: &self.entry_point_selector,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransactionV0 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address: tagged.contract_address,
            entry_point_selector: tagged.entry_point_selector,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for BroadcastedInvokeTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
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
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            r#type: "INVOKE",
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
        })
    }
}

impl Serialize for DeclareTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            r#type: "DECLARE",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub status: TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type: "DECLARE",
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
            sender_address: &self.sender_address,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            sender_address: tagged.sender_address,
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
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type: "DECLARE",
            max_fee: &self.max_fee,
            version: &2,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
            compiled_class_hash: &self.compiled_class_hash,
            sender_address: &self.sender_address,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            compiled_class_hash: tagged.compiled_class_hash,
            sender_address: tagged.sender_address,
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

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type: "DEPLOY_ACCOUNT",
            max_fee: &self.max_fee,
            version: &self.version,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
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

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            version: tagged.version,
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
            pub status: &'a TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            r#type: "DEPLOY_ACCOUNT",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            pub status: TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            class_hash: &self.class_hash,
            version: &self.version,
            r#type: "DEPLOY",
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
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
            #[serde_as(as = "UfeHex")]
            pub class_hash: FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<FieldElement>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            class_hash: tagged.class_hash,
            version: tagged.version,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
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
            pub status: &'a TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            r#type: "DEPLOY",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            pub status: TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for InvokeTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            r#type: "INVOKE",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub status: TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type: "INVOKE",
            max_fee: &self.max_fee,
            version: &0,
            signature: &self.signature,
            nonce: &self.nonce,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
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
            nonce: tagged.nonce,
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
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [FieldElement],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [FieldElement],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type: "INVOKE",
            max_fee: &self.max_fee,
            version: &1,
            signature: &self.signature,
            nonce: &self.nonce,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: FieldElement,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
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
            if tag_field != &1 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
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

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            version: &self.version,
            r#type: "L1_HANDLER",
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
            #[serde(skip_serializing_if = "Option::is_none")]
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
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: &'a FieldElement,
            pub status: &'a TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: &'a FieldElement,
            pub block_number: &'a u64,
            pub r#type: &'a str,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            status: &self.status,
            block_hash: &self.block_hash,
            block_number: &self.block_number,
            r#type: "L1_HANDLER",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub actual_fee: FieldElement,
            pub status: TransactionStatus,
            #[serde_as(as = "UfeHex")]
            pub block_hash: FieldElement,
            pub block_number: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            status: tagged.status,
            block_hash: tagged.block_hash,
            block_number: tagged.block_number,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DECLARE",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DEPLOY_ACCOUNT",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "DEPLOY",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "INVOKE",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
        }

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            r#type: "L1_HANDLER",
            messages_sent: &self.messages_sent,
            events: &self.events,
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
            #[serde(skip_serializing_if = "Option::is_none")]
            pub r#type: Option<String>,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub declare_transaction: BroadcastedDeclareTransaction,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            declare_transaction: field0.declare_transaction,
        })
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub deploy_account_transaction: BroadcastedDeployAccountTransaction,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            deploy_account_transaction: field0.deploy_account_transaction,
        })
    }
}

impl Serialize for AddDeployTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_transaction: &'a BroadcastedDeployTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_transaction: &self.deploy_transaction,
        })?;

        seq.end()
    }
}

impl<'a> Serialize for AddDeployTransactionRequestRef<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub deploy_transaction: &'a BroadcastedDeployTransaction,
        }

        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&Field0 {
            deploy_transaction: self.deploy_transaction,
        })?;

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AddDeployTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub deploy_transaction: BroadcastedDeployTransaction,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            deploy_transaction: field0.deploy_transaction,
        })
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub invoke_transaction: BroadcastedInvokeTransaction,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            invoke_transaction: field0.invoke_transaction,
        })
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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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
            pub request: &'a BroadcastedTransaction,
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
            pub request: &'a BroadcastedTransaction,
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub request: BroadcastedTransaction,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub block_id: BlockId,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            block_id: field0.block_id,
        })
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            block_id: field0.block_id,
        })
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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            block_id: field0.block_id,
        })
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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub filter: EventFilterWithPage,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            filter: field0.filter,
        })
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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub block_id: BlockId,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            block_id: field0.block_id,
        })
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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

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
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            transaction_hash: field0.transaction_hash,
        })
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
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: FieldElement,
        }

        let mut elements = Vec::<serde_json::Value>::deserialize(deserializer)?;

        let field0 = serde_json::from_value::<Field0>(
            elements
                .pop()
                .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
        )
        .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {}", err)))?;

        Ok(Self {
            transaction_hash: field0.transaction_hash,
        })
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
