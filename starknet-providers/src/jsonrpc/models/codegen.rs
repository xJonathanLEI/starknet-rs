// AUTO-GENERATED CODE. DO NOT EDIT
// To change the code generated, modify the codegen tool instead:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen

// Code generated with version:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen#ca92f96a5f50ee31a2be0eecc0d744be415e3f6e

// Code generation requested but not implemented for these types:
// - `BLOCK_ID`
// - `TXN`
// - `BROADCASTED_TXN`
// - `DECLARE_TXN`
// - `BROADCASTED_DECLARE_TXN`
// - `INVOKE_TXN`
// - `BROADCASTED_INVOKE_TXN`
// - `TXN_RECEIPT`
// - `PENDING_TXN_RECEIPT`
// - `CONTRACT_CLASS`
// - `CONTRACT_ABI_ENTRY`

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use starknet_core::{
    serde::{byte_array::base64, unsigned_field_element::UfeHex},
    types::FieldElement,
};

use super::{serde_impls::NumAsHex, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResultPageRequest {
    /// A pointer to the last element of the delivered page, use this token in a subsequent query to
    /// obtain the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    pub chunk_size: u64,
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
    /// Filter key values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub keys: Option<Vec<FieldElement>>,
}

/// A tag specifying a dynamic reference to a block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
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
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
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

/// Mempool representation of a version 1 declare transaction.
#[derive(Debug, Clone)]
pub struct BroadcastedDeclareTransactionV1 {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    /// The class to be declared
    pub contract_class: LegacyContractClass,
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
    pub contract_class: SierraContractClass,
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

/// The status of the transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// The status of the block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// The definition of a sierra Starknet contract class.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraContractClass {
    /// Sierra program bytecode
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<FieldElement>,
    pub entry_points_by_type: EntryPointsByType,
    /// String representation of the abi, uploaded by the declarer
    pub abi: String,
    /// Sierra contract class version
    pub contract_class_version: String,
}

/// The definition of a legacy (cairo 0) Starknet contract class.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct LegacyContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: LegacyEntryPointsByType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<ContractAbiEntry>>,
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
pub struct ContractEntryPoint {
    pub function_idx: u64,
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructAbiType {
    #[serde(rename = "struct")]
    Struct,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventAbiType {
    #[serde(rename = "event")]
    Event,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionAbiType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "l1_handler")]
    L1Handler,
    #[serde(rename = "constructor")]
    Constructor,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EventAbiEntry {
    pub r#type: EventAbiType,
    /// The event name
    pub name: String,
    pub keys: Vec<TypedParameter>,
    pub data: Vec<TypedParameter>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TypedParameter {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    pub r#type: String,
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

/// JSON-RPC error codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &2 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            pub contract_class: &'a LegacyContractClass,
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
            pub contract_class: LegacyContractClass,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            pub contract_class: &'a SierraContractClass,
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
            pub contract_class: SierraContractClass,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: FieldElement,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &2 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &0 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &1 {
                return Err(serde::de::Error::custom("Invalid `version` value"));
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
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: &'a Vec<FieldElement>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
            pub messages_sent: &'a Vec<MsgToL1>,
            pub events: &'a Vec<Event>,
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
                return Err(serde::de::Error::custom("Invalid `type` value"));
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
