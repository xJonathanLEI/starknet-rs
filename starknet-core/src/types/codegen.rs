// AUTO-GENERATED CODE. DO NOT EDIT
// To change the code generated, modify the codegen tool instead:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen

// Code generated with version:
//     https://github.com/xJonathanLEI/starknet-jsonrpc-codegen#a8e2da3746497b437d551b1982b2ed8a05f43d99

// These types are ignored from code generation. Implement them manually:
// - `RECEIPT_BLOCK`
// - `TXN_STATUS_RESULT`
// - `SUBSCRIPTION_BLOCK_TAG`

// Code generation requested but not implemented for these types:
// - `BLOCK_ID`
// - `BROADCASTED_TXN`
// - `CONFIRMED_BLOCK_ID`
// - `CONTRACT_ABI_ENTRY`
// - `CONTRACT_CLASS`
// - `CONTRACT_EXECUTION_ERROR`
// - `DECLARE_TXN`
// - `DECLARE_TXN_CONTENT`
// - `DEPLOY_ACCOUNT_TXN`
// - `DEPLOY_ACCOUNT_TXN_CONTENT`
// - `EXECUTE_INVOCATION`
// - `INVOKE_TXN`
// - `INVOKE_TXN_CONTENT`
// - `MERKLE_NODE`
// - `TRANSACTION_TRACE`
// - `TXN`
// - `TXN_CONTENT`
// - `TXN_RECEIPT`

#![allow(missing_docs)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_const_for_fn)]

use alloc::{format, string::*, vec::*};

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

use crate::serde::byte_array::base64;

use super::{
    serde_impls::{MerkleNodeMap, NumAsHex, OwnedContractExecutionError},
    *,
};

#[cfg(target_has_atomic = "ptr")]
pub type OwnedPtr<T> = alloc::sync::Arc<T>;
#[cfg(not(target_has_atomic = "ptr"))]
pub type OwnedPtr<T> = alloc::boxed::Box<T>;

#[cfg(feature = "std")]
type RandomState = std::hash::RandomState;
#[cfg(not(feature = "std"))]
type RandomState = foldhash::fast::RandomState;

const QUERY_VERSION_OFFSET: Felt = Felt::from_raw([
    576460752142434320,
    18446744073709551584,
    17407,
    18446744073700081665,
]);

pub type BroadcastedInvokeTransaction = BroadcastedInvokeTransactionV3;
pub type BroadcastedDeployAccountTransaction = BroadcastedDeployAccountTransactionV3;
pub type BroadcastedDeclareTransaction = BroadcastedDeclareTransactionV3;

/// An internal node whose both children are non-zero.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BinaryNode {
    /// The hash of the left child
    #[serde_as(as = "UfeHex")]
    pub left: Felt,
    /// The hash of the right child
    #[serde_as(as = "UfeHex")]
    pub right: Felt,
}

/// Block header.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockHeader {
    /// Block hash
    #[serde_as(as = "UfeHex")]
    pub block_hash: Felt,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: Felt,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: Felt,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
}

/// Block status.
///
/// The status of the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStatus {
    #[serde(rename = "PRE_CONFIRMED")]
    PreConfirmed,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
}

/// Block tag.
///
/// A tag specifying a dynamic reference to a block. Tag `l1_accepted` refers to the latest Starknet
/// block which was included in a state update on L1 and finalized by the consensus on L1. Tag
/// `latest` refers to the latest Starknet block finalized by the consensus on L2. Tag
/// `pre_confirmed` refers to the block which is currently being built by the block proposer in
/// height `latest` + 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "l1_accepted")]
    L1Accepted,
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pre_confirmed")]
    PreConfirmed,
}

/// Block with transactions and receipts.
///
/// The block object.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockWithReceipts {
    /// Status
    pub status: BlockStatus,
    /// Block hash
    #[serde_as(as = "UfeHex")]
    pub block_hash: Felt,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: Felt,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: Felt,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
    /// The transactions in this block
    pub transactions: Vec<TransactionWithReceipt>,
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
    pub block_hash: Felt,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: Felt,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: Felt,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<Felt>,
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
    pub block_hash: Felt,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: Felt,
    /// The block number (its height)
    pub block_number: u64,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: Felt,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
}

/// Broadcasted declare transaction v3.
///
/// Broadcasted declare contract transaction v3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedDeclareTransactionV3 {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The class to be declared
    pub contract_class: OwnedPtr<FlattenedSierraClass>,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedDeployAccountTransactionV3 {
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastedInvokeTransactionV3 {
    /// Sender address
    pub sender_address: Felt,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt>,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
    /// If set to `true`, uses a query-only transaction version that's invalid for execution
    pub is_query: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallType {
    #[serde(rename = "LIBRARY_CALL")]
    LibraryCall,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "DELEGATE")]
    Delegate,
}

/// Deprecated contract class.
///
/// The definition of a Starknet contract class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CompressedLegacyContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    /// Deprecated entry points by type
    pub entry_points_by_type: LegacyEntryPointsByType,
    /// Contract abi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<LegacyContractAbiEntry>>,
}

/// More data about the execution failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractErrorData {
    /// The execution trace up to the point of failure
    pub revert_error: ContractExecutionError,
}

/// The nonce and class hash for each requested contract address, in the order in which they appear
/// in the request. These values are needed to construct the associated leaf node.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractLeafData {
    #[serde_as(as = "UfeHex")]
    pub nonce: Felt,
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub storage_root: Option<Felt>,
}

/// Contract storage diff item.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "UfeHex")]
    pub address: Felt,
    /// The changes in the storage of the contract
    pub storage_entries: Vec<StorageEntry>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractStorageKeys {
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    #[serde_as(as = "Vec<UfeHex>")]
    pub storage_keys: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractsProof {
    /// The nodes in the union of the paths from the contracts tree root to the requested leaves
    #[serde_as(as = "MerkleNodeMap")]
    pub nodes: IndexMap<Felt, MerkleNode, RandomState>,
    pub contract_leaves_data: Vec<ContractLeafData>,
}

/// Da mode.
///
/// Specifies a storage domain in Starknet. Each domain has different guarantees regarding
/// availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataAvailabilityMode {
    #[serde(rename = "L1")]
    L1,
    #[serde(rename = "L2")]
    L2,
}

/// Declare transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt,
    /// The fee that was charged by the sequencer
    pub actual_fee: FeePayment,
    /// Finality status of the tx
    pub finality_status: TransactionFinalityStatus,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The resources consumed by the transaction
    pub execution_resources: ExecutionResources,
    pub execution_result: ExecutionResult,
}

/// The execution trace of a declare transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionTrace {
    pub validate_invocation: Option<FunctionInvocation>,
    pub fee_transfer_invocation: Option<FunctionInvocation>,
    /// The state diffs induced by the transaction
    pub state_diff: Option<StateDiff>,
    /// The resources consumed by the transaction, includes both computation and data
    pub execution_resources: ExecutionResources,
}

/// Declare contract transaction v0.
///
/// Declare contract transaction v0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV0 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare txn v0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV0Content {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare contract transaction v1.
///
/// Declare contract transaction v1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV1 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare txn v1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV1Content {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare transaction v2.
///
/// Declare contract transaction v2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV2 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare txn v2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV2Content {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
}

/// Declare transaction v3.
///
/// Declare contract transaction v3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV3 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// Declare txn v3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareTransactionV3Content {
    /// The address of the account contract sending the declaration transaction
    pub sender_address: Felt,
    /// The hash of the cairo assembly resulting from the sierra compilation
    pub compiled_class_hash: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The hash of the declared class
    pub class_hash: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// New classes.
///
/// The declared class hash and compiled class hash.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclaredClassItem {
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    /// The cairo assembly hash corresponding to the declared class
    #[serde_as(as = "UfeHex")]
    pub compiled_class_hash: Felt,
}

/// Deploy account transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt,
    /// The fee that was charged by the sequencer
    pub actual_fee: FeePayment,
    /// Finality status of the tx
    pub finality_status: TransactionFinalityStatus,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The resources consumed by the transaction
    pub execution_resources: ExecutionResources,
    pub execution_result: ExecutionResult,
    /// The address of the deployed contract
    pub contract_address: Felt,
}

/// The execution trace of a deploy account transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionTrace {
    pub validate_invocation: Option<FunctionInvocation>,
    /// The trace of the constructor call
    pub constructor_invocation: FunctionInvocation,
    pub fee_transfer_invocation: Option<FunctionInvocation>,
    /// The state diffs induced by the transaction
    pub state_diff: Option<StateDiff>,
    /// The resources consumed by the transaction, includes both computation and data
    pub execution_resources: ExecutionResources,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionV1 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionV1Content {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionV3 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// Deploy account transaction.
///
/// Deploys an account contract, charges fee from the pre-funded account addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployAccountTransactionV3Content {
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// Deploy contract transaction.
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransaction {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// Version of the transaction scheme
    pub version: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
}

/// Deploy txn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransactionContent {
    /// Version of the transaction scheme
    pub version: Felt,
    /// The salt for the address of the deployed contract
    pub contract_address_salt: Felt,
    /// The parameters passed to the constructor
    pub constructor_calldata: Vec<Felt>,
    /// The hash of the deployed contract's class
    pub class_hash: Felt,
}

/// Deploy transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt,
    /// The fee that was charged by the sequencer
    pub actual_fee: FeePayment,
    /// Finality status of the tx
    pub finality_status: TransactionFinalityStatus,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The resources consumed by the transaction
    pub execution_resources: ExecutionResources,
    pub execution_result: ExecutionResult,
    /// The address of the deployed contract
    pub contract_address: Felt,
}

/// Deployed contract item.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContractItem {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub address: Felt,
    /// The hash of the contract code
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
}

/// Represents a path to the highest non-zero descendant node.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EdgeNode {
    /// An unsigned integer whose binary representation represents the path from the current node to
    /// its highest non-zero descendant (bounded by 2^251)
    #[serde_as(as = "UfeHex")]
    pub path: Felt,
    /// The length of the path (bounded by 251)
    pub length: u64,
    /// The hash of the unique non-zero maximal-height descendant node
    #[serde_as(as = "UfeHex")]
    pub child: Felt,
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
    pub from_address: Felt,
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<Felt>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<Felt>,
    /// The hash of the block in which the event was emitted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub block_hash: Option<Felt>,
    /// The number of the block in which the event was emitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
    /// The transaction that emitted the event
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct EmittedEventWithFinality {
    #[serde(flatten)]
    pub emitted_event: EmittedEvent,
    /// Finality status of the transaction
    pub finality_status: TransactionFinalityStatus,
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

/// Entry points by type.
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
    pub from_address: Felt,
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<Felt>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<Felt>,
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
    pub address: Option<Felt>,
    /// The keys to filter over
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<Vec<UfeHex>>>")]
    pub keys: Option<Vec<Vec<Felt>>>,
}

/// Events request.
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

/// Execution resources.
///
/// The resources consumed by the transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ExecutionResources {
    /// L1 gas consumed by this transaction, used for L2-->L1 messages and state updates if blobs
    /// are not used
    pub l1_gas: u64,
    /// Data gas consumed by this transaction, 0 if blobs are not used
    pub l1_data_gas: u64,
    /// L2 gas consumed by this transaction, used for computation and calldata
    pub l2_gas: u64,
}

/// Fee estimation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeEstimate {
    /// The Ethereum gas consumption of the transaction, charged for L1->L2 messages and, depending
    /// on the block's da_mode, state diffs
    pub l1_gas_consumed: u64,
    /// The gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l1_gas_price: u128,
    /// The L2 gas consumption of the transaction
    pub l2_gas_consumed: u64,
    /// The L2 gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l2_gas_price: u128,
    /// The Ethereum data gas consumption of the transaction
    pub l1_data_gas_consumed: u64,
    /// The data gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l1_data_gas_price: u128,
    /// The estimated fee for the transaction (in wei or fri, depending on the tx version), equals
    /// to l1_gas_consumed*l1_gas_price + l1_data_gas_consumed*l1_data_gas_price +
    /// l2_gas_consumed*l2_gas_price
    pub overall_fee: u128,
}

/// Fee payment.
///
/// Fee payment info as it appears in receipts.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FeePayment {
    /// Amount paid
    #[serde_as(as = "UfeHex")]
    pub amount: Felt,
    /// Units in which the fee is given
    pub unit: PriceUnit,
}

/// The definition of a sierra Starknet contract class.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FlattenedSierraClass {
    /// The list of sierra instructions of which the program consists
    #[serde_as(as = "Vec<UfeHex>")]
    pub sierra_program: Vec<Felt>,
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
    pub contract_address: Felt,
    /// Entry point selector
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionInvocation {
    /// Contract address
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    /// Entry point selector
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<Felt>,
    /// The address of the invoking contract. 0 for the root invocation
    #[serde_as(as = "UfeHex")]
    pub caller_address: Felt,
    /// The hash of the class being called
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    pub entry_point_type: EntryPointType,
    pub call_type: CallType,
    /// The value returned from the function invocation
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<Felt>,
    /// The calls made by this invocation
    pub calls: Vec<FunctionInvocation>,
    /// The events emitted in this invocation
    pub events: Vec<OrderedEvent>,
    /// The messages sent by this invocation to L1
    pub messages: Vec<OrderedMessage>,
    /// Resources consumed by the call tree rooted at this given call (including the root)
    pub execution_resources: InnerCallExecutionResources,
    /// True if this inner call panicked
    pub is_reverted: bool,
}

/// Function state mutability type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionStateMutability {
    #[serde(rename = "view")]
    View,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct GlobalRoots {
    #[serde_as(as = "UfeHex")]
    pub contracts_tree_root: Felt,
    #[serde_as(as = "UfeHex")]
    pub classes_tree_root: Felt,
    /// The associated block hash (needed in case the caller used a block tag for the block_id
    /// parameter)
    #[serde_as(as = "UfeHex")]
    pub block_hash: Felt,
}

/// Execution resources.
///
/// The resources consumed by an inner call (does not account for state diffs since data is squashed
/// across the transaction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InnerCallExecutionResources {
    /// L1 gas consumed by this transaction, used for L2-->L1 messages and state updates if blobs
    /// are not used
    pub l1_gas: u64,
    /// L2 gas consumed by this transaction, used for computation and calldata
    pub l2_gas: u64,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InnerContractExecutionError {
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub selector: Felt,
    #[serde_as(as = "OwnedContractExecutionError")]
    pub error: OwnedPtr<ContractExecutionError>,
}

/// Invoke transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionReceipt {
    /// The hash identifying the transaction
    pub transaction_hash: Felt,
    /// The fee that was charged by the sequencer
    pub actual_fee: FeePayment,
    /// Finality status of the tx
    pub finality_status: TransactionFinalityStatus,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The resources consumed by the transaction
    pub execution_resources: ExecutionResources,
    pub execution_result: ExecutionResult,
}

/// The execution trace of an invoke transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionTrace {
    pub validate_invocation: Option<FunctionInvocation>,
    pub execute_invocation: ExecuteInvocation,
    pub fee_transfer_invocation: Option<FunctionInvocation>,
    /// The state diffs induced by the transaction
    pub state_diff: Option<StateDiff>,
    /// The resources consumed by the transaction, includes both computation and data
    pub execution_resources: ExecutionResources,
}

/// Invoke transaction v0.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV0 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Contract address
    pub contract_address: Felt,
    /// Entry point selector
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    pub calldata: Vec<Felt>,
}

/// Invoke transaction v0.
///
/// Invokes a specific function in the desired contract (not necessarily an account).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV0Content {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Contract address
    pub contract_address: Felt,
    /// Entry point selector
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    pub calldata: Vec<Felt>,
}

/// Invoke transaction v1.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV1 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// Sender address
    pub sender_address: Felt,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt>,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
}

/// Invoke transaction v1.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV1Content {
    /// Sender address
    pub sender_address: Felt,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt>,
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: Felt,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
}

/// Invoke transaction v3.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV3 {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// Sender address
    pub sender_address: Felt,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt>,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// Invoke transaction v3.
///
/// Initiates a transaction from a given account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeTransactionV3Content {
    /// Sender address
    pub sender_address: Felt,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    pub calldata: Vec<Felt>,
    /// Signature
    pub signature: Vec<Felt>,
    /// Nonce
    pub nonce: Felt,
    /// Resource bounds for the transaction execution
    pub resource_bounds: ResourceBoundsMapping,
    /// The tip for the transaction
    pub tip: u64,
    /// Data needed to allow the paymaster to pay for the transaction in native tokens
    pub paymaster_data: Vec<Felt>,
    /// Data needed to deploy the account contract from which this tx will be initiated
    pub account_deployment_data: Vec<Felt>,
    /// The storage domain of the account's nonce (an account has a nonce per da mode)
    pub nonce_data_availability_mode: DataAvailabilityMode,
    /// The storage domain of the account's balance from which fee will be charged
    pub fee_data_availability_mode: DataAvailabilityMode,
}

/// L1 da mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum L1DataAvailabilityMode {
    #[serde(rename = "BLOB")]
    Blob,
    #[serde(rename = "CALLDATA")]
    Calldata,
}

/// L1 handler transaction.
///
/// A call to an l1_handler on an L2 contract induced by a message from L1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransaction {
    /// Transaction hash
    pub transaction_hash: Felt,
    /// Version of the transaction scheme
    pub version: Felt,
    /// The L1->L2 message nonce field of the sn core L1 contract at the time the transaction was
    /// sent
    pub nonce: u64,
    /// Contract address
    pub contract_address: Felt,
    /// Entry point selector
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    pub calldata: Vec<Felt>,
}

/// L1 handler transaction.
///
/// A call to an l1_handler on an L2 contract induced by a message from L1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransactionContent {
    /// Version of the transaction scheme
    pub version: Felt,
    /// The L1->L2 message nonce field of the sn core L1 contract at the time the transaction was
    /// sent
    pub nonce: u64,
    /// Contract address
    pub contract_address: Felt,
    /// Entry point selector
    pub entry_point_selector: Felt,
    /// The parameters passed to the function
    pub calldata: Vec<Felt>,
}

/// L1 handler transaction receipt.
///
/// Receipt for L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransactionReceipt {
    /// The message hash as it appears on the L1 core contract
    pub message_hash: Hash256,
    /// The hash identifying the transaction
    pub transaction_hash: Felt,
    /// The fee that was charged by the sequencer
    pub actual_fee: FeePayment,
    /// Finality status of the tx
    pub finality_status: TransactionFinalityStatus,
    /// Messages sent
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The resources consumed by the transaction
    pub execution_resources: ExecutionResources,
    pub execution_result: ExecutionResult,
}

/// The execution trace of an L1 handler transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1HandlerTransactionTrace {
    /// The trace of the L1 handler call
    pub function_invocation: ExecuteInvocation,
    /// The state diffs induced by the transaction
    pub state_diff: Option<StateDiff>,
    /// The resources consumed by the transaction, includes both computation and data
    pub execution_resources: ExecutionResources,
}

/// Layer-2 finality status.
///
/// The layer-2-only finality status of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum L2TransactionFinalityStatus {
    #[serde(rename = "PRE_CONFIRMED")]
    PreConfirmed,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
}

/// Layer-2 transaction status.
///
/// The layer-2-only finality status of the transaction, including the case the txn is still in the
/// mempool or failed validation during the block construction phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum L2TransactionStatus {
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "CANDIDATE")]
    Candidate,
    #[serde(rename = "PRE_CONFIRMED")]
    PreConfirmed,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
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
    pub selector: Felt,
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

/// Message fee estimation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageFeeEstimate {
    /// The Ethereum gas consumption of the transaction, charged for L1->L2 messages and, depending
    /// on the block's da_mode, state diffs
    pub l1_gas_consumed: u64,
    /// The gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l1_gas_price: u128,
    /// The L2 gas consumption of the transaction
    pub l2_gas_consumed: u64,
    /// The L2 gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l2_gas_price: u128,
    /// The Ethereum data gas consumption of the transaction
    pub l1_data_gas_consumed: u64,
    /// The data gas price (in wei or fri, depending on the tx version) that was used in the cost
    /// estimation
    pub l1_data_gas_price: u128,
    /// The estimated fee for the transaction (in wei or fri, depending on the tx version), equals
    /// to l1_gas_consumed*l1_gas_price + l1_data_gas_consumed*l1_data_gas_price +
    /// l2_gas_consumed*l2_gas_price
    pub overall_fee: u128,
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
    pub to_address: Felt,
    /// The selector of the l1_handler in invoke in the target contract
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: Felt,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<Felt>,
}

/// Message to L1.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct MsgToL1 {
    /// The address of the L2 contract sending the message
    #[serde_as(as = "UfeHex")]
    pub from_address: Felt,
    /// The target L1 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: Felt,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<Felt>,
}

/// New transaction status.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NewTransactionStatus {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    pub status: TransactionStatus,
}

/// Extra information on why trace is not available. Either it wasn't executed yet (received), or
/// the transaction failed (rejected).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NoTraceAvailableErrorData {
    pub status: SequencerTransactionStatus,
}

/// Nonce update.
///
/// The updated nonce per contract address.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "UfeHex")]
    pub nonce: Felt,
}

/// Orderedevent.
///
/// An event alongside its order within the transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct OrderedEvent {
    /// The order of the event within the transaction
    pub order: u64,
    /// Keys
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<Felt>,
    /// Data
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<Felt>,
}

/// Orderedmessage.
///
/// A message alongside its order within the transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct OrderedMessage {
    /// The order of the message within the transaction
    pub order: u64,
    /// The address of the L2 contract sending the message
    #[serde_as(as = "UfeHex")]
    pub from_address: Felt,
    /// The target L1 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: Felt,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<Felt>,
}

/// Pre-confirmed block with transactions and receipts.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PreConfirmedBlockWithReceipts {
    /// The transactions in this block
    pub transactions: Vec<TransactionWithReceipt>,
    /// The block number of the block that the proposer is currently building. Note that this is a
    /// local view of the node, whose accuracy depends on its polling interval length.
    pub block_number: u64,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
}

/// Pre-confirmed block with transaction hashes.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PreConfirmedBlockWithTxHashes {
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<Felt>,
    /// The block number of the block that the proposer is currently building. Note that this is a
    /// local view of the node, whose accuracy depends on its polling interval length.
    pub block_number: u64,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
}

/// Pre-confirmed block with transactions.
///
/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PreConfirmedBlockWithTxs {
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
    /// The block number of the block that the proposer is currently building. Note that this is a
    /// local view of the node, whose accuracy depends on its polling interval length.
    pub block_number: u64,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The Starknet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: Felt,
    /// The price of L1 gas in the block
    pub l1_gas_price: ResourcePrice,
    /// The price of L2 gas in the block
    pub l2_gas_price: ResourcePrice,
    /// The price of L1 data gas in the block
    pub l1_data_gas_price: ResourcePrice,
    /// Specifies whether the data of this block is published via blob data or calldata
    pub l1_da_mode: L1DataAvailabilityMode,
    /// Semver of the current Starknet protocol
    pub starknet_version: String,
}

/// Pre-confirmed state update.
///
/// Pre-confirmed state update.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct PreConfirmedStateUpdate {
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: Felt,
    /// State diff
    pub state_diff: StateDiff,
}

/// Price unit.
///
/// Units in which the fee is given.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceUnit {
    #[serde(rename = "WEI")]
    Wei,
    #[serde(rename = "FRI")]
    Fri,
}

/// Price unit fri.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceUnitFri {
    #[serde(rename = "FRI")]
    Fri,
}

/// Price unit wei.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PriceUnitWei {
    #[serde(rename = "WEI")]
    Wei,
}

/// Reorg data.
///
/// Data about reorganized blocks, starting and ending block number and hash.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ReorgData {
    /// Hash of the first known block of the orphaned chain
    #[serde_as(as = "UfeHex")]
    pub starting_block_hash: Felt,
    /// Number of the first known block of the orphaned chain
    pub starting_block_number: u64,
    /// The last known block of the orphaned chain
    #[serde_as(as = "UfeHex")]
    pub ending_block_hash: Felt,
    /// Number of the last known block of the orphaned chain
    pub ending_block_number: u64,
}

/// Replaced class.
///
/// The list of contracts whose class was replaced.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ReplacedClassItem {
    /// The address of the contract whose class was replaced
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    /// The new class hash
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResourceBounds {
    /// The max amount of the resource that can be used in the tx
    #[serde_as(as = "NumAsHex")]
    pub max_amount: u64,
    /// The max price per unit of this resource for this tx
    #[serde_as(as = "NumAsHex")]
    pub max_price_per_unit: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResourceBoundsMapping {
    /// The max amount and max price per unit of L1 gas used in this tx
    pub l1_gas: ResourceBounds,
    /// The max amount and max price per unit of L1 blob gas used in this tx
    pub l1_data_gas: ResourceBounds,
    /// The max amount and max price per unit of L2 gas used in this tx
    pub l2_gas: ResourceBounds,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResourcePrice {
    /// The price of one unit of the given resource, denominated in fri (10^-18 strk)
    #[serde_as(as = "UfeHex")]
    pub price_in_fri: Felt,
    /// The price of one unit of the given resource, denominated in wei
    #[serde_as(as = "UfeHex")]
    pub price_in_wei: Felt,
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
    /// The revert reason for the failed invocation
    pub revert_reason: String,
}

/// Transaction status.
///
/// The finality status of the transaction, including the case the txn is still in the mempool or
/// failed validation during the block construction phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequencerTransactionStatus {
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "CANDIDATE")]
    Candidate,
    #[serde(rename = "PRE_CONFIRMED")]
    PreConfirmed,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
}

/// Sierra entry point.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SierraEntryPoint {
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "UfeHex")]
    pub selector: Felt,
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

/// Flags that indicate how to simulate a given transaction. By default, the sequencer behavior is
/// replicated locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationFlagForEstimateFee {
    #[serde(rename = "SKIP_VALIDATE")]
    SkipValidate,
}

/// JSON-RPC error codes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarknetError {
    /// Failed to write transaction
    FailedToReceiveTransaction,
    /// Contract not found
    ContractNotFound,
    /// Requested entrypoint does not exist in the contract
    EntrypointNotFound,
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
    ContractError(ContractErrorData),
    /// Transaction execution error
    TransactionExecutionError(TransactionExecutionErrorData),
    /// the node doesn't support storage proofs for blocks that are too far in the past
    StorageProofNotSupported,
    /// Class already declared
    ClassAlreadyDeclared,
    /// Invalid transaction nonce
    InvalidTransactionNonce(String),
    /// The transaction's resources don't cover validation or the minimal transaction fee
    InsufficientResourcesForValidate,
    /// Account balance is smaller than the transaction's maximal fee (calculated as the sum of each
    /// resource's limit x max price)
    InsufficientAccountBalance,
    /// Account validation failed
    ValidationFailure(String),
    /// Compilation failed
    CompilationFailed(String),
    /// Contract class size is too large
    ContractClassSizeIsTooLarge,
    /// Sender address is not an account contract
    NonAccount,
    /// A transaction with the same hash already exists in the mempool
    DuplicateTx,
    /// the compiled class hash did not match the one supplied in the transaction
    CompiledClassHashMismatch,
    /// the transaction version is not supported
    UnsupportedTxVersion,
    /// the contract class version is not supported
    UnsupportedContractClassVersion,
    /// An unexpected error occurred
    UnexpectedError(String),
    /// Replacement transaction is underpriced
    ReplacementTransactionUnderpriced,
    /// Transaction fee below minimum
    FeeBelowMinimum,
    /// No trace available for transaction
    NoTraceAvailable(NoTraceAvailableErrorData),
    /// Invalid subscription id
    InvalidSubscriptionId,
    /// Too many addresses in filter sender_address filter
    TooManyAddressesInFilter,
    /// Cannot go back more than 1024 blocks
    TooManyBlocksBack,
}

#[cfg(feature = "std")]
impl std::error::Error for StarknetError {}

impl core::fmt::Display for StarknetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FailedToReceiveTransaction => write!(f, "FailedToReceiveTransaction"),
            Self::ContractNotFound => write!(f, "ContractNotFound"),
            Self::EntrypointNotFound => write!(f, "EntrypointNotFound"),
            Self::BlockNotFound => write!(f, "BlockNotFound"),
            Self::InvalidTransactionIndex => write!(f, "InvalidTransactionIndex"),
            Self::ClassHashNotFound => write!(f, "ClassHashNotFound"),
            Self::TransactionHashNotFound => write!(f, "TransactionHashNotFound"),
            Self::PageSizeTooBig => write!(f, "PageSizeTooBig"),
            Self::NoBlocks => write!(f, "NoBlocks"),
            Self::InvalidContinuationToken => write!(f, "InvalidContinuationToken"),
            Self::TooManyKeysInFilter => write!(f, "TooManyKeysInFilter"),
            Self::ContractError(e) => write!(f, "ContractError: {e:?}"),
            Self::TransactionExecutionError(e) => write!(f, "TransactionExecutionError: {e:?}"),
            Self::StorageProofNotSupported => write!(f, "StorageProofNotSupported"),
            Self::ClassAlreadyDeclared => write!(f, "ClassAlreadyDeclared"),
            Self::InvalidTransactionNonce(e) => write!(f, "InvalidTransactionNonce: {e:?}"),
            Self::InsufficientResourcesForValidate => write!(f, "InsufficientResourcesForValidate"),
            Self::InsufficientAccountBalance => write!(f, "InsufficientAccountBalance"),
            Self::ValidationFailure(e) => write!(f, "ValidationFailure: {e:?}"),
            Self::CompilationFailed(e) => write!(f, "CompilationFailed: {e:?}"),
            Self::ContractClassSizeIsTooLarge => write!(f, "ContractClassSizeIsTooLarge"),
            Self::NonAccount => write!(f, "NonAccount"),
            Self::DuplicateTx => write!(f, "DuplicateTx"),
            Self::CompiledClassHashMismatch => write!(f, "CompiledClassHashMismatch"),
            Self::UnsupportedTxVersion => write!(f, "UnsupportedTxVersion"),
            Self::UnsupportedContractClassVersion => write!(f, "UnsupportedContractClassVersion"),
            Self::UnexpectedError(e) => write!(f, "UnexpectedError: {e:?}"),
            Self::ReplacementTransactionUnderpriced => {
                write!(f, "ReplacementTransactionUnderpriced")
            }
            Self::FeeBelowMinimum => write!(f, "FeeBelowMinimum"),
            Self::NoTraceAvailable(e) => write!(f, "NoTraceAvailable: {e:?}"),
            Self::InvalidSubscriptionId => write!(f, "InvalidSubscriptionId"),
            Self::TooManyAddressesInFilter => write!(f, "TooManyAddressesInFilter"),
            Self::TooManyBlocksBack => write!(f, "TooManyBlocksBack"),
        }
    }
}

impl StarknetError {
    pub const fn code(&self) -> u32 {
        match self {
            Self::FailedToReceiveTransaction => 1,
            Self::ContractNotFound => 20,
            Self::EntrypointNotFound => 21,
            Self::BlockNotFound => 24,
            Self::InvalidTransactionIndex => 27,
            Self::ClassHashNotFound => 28,
            Self::TransactionHashNotFound => 29,
            Self::PageSizeTooBig => 31,
            Self::NoBlocks => 32,
            Self::InvalidContinuationToken => 33,
            Self::TooManyKeysInFilter => 34,
            Self::ContractError(_) => 40,
            Self::TransactionExecutionError(_) => 41,
            Self::StorageProofNotSupported => 42,
            Self::ClassAlreadyDeclared => 51,
            Self::InvalidTransactionNonce(_) => 52,
            Self::InsufficientResourcesForValidate => 53,
            Self::InsufficientAccountBalance => 54,
            Self::ValidationFailure(_) => 55,
            Self::CompilationFailed(_) => 56,
            Self::ContractClassSizeIsTooLarge => 57,
            Self::NonAccount => 58,
            Self::DuplicateTx => 59,
            Self::CompiledClassHashMismatch => 60,
            Self::UnsupportedTxVersion => 61,
            Self::UnsupportedContractClassVersion => 62,
            Self::UnexpectedError(_) => 63,
            Self::ReplacementTransactionUnderpriced => 64,
            Self::FeeBelowMinimum => 65,
            Self::NoTraceAvailable(_) => 10,
            Self::InvalidSubscriptionId => 66,
            Self::TooManyAddressesInFilter => 67,
            Self::TooManyBlocksBack => 68,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::FailedToReceiveTransaction => "Failed to write transaction",
            Self::ContractNotFound => "Contract not found",
            Self::EntrypointNotFound => "Requested entrypoint does not exist in the contract",
            Self::BlockNotFound => "Block not found",
            Self::InvalidTransactionIndex => "Invalid transaction index in a block",
            Self::ClassHashNotFound => "Class hash not found",
            Self::TransactionHashNotFound => "Transaction hash not found",
            Self::PageSizeTooBig => "Requested page size is too big",
            Self::NoBlocks => "There are no blocks",
            Self::InvalidContinuationToken => {
                "The supplied continuation token is invalid or unknown"
            }
            Self::TooManyKeysInFilter => "Too many keys provided in a filter",
            Self::ContractError(_) => "Contract error",
            Self::TransactionExecutionError(_) => "Transaction execution error",
            Self::StorageProofNotSupported => {
                "the node doesn't support storage proofs for blocks that are too far in the past"
            }
            Self::ClassAlreadyDeclared => "Class already declared",
            Self::InvalidTransactionNonce(_) => "Invalid transaction nonce",
            Self::InsufficientResourcesForValidate => {
                "The transaction's resources don't cover validation or the minimal transaction fee"
            }
            Self::InsufficientAccountBalance => {
                "Account balance is smaller than the transaction's maximal fee (calculated as the sum of each resource's limit x max price)"
            }
            Self::ValidationFailure(_) => "Account validation failed",
            Self::CompilationFailed(_) => "Compilation failed",
            Self::ContractClassSizeIsTooLarge => "Contract class size is too large",
            Self::NonAccount => "Sender address is not an account contract",
            Self::DuplicateTx => "A transaction with the same hash already exists in the mempool",
            Self::CompiledClassHashMismatch => {
                "the compiled class hash did not match the one supplied in the transaction"
            }
            Self::UnsupportedTxVersion => "the transaction version is not supported",
            Self::UnsupportedContractClassVersion => "the contract class version is not supported",
            Self::UnexpectedError(_) => "An unexpected error occurred",
            Self::ReplacementTransactionUnderpriced => "Replacement transaction is underpriced",
            Self::FeeBelowMinimum => "Transaction fee below minimum",
            Self::NoTraceAvailable(_) => "No trace available for transaction",
            Self::InvalidSubscriptionId => "Invalid subscription id",
            Self::TooManyAddressesInFilter => "Too many addresses in filter sender_address filter",
            Self::TooManyBlocksBack => "Cannot go back more than 1024 blocks",
        }
    }
}

/// The change in state applied in this block, given as a mapping of addresses to the new values
/// and/or new contracts.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    /// Storage diffs
    pub storage_diffs: Vec<ContractStorageDiffItem>,
    /// Deprecated declared classes
    #[serde_as(as = "Vec<UfeHex>")]
    pub deprecated_declared_classes: Vec<Felt>,
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
    pub block_hash: Felt,
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: Felt,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: Felt,
    /// State diff
    pub state_diff: StateDiff,
}

/// Storage diff item.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageEntry {
    /// The key of the changed value
    #[serde_as(as = "UfeHex")]
    pub key: Felt,
    /// The new value applied to the given address
    #[serde_as(as = "UfeHex")]
    pub value: Felt,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageProof {
    #[serde_as(as = "MerkleNodeMap")]
    pub classes_proof: IndexMap<Felt, MerkleNode, RandomState>,
    pub contracts_proof: ContractsProof,
    #[serde_as(as = "Vec<MerkleNodeMap>")]
    pub contracts_storage_proofs: Vec<IndexMap<Felt, MerkleNode, RandomState>>,
    pub global_roots: GlobalRoots,
}

/// Subscription id.
///
/// An identifier for this subscription stream used to associate events with this subscription.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubscriptionId(pub String);

/// Sync status.
///
/// An object describing the node synchronization status.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct SyncStatus {
    /// The hash of the block from which the sync started
    #[serde_as(as = "UfeHex")]
    pub starting_block_hash: Felt,
    /// The number (height) of the block from which the sync started
    pub starting_block_num: u64,
    /// The hash of the current block being synchronized
    #[serde_as(as = "UfeHex")]
    pub current_block_hash: Felt,
    /// The number (height) of the current block being synchronized
    pub current_block_num: u64,
    /// The hash of the estimated highest block to be synchronized
    #[serde_as(as = "UfeHex")]
    pub highest_block_hash: Felt,
    /// The number (height) of the estimated highest block to be synchronized
    pub highest_block_num: u64,
}

/// More data about the execution failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionExecutionErrorData {
    /// The index of the first transaction failing in a sequence of given transactions
    pub transaction_index: u64,
    /// The execution trace up to the point of failure
    pub execution_error: ContractExecutionError,
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
    #[serde(rename = "PRE_CONFIRMED")]
    PreConfirmed,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
}

/// Transaction receipt with block info.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionReceiptWithBlockInfo {
    #[serde(flatten)]
    pub receipt: TransactionReceipt,
    #[serde(flatten)]
    pub block: ReceiptBlock,
}

/// A single pair of transaction hash and corresponding trace.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTraceWithHash {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    pub trace_root: TransactionTrace,
}

/// Either a tranasaction hash or full transaction details, based on subscription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionWithL2Status {
    #[serde(flatten)]
    pub txn: Transaction,
    /// Finality status of the transaction
    pub finality_status: L2TransactionStatus,
}

/// Transaction and receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionWithReceipt {
    /// Transaction
    pub transaction: TransactionContent,
    /// Receipt
    pub receipt: TransactionReceipt,
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
    /// describes what parts of the transaction should be executed
    pub simulation_flags: Vec<SimulationFlagForEstimateFee>,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag,
    /// for the block referencing the state or call the transaction on.
    pub block_id: BlockId,
}

/// Reference version of [EstimateFeeRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstimateFeeRequestRef<'a> {
    pub request: &'a [BroadcastedTransaction],
    pub simulation_flags: &'a [SimulationFlagForEstimateFee],
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

/// Request for method starknet_getBlockWithReceipts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithReceiptsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetBlockWithReceiptsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlockWithReceiptsRequestRef<'a> {
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
    pub contract_address: Felt,
}

/// Reference version of [GetClassAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt,
}

/// Request for method starknet_getClassHashAt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassHashAtRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose class hash will be returned
    pub contract_address: Felt,
}

/// Reference version of [GetClassHashAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassHashAtRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt,
}

/// Request for method starknet_getClass
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The hash of the requested contract class
    pub class_hash: Felt,
}

/// Reference version of [GetClassRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClassRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub class_hash: &'a Felt,
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

/// Request for method starknet_getMessagesStatus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMessagesStatusRequest {
    pub transaction_hash: Hash256,
}

/// Reference version of [GetMessagesStatusRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMessagesStatusRequestRef<'a> {
    pub transaction_hash: &'a Hash256,
}

/// Request for method starknet_getNonce
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNonceRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
    /// The address of the contract whose nonce we're seeking
    pub contract_address: Felt,
}

/// Reference version of [GetNonceRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetNonceRequestRef<'a> {
    pub block_id: &'a BlockId,
    pub contract_address: &'a Felt,
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
    pub contract_address: Felt,
    /// The key to the storage value for the given contract
    pub key: Felt,
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: BlockId,
}

/// Reference version of [GetStorageAtRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStorageAtRequestRef<'a> {
    pub contract_address: &'a Felt,
    pub key: &'a Felt,
    pub block_id: &'a BlockId,
}

/// Request for method starknet_getStorageProof
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStorageProofRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: ConfirmedBlockId,
    /// a list of the class hashes for which we want to prove membership in the classes trie
    pub class_hashes: Option<Vec<Felt>>,
    /// a list of contracts for which we want to prove membership in the global state trie
    pub contract_addresses: Option<Vec<Felt>>,
    /// a list of (contract_address, storage_keys) pairs
    pub contracts_storage_keys: Option<Vec<ContractStorageKeys>>,
}

/// Reference version of [GetStorageProofRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetStorageProofRequestRef<'a> {
    pub block_id: &'a ConfirmedBlockId,
    pub class_hashes: Option<&'a [Felt]>,
    pub contract_addresses: Option<&'a [Felt]>,
    pub contracts_storage_keys: Option<&'a [ContractStorageKeys]>,
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
    pub transaction_hash: Felt,
}

/// Reference version of [GetTransactionByHashRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionByHashRequestRef<'a> {
    pub transaction_hash: &'a Felt,
}

/// Request for method starknet_getTransactionReceipt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionReceiptRequest {
    pub transaction_hash: Felt,
}

/// Reference version of [GetTransactionReceiptRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionReceiptRequestRef<'a> {
    pub transaction_hash: &'a Felt,
}

/// Request for method starknet_getTransactionStatus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionStatusRequest {
    pub transaction_hash: Felt,
}

/// Reference version of [GetTransactionStatusRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTransactionStatusRequestRef<'a> {
    pub transaction_hash: &'a Felt,
}

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

/// Request for method starknet_specVersion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecVersionRequest;

/// Request for method starknet_subscribeEvents
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeEventsRequest {
    pub from_address: Option<Felt>,
    pub keys: Option<Vec<Vec<Felt>>>,
    pub block_id: Option<ConfirmedBlockId>,
    pub finality_status: Option<L2TransactionFinalityStatus>,
}

/// Reference version of [SubscribeEventsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeEventsRequestRef<'a> {
    pub from_address: &'a Option<Felt>,
    pub keys: Option<&'a [Vec<Felt>]>,
    pub block_id: &'a Option<ConfirmedBlockId>,
    pub finality_status: &'a Option<L2TransactionFinalityStatus>,
}

/// Request for method starknet_subscribeNewHeads
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewHeadsRequest {
    pub block_id: Option<ConfirmedBlockId>,
}

/// Reference version of [SubscribeNewHeadsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewHeadsRequestRef<'a> {
    pub block_id: &'a Option<ConfirmedBlockId>,
}

/// Request for method starknet_subscribeNewTransactionReceipts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewTransactionReceiptsRequest {
    pub finality_status: Option<Vec<L2TransactionFinalityStatus>>,
    pub sender_address: Option<Vec<Felt>>,
}

/// Reference version of [SubscribeNewTransactionReceiptsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewTransactionReceiptsRequestRef<'a> {
    pub finality_status: Option<&'a [L2TransactionFinalityStatus]>,
    pub sender_address: Option<&'a [Felt]>,
}

/// Request for method starknet_subscribeNewTransactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewTransactionsRequest {
    pub finality_status: Option<Vec<L2TransactionStatus>>,
    pub sender_address: Option<Vec<Felt>>,
}

/// Reference version of [SubscribeNewTransactionsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeNewTransactionsRequestRef<'a> {
    pub finality_status: Option<&'a [L2TransactionStatus]>,
    pub sender_address: Option<&'a [Felt]>,
}

/// Request for method starknet_subscribeTransactionStatus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeTransactionStatusRequest {
    pub transaction_hash: Felt,
}

/// Reference version of [SubscribeTransactionStatusRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscribeTransactionStatusRequestRef<'a> {
    pub transaction_hash: &'a Felt,
}

/// Request for method starknet_subscriptionEvents
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionEventsRequest {
    pub subscription_id: SubscriptionId,
    pub result: EmittedEventWithFinality,
}

/// Reference version of [SubscriptionEventsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionEventsRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a EmittedEventWithFinality,
}

/// Request for method starknet_subscriptionNewHeads
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewHeadsRequest {
    pub subscription_id: SubscriptionId,
    pub result: BlockHeader,
}

/// Reference version of [SubscriptionNewHeadsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewHeadsRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a BlockHeader,
}

/// Request for method starknet_subscriptionNewTransactionReceipts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewTransactionReceiptsRequest {
    pub subscription_id: SubscriptionId,
    /// A transaction receipt
    pub result: TransactionReceiptWithBlockInfo,
}

/// Reference version of [SubscriptionNewTransactionReceiptsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewTransactionReceiptsRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a TransactionReceiptWithBlockInfo,
}

/// Request for method starknet_subscriptionNewTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewTransactionRequest {
    pub subscription_id: SubscriptionId,
    /// A transaction and its current finality status
    pub result: TransactionWithL2Status,
}

/// Reference version of [SubscriptionNewTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionNewTransactionRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a TransactionWithL2Status,
}

/// Request for method starknet_subscriptionReorg
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionReorgRequest {
    pub subscription_id: SubscriptionId,
    pub result: ReorgData,
}

/// Reference version of [SubscriptionReorgRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionReorgRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a ReorgData,
}

/// Request for method starknet_subscriptionTransactionStatus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionTransactionStatusRequest {
    pub subscription_id: SubscriptionId,
    pub result: NewTransactionStatus,
}

/// Reference version of [SubscriptionTransactionStatusRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionTransactionStatusRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
    pub result: &'a NewTransactionStatus,
}

/// Request for method starknet_syncing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncingRequest;

/// Request for method starknet_traceBlockTransactions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceBlockTransactionsRequest {
    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    pub block_id: ConfirmedBlockId,
}

/// Reference version of [TraceBlockTransactionsRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceBlockTransactionsRequestRef<'a> {
    pub block_id: &'a ConfirmedBlockId,
}

/// Request for method starknet_traceTransaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceTransactionRequest {
    pub transaction_hash: Felt,
}

/// Reference version of [TraceTransactionRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceTransactionRequestRef<'a> {
    pub transaction_hash: &'a Felt,
}

/// Request for method starknet_unsubscribe
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsubscribeRequest {
    pub subscription_id: SubscriptionId,
}

/// Reference version of [UnsubscribeRequest].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsubscribeRequestRef<'a> {
    pub subscription_id: &'a SubscriptionId,
}

impl Serialize for BroadcastedDeclareTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            pub contract_class: &'a FlattenedSierraClass,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DECLARE";

        let version = &(if self.is_query {
            Felt::THREE + QUERY_VERSION_OFFSET
        } else {
            Felt::THREE
        });

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_class: &self.contract_class,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeclareTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: Felt,
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            pub contract_class: FlattenedSierraClass,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == Felt::THREE {
            false
        } else if tagged.version == Felt::THREE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_class: OwnedPtr::new(tagged.contract_class),
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
            is_query,
        })
    }
}

impl Serialize for BroadcastedDeployAccountTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &(if self.is_query {
            Felt::THREE + QUERY_VERSION_OFFSET
        } else {
            Felt::THREE
        });

        let tagged = Tagged {
            r#type,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedDeployAccountTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == Felt::THREE {
            false
        } else if tagged.version == Felt::THREE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
            is_query,
        })
    }
}

impl Serialize for BroadcastedInvokeTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "INVOKE";

        let version = &(if self.is_query {
            Felt::THREE + QUERY_VERSION_OFFSET
        } else {
            Felt::THREE
        });

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for BroadcastedInvokeTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        let is_query = if tagged.version == Felt::THREE {
            false
        } else if tagged.version == Felt::THREE + QUERY_VERSION_OFFSET {
            true
        } else {
            return Err(serde::de::Error::custom("invalid `version` value"));
        };

        Ok(Self {
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            signature: tagged.signature,
            nonce: tagged.nonce,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
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
            pub transaction_hash: &'a Felt,
            pub actual_fee: &'a FeePayment,
            pub finality_status: &'a TransactionFinalityStatus,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub execution_resources: &'a ExecutionResources,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "DECLARE";

        let tagged = Tagged {
            r#type,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_resources: &self.execution_resources,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub actual_fee: FeePayment,
            pub finality_status: TransactionFinalityStatus,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub execution_resources: ExecutionResources,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_resources: tagged.execution_resources,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for DeclareTransactionTrace {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: &'a Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: &'a Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: &'a Option<StateDiff>,
            pub execution_resources: &'a ExecutionResources,
            pub r#type: &'a str,
        }

        let r#type = "DECLARE";

        let tagged = Tagged {
            validate_invocation: &self.validate_invocation,
            fee_transfer_invocation: &self.fee_transfer_invocation,
            state_diff: &self.state_diff,
            execution_resources: &self.execution_resources,
            r#type,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionTrace {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: Option<StateDiff>,
            pub execution_resources: ExecutionResources,
            pub r#type: Option<String>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            validate_invocation: tagged.validate_invocation,
            fee_transfer_invocation: tagged.fee_transfer_invocation,
            state_diff: tagged.state_diff,
            execution_resources: tagged.execution_resources,
        })
    }
}

impl Serialize for DeclareTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &0;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            class_hash: &self.class_hash,
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
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV0Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &0;

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV0Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &1;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
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
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV1Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &1;

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV1Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &2;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
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
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV2Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DECLARE";

        let version = &2;

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            max_fee: &self.max_fee,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV2Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: Felt,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeclareTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DECLARE";

        let version = &3;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for DeclareTransactionV3Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DECLARE";

        let version = &3;

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            compiled_class_hash: &self.compiled_class_hash,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            class_hash: &self.class_hash,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeclareTransactionV3Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub compiled_class_hash: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DECLARE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            sender_address: tagged.sender_address,
            compiled_class_hash: tagged.compiled_class_hash,
            signature: tagged.signature,
            nonce: tagged.nonce,
            class_hash: tagged.class_hash,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for DeployAccountTransactionReceipt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub actual_fee: &'a FeePayment,
            pub finality_status: &'a TransactionFinalityStatus,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub execution_resources: &'a ExecutionResources,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_resources: &self.execution_resources,
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
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub actual_fee: FeePayment,
            pub finality_status: TransactionFinalityStatus,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub execution_resources: ExecutionResources,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_resources: tagged.execution_resources,
            execution_result: tagged.execution_result,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for DeployAccountTransactionTrace {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: &'a Option<FunctionInvocation>,
            pub constructor_invocation: &'a FunctionInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: &'a Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: &'a Option<StateDiff>,
            pub execution_resources: &'a ExecutionResources,
            pub r#type: &'a str,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let tagged = Tagged {
            validate_invocation: &self.validate_invocation,
            constructor_invocation: &self.constructor_invocation,
            fee_transfer_invocation: &self.fee_transfer_invocation,
            state_diff: &self.state_diff,
            execution_resources: &self.execution_resources,
            r#type,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransactionTrace {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: Option<FunctionInvocation>,
            pub constructor_invocation: FunctionInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: Option<StateDiff>,
            pub execution_resources: ExecutionResources,
            pub r#type: Option<String>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            validate_invocation: tagged.validate_invocation,
            constructor_invocation: tagged.constructor_invocation,
            fee_transfer_invocation: tagged.fee_transfer_invocation,
            state_diff: tagged.state_diff,
            execution_resources: tagged.execution_resources,
        })
    }
}

impl Serialize for DeployAccountTransactionV1 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
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

impl<'de> Deserialize<'de> for DeployAccountTransactionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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

impl Serialize for DeployAccountTransactionV1Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &1;

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

impl<'de> Deserialize<'de> for DeployAccountTransactionV1Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
        })
    }
}

impl Serialize for DeployAccountTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &3;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for DeployAccountTransactionV3Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "DEPLOY_ACCOUNT";

        let version = &3;

        let tagged = Tagged {
            r#type,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployAccountTransactionV3Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY_ACCOUNT" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            signature: tagged.signature,
            nonce: tagged.nonce,
            contract_address_salt: tagged.contract_address_salt,
            constructor_calldata: tagged.constructor_calldata,
            class_hash: tagged.class_hash,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for DeployTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
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
            pub transaction_hash: Felt,
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
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

impl Serialize for DeployTransactionContent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub constructor_calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub class_hash: &'a Felt,
        }

        let r#type = "DEPLOY";

        let tagged = Tagged {
            version: &self.version,
            r#type,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            class_hash: &self.class_hash,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for DeployTransactionContent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address_salt: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub constructor_calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub class_hash: Felt,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "DEPLOY" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
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
            pub transaction_hash: &'a Felt,
            pub actual_fee: &'a FeePayment,
            pub finality_status: &'a TransactionFinalityStatus,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub execution_resources: &'a ExecutionResources,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
        }

        let r#type = "DEPLOY";

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_resources: &self.execution_resources,
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
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub actual_fee: FeePayment,
            pub finality_status: TransactionFinalityStatus,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub execution_resources: ExecutionResources,
            #[serde(flatten)]
            pub execution_result: ExecutionResult,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_resources: tagged.execution_resources,
            execution_result: tagged.execution_result,
            contract_address: tagged.contract_address,
        })
    }
}

impl Serialize for FeeEstimate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub overall_fee: &'a u128,
            pub unit: &'a PriceUnitFri,
        }

        let unit = &PriceUnitFri::Fri;

        let tagged = Tagged {
            l1_gas_consumed: &self.l1_gas_consumed,
            l1_gas_price: &self.l1_gas_price,
            l2_gas_consumed: &self.l2_gas_consumed,
            l2_gas_price: &self.l2_gas_price,
            l1_data_gas_consumed: &self.l1_data_gas_consumed,
            l1_data_gas_price: &self.l1_data_gas_price,
            overall_fee: &self.overall_fee,
            unit,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for FeeEstimate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub overall_fee: u128,
            pub unit: Option<PriceUnitFri>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.unit {
            if tag_field != &PriceUnitFri::Fri {
                return Err(serde::de::Error::custom("invalid `unit` value"));
            }
        }

        Ok(Self {
            l1_gas_consumed: tagged.l1_gas_consumed,
            l1_gas_price: tagged.l1_gas_price,
            l2_gas_consumed: tagged.l2_gas_consumed,
            l2_gas_price: tagged.l2_gas_price,
            l1_data_gas_consumed: tagged.l1_data_gas_consumed,
            l1_data_gas_price: tagged.l1_data_gas_price,
            overall_fee: tagged.overall_fee,
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
            pub transaction_hash: &'a Felt,
            pub actual_fee: &'a FeePayment,
            pub finality_status: &'a TransactionFinalityStatus,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub execution_resources: &'a ExecutionResources,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "INVOKE";

        let tagged = Tagged {
            r#type,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_resources: &self.execution_resources,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub actual_fee: FeePayment,
            pub finality_status: TransactionFinalityStatus,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub execution_resources: ExecutionResources,
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
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_resources: tagged.execution_resources,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for InvokeTransactionTrace {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: &'a Option<FunctionInvocation>,
            pub execute_invocation: &'a ExecuteInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: &'a Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: &'a Option<StateDiff>,
            pub execution_resources: &'a ExecutionResources,
            pub r#type: &'a str,
        }

        let r#type = "INVOKE";

        let tagged = Tagged {
            validate_invocation: &self.validate_invocation,
            execute_invocation: &self.execute_invocation,
            fee_transfer_invocation: &self.fee_transfer_invocation,
            state_diff: &self.state_diff,
            execution_resources: &self.execution_resources,
            r#type,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionTrace {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub validate_invocation: Option<FunctionInvocation>,
            pub execute_invocation: ExecuteInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fee_transfer_invocation: Option<FunctionInvocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: Option<StateDiff>,
            pub execution_resources: ExecutionResources,
            pub r#type: Option<String>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            validate_invocation: tagged.validate_invocation,
            execute_invocation: tagged.execute_invocation,
            fee_transfer_invocation: tagged.fee_transfer_invocation,
            state_diff: tagged.state_diff,
            execution_resources: tagged.execution_resources,
        })
    }
}

impl Serialize for InvokeTransactionV0 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
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
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
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

impl Serialize for InvokeTransactionV0Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
        }

        let r#type = "INVOKE";

        let version = &0;

        let tagged = Tagged {
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

impl<'de> Deserialize<'de> for InvokeTransactionV0Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
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
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
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
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
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

impl Serialize for InvokeTransactionV1Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a Felt,
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
        }

        let r#type = "INVOKE";

        let version = &1;

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

impl<'de> Deserialize<'de> for InvokeTransactionV1Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub max_fee: Felt,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
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
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            max_fee: tagged.max_fee,
            signature: tagged.signature,
            nonce: tagged.nonce,
        })
    }
}

impl Serialize for InvokeTransactionV3 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "INVOKE";

        let version = &3;

        let tagged = Tagged {
            transaction_hash: &self.transaction_hash,
            r#type,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV3 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            transaction_hash: tagged.transaction_hash,
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            signature: tagged.signature,
            nonce: tagged.nonce,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for InvokeTransactionV3Content {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub r#type: &'a str,
            #[serde_as(as = "UfeHex")]
            pub sender_address: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
            #[serde_as(as = "NumAsHex")]
            pub version: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub signature: &'a [Felt],
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a Felt,
            pub resource_bounds: &'a ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: &'a u64,
            #[serde_as(as = "[UfeHex]")]
            pub paymaster_data: &'a [Felt],
            #[serde_as(as = "[UfeHex]")]
            pub account_deployment_data: &'a [Felt],
            pub nonce_data_availability_mode: &'a DataAvailabilityMode,
            pub fee_data_availability_mode: &'a DataAvailabilityMode,
        }

        let r#type = "INVOKE";

        let version = &3;

        let tagged = Tagged {
            r#type,
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            version,
            signature: &self.signature,
            nonce: &self.nonce,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for InvokeTransactionV3Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub r#type: Option<String>,
            #[serde_as(as = "UfeHex")]
            pub sender_address: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
            #[serde_as(as = "Option<NumAsHex>")]
            pub version: Option<u64>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: Vec<Felt>,
            #[serde_as(as = "UfeHex")]
            pub nonce: Felt,
            pub resource_bounds: ResourceBoundsMapping,
            #[serde_as(as = "NumAsHex")]
            pub tip: u64,
            #[serde_as(as = "Vec<UfeHex>")]
            pub paymaster_data: Vec<Felt>,
            #[serde_as(as = "Vec<UfeHex>")]
            pub account_deployment_data: Vec<Felt>,
            pub nonce_data_availability_mode: DataAvailabilityMode,
            pub fee_data_availability_mode: DataAvailabilityMode,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "INVOKE" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        if let Some(tag_field) = &tagged.version {
            if tag_field != &3 {
                return Err(serde::de::Error::custom("invalid `version` value"));
            }
        }

        Ok(Self {
            sender_address: tagged.sender_address,
            calldata: tagged.calldata,
            signature: tagged.signature,
            nonce: tagged.nonce,
            resource_bounds: tagged.resource_bounds,
            tip: tagged.tip,
            paymaster_data: tagged.paymaster_data,
            account_deployment_data: tagged.account_deployment_data,
            nonce_data_availability_mode: tagged.nonce_data_availability_mode,
            fee_data_availability_mode: tagged.fee_data_availability_mode,
        })
    }
}

impl Serialize for L1HandlerTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub nonce: &'a u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
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
            pub transaction_hash: Felt,
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "NumAsHex")]
            pub nonce: u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
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

impl Serialize for L1HandlerTransactionContent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "UfeHex")]
            pub version: &'a Felt,
            pub r#type: &'a str,
            #[serde_as(as = "NumAsHex")]
            pub nonce: &'a u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: &'a Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: &'a Felt,
            #[serde_as(as = "[UfeHex]")]
            pub calldata: &'a [Felt],
        }

        let r#type = "L1_HANDLER";

        let tagged = Tagged {
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

impl<'de> Deserialize<'de> for L1HandlerTransactionContent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "UfeHex")]
            pub version: Felt,
            pub r#type: Option<String>,
            #[serde_as(as = "NumAsHex")]
            pub nonce: u64,
            #[serde_as(as = "UfeHex")]
            pub contract_address: Felt,
            #[serde_as(as = "UfeHex")]
            pub entry_point_selector: Felt,
            #[serde_as(as = "Vec<UfeHex>")]
            pub calldata: Vec<Felt>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
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
            pub message_hash: &'a Hash256,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: &'a Felt,
            pub actual_fee: &'a FeePayment,
            pub finality_status: &'a TransactionFinalityStatus,
            pub messages_sent: &'a [MsgToL1],
            pub events: &'a [Event],
            pub execution_resources: &'a ExecutionResources,
            #[serde(flatten)]
            pub execution_result: &'a ExecutionResult,
        }

        let r#type = "L1_HANDLER";

        let tagged = Tagged {
            r#type,
            message_hash: &self.message_hash,
            transaction_hash: &self.transaction_hash,
            actual_fee: &self.actual_fee,
            finality_status: &self.finality_status,
            messages_sent: &self.messages_sent,
            events: &self.events,
            execution_resources: &self.execution_resources,
            execution_result: &self.execution_result,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransactionReceipt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Tagged {
            pub r#type: Option<String>,
            pub message_hash: Hash256,
            #[serde_as(as = "UfeHex")]
            pub transaction_hash: Felt,
            pub actual_fee: FeePayment,
            pub finality_status: TransactionFinalityStatus,
            pub messages_sent: Vec<MsgToL1>,
            pub events: Vec<Event>,
            pub execution_resources: ExecutionResources,
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
            message_hash: tagged.message_hash,
            transaction_hash: tagged.transaction_hash,
            actual_fee: tagged.actual_fee,
            finality_status: tagged.finality_status,
            messages_sent: tagged.messages_sent,
            events: tagged.events,
            execution_resources: tagged.execution_resources,
            execution_result: tagged.execution_result,
        })
    }
}

impl Serialize for L1HandlerTransactionTrace {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Tagged<'a> {
            pub function_invocation: &'a ExecuteInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: &'a Option<StateDiff>,
            pub execution_resources: &'a ExecutionResources,
            pub r#type: &'a str,
        }

        let r#type = "L1_HANDLER";

        let tagged = Tagged {
            function_invocation: &self.function_invocation,
            state_diff: &self.state_diff,
            execution_resources: &self.execution_resources,
            r#type,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for L1HandlerTransactionTrace {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            pub function_invocation: ExecuteInvocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state_diff: Option<StateDiff>,
            pub execution_resources: ExecutionResources,
            pub r#type: Option<String>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.r#type {
            if tag_field != "L1_HANDLER" {
                return Err(serde::de::Error::custom("invalid `type` value"));
            }
        }

        Ok(Self {
            function_invocation: tagged.function_invocation,
            state_diff: tagged.state_diff,
            execution_resources: tagged.execution_resources,
        })
    }
}

impl Serialize for MessageFeeEstimate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[serde_as]
        #[derive(Serialize)]
        struct Tagged<'a> {
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_consumed: &'a u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_price: &'a u128,
            #[serde_as(as = "NumAsHex")]
            pub overall_fee: &'a u128,
            pub unit: &'a PriceUnitWei,
        }

        let unit = &PriceUnitWei::Wei;

        let tagged = Tagged {
            l1_gas_consumed: &self.l1_gas_consumed,
            l1_gas_price: &self.l1_gas_price,
            l2_gas_consumed: &self.l2_gas_consumed,
            l2_gas_price: &self.l2_gas_price,
            l1_data_gas_consumed: &self.l1_data_gas_consumed,
            l1_data_gas_price: &self.l1_data_gas_price,
            overall_fee: &self.overall_fee,
            unit,
        };

        Tagged::serialize(&tagged, serializer)
    }
}

impl<'de> Deserialize<'de> for MessageFeeEstimate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize)]
        #[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
        struct Tagged {
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l2_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_consumed: u64,
            #[serde_as(as = "NumAsHex")]
            pub l1_data_gas_price: u128,
            #[serde_as(as = "NumAsHex")]
            pub overall_fee: u128,
            pub unit: Option<PriceUnitWei>,
        }

        let tagged = Tagged::deserialize(deserializer)?;

        if let Some(tag_field) = &tagged.unit {
            if tag_field != &PriceUnitWei::Wei {
                return Err(serde::de::Error::custom("invalid `unit` value"));
            }
        }

        Ok(Self {
            l1_gas_consumed: tagged.l1_gas_consumed,
            l1_gas_price: tagged.l1_gas_price,
            l2_gas_consumed: tagged.l2_gas_consumed,
            l2_gas_price: tagged.l2_gas_price,
            l1_data_gas_consumed: tagged.l1_data_gas_consumed,
            l1_data_gas_price: tagged.l1_data_gas_price,
            overall_fee: tagged.overall_fee,
        })
    }
}

impl Serialize for AddDeclareTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            declare_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedDeclareTransaction,
        }

        AsObject::serialize(
            &AsObject {
                declare_transaction: Field0 {
                    value: &self.declare_transaction,
                },
            },
            serializer,
        )
    }
}

impl Serialize for AddDeclareTransactionRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            declare_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedDeclareTransaction,
        }

        AsObject::serialize(
            &AsObject {
                declare_transaction: Field0 {
                    value: self.declare_transaction,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for AddDeclareTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            declare_transaction: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BroadcastedDeclareTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                declare_transaction: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                declare_transaction: object.declare_transaction.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddDeployAccountTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            deploy_account_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedDeployAccountTransaction,
        }

        AsObject::serialize(
            &AsObject {
                deploy_account_transaction: Field0 {
                    value: &self.deploy_account_transaction,
                },
            },
            serializer,
        )
    }
}

impl Serialize for AddDeployAccountTransactionRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            deploy_account_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedDeployAccountTransaction,
        }

        AsObject::serialize(
            &AsObject {
                deploy_account_transaction: Field0 {
                    value: self.deploy_account_transaction,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for AddDeployAccountTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            deploy_account_transaction: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BroadcastedDeployAccountTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                deploy_account_transaction: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                deploy_account_transaction: object.deploy_account_transaction.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for AddInvokeTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            invoke_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedInvokeTransaction,
        }

        AsObject::serialize(
            &AsObject {
                invoke_transaction: Field0 {
                    value: &self.invoke_transaction,
                },
            },
            serializer,
        )
    }
}

impl Serialize for AddInvokeTransactionRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            invoke_transaction: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BroadcastedInvokeTransaction,
        }

        AsObject::serialize(
            &AsObject {
                invoke_transaction: Field0 {
                    value: self.invoke_transaction,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for AddInvokeTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            invoke_transaction: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BroadcastedInvokeTransaction,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                invoke_transaction: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                invoke_transaction: object.invoke_transaction.value,
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
        struct AsObject<'a> {
            request: Field0<'a>,
            block_id: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                request: Field0 {
                    value: &self.request,
                },
                block_id: Field1 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for CallRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            request: Field0<'a>,
            block_id: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a FunctionCall,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                request: Field0 {
                    value: self.request,
                },
                block_id: Field1 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for CallRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            request: Field0,
            block_id: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: FunctionCall,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                request: field0.value,
                block_id: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request.value,
                block_id: object.block_id.value,
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
        struct AsObject<'a> {
            request: Field0<'a>,
            simulation_flags: Field1<'a>,
            block_id: Field2<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a [SimulationFlagForEstimateFee],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                request: Field0 {
                    value: &self.request,
                },
                simulation_flags: Field1 {
                    value: &self.simulation_flags,
                },
                block_id: Field2 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for EstimateFeeRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            request: Field0<'a>,
            simulation_flags: Field1<'a>,
            block_id: Field2<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a [SimulationFlagForEstimateFee],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                request: Field0 {
                    value: self.request,
                },
                simulation_flags: Field1 {
                    value: self.simulation_flags,
                },
                block_id: Field2 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for EstimateFeeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            request: Field0,
            simulation_flags: Field1,
            block_id: Field2,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: Vec<BroadcastedTransaction>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: Vec<SimulationFlagForEstimateFee>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                request: field0.value,
                simulation_flags: field1.value,
                block_id: field2.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                request: object.request.value,
                simulation_flags: object.simulation_flags.value,
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for EstimateMessageFeeRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            message: Field0<'a>,
            block_id: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a MsgFromL1,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                message: Field0 {
                    value: &self.message,
                },
                block_id: Field1 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for EstimateMessageFeeRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            message: Field0<'a>,
            block_id: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a MsgFromL1,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                message: Field0 {
                    value: self.message,
                },
                block_id: Field1 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for EstimateMessageFeeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            message: Field0,
            block_id: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: MsgFromL1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                message: field0.value,
                block_id: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                message: object.message.value,
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockTransactionCountRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetBlockTransactionCountRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetBlockTransactionCountRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithReceiptsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetBlockWithReceiptsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetBlockWithReceiptsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxHashesRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetBlockWithTxHashesRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxHashesRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetBlockWithTxsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetBlockWithTxsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetBlockWithTxsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                contract_address: Field1 {
                    value: &self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetClassAtRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                contract_address: Field1 {
                    value: self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetClassAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            contract_address: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                contract_address: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                contract_address: object.contract_address.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassHashAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                contract_address: Field1 {
                    value: &self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetClassHashAtRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                contract_address: Field1 {
                    value: self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetClassHashAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            contract_address: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                contract_address: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                contract_address: object.contract_address.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetClassRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            class_hash: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                class_hash: Field1 {
                    value: &self.class_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetClassRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            class_hash: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                class_hash: Field1 {
                    value: self.class_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetClassRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            class_hash: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                class_hash: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                class_hash: object.class_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetEventsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            filter: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a EventFilterWithPage,
        }

        AsObject::serialize(
            &AsObject {
                filter: Field0 {
                    value: &self.filter,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetEventsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            filter: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a EventFilterWithPage,
        }

        AsObject::serialize(
            &AsObject {
                filter: Field0 { value: self.filter },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetEventsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            filter: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: EventFilterWithPage,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                filter: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                filter: object.filter.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetMessagesStatusRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a Hash256,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetMessagesStatusRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a Hash256,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetMessagesStatusRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: Hash256,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetNonceRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                contract_address: Field1 {
                    value: &self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetNonceRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            contract_address: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                contract_address: Field1 {
                    value: self.contract_address,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetNonceRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            contract_address: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                contract_address: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                contract_address: object.contract_address.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStateUpdateRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetStateUpdateRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetStateUpdateRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStorageAtRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            contract_address: Field0<'a>,
            key: Field1<'a>,
            block_id: Field2<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                contract_address: Field0 {
                    value: &self.contract_address,
                },
                key: Field1 { value: &self.key },
                block_id: Field2 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetStorageAtRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            contract_address: Field0<'a>,
            key: Field1<'a>,
            block_id: Field2<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a BlockId,
        }

        AsObject::serialize(
            &AsObject {
                contract_address: Field0 {
                    value: self.contract_address,
                },
                key: Field1 { value: self.key },
                block_id: Field2 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetStorageAtRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            contract_address: Field0,
            key: Field1,
            block_id: Field2,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub value: BlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                contract_address: field0.value,
                key: field1.value,
                block_id: field2.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                contract_address: object.contract_address.value,
                key: object.key.value,
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetStorageProofRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            class_hashes: Option<Field1<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contract_addresses: Option<Field2<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contracts_storage_keys: Option<Field3<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field3<'a> {
            pub value: &'a [ContractStorageKeys],
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                class_hashes: self.class_hashes.as_ref().map(|f| Field1 { value: f }),
                contract_addresses: self
                    .contract_addresses
                    .as_ref()
                    .map(|f| Field2 { value: f }),
                contracts_storage_keys: self
                    .contracts_storage_keys
                    .as_ref()
                    .map(|f| Field3 { value: f }),
            },
            serializer,
        )
    }
}

impl Serialize for GetStorageProofRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            class_hashes: Option<Field1<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contract_addresses: Option<Field2<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contracts_storage_keys: Option<Field3<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field3<'a> {
            pub value: &'a [ContractStorageKeys],
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                class_hashes: self.class_hashes.as_ref().map(|f| Field1 { value: f }),
                contract_addresses: self
                    .contract_addresses
                    .as_ref()
                    .map(|f| Field2 { value: f }),
                contracts_storage_keys: self
                    .contracts_storage_keys
                    .as_ref()
                    .map(|f| Field3 { value: f }),
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetStorageProofRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            #[serde(skip_serializing_if = "Option::is_none")]
            class_hashes: Option<Field1>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contract_addresses: Option<Field2>,
            #[serde(skip_serializing_if = "Option::is_none")]
            contracts_storage_keys: Option<Field3>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: ConfirmedBlockId,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "Vec<UfeHex>")]
            pub value: Vec<Felt>,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            #[serde_as(as = "Vec<UfeHex>")]
            pub value: Vec<Felt>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field3 {
            pub value: Vec<ContractStorageKeys>,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let element_count = elements.len();

            let field3 = if element_count > 3 {
                Some(
                    serde_json::from_value::<Field3>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field2 = if element_count > 2 {
                Some(
                    serde_json::from_value::<Field2>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field1 = if element_count > 1 {
                Some(
                    serde_json::from_value::<Field1>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                class_hashes: field1.map(|f| f.value),
                contract_addresses: field2.map(|f| f.value),
                contracts_storage_keys: field3.map(|f| f.value),
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                class_hashes: object.class_hashes.map(|f| f.value),
                contract_addresses: object.contract_addresses.map(|f| f.value),
                contracts_storage_keys: object.contracts_storage_keys.map(|f| f.value),
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByBlockIdAndIndexRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            index: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a u64,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                index: Field1 { value: &self.index },
            },
            serializer,
        )
    }
}

impl Serialize for GetTransactionByBlockIdAndIndexRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            index: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a u64,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                index: Field1 { value: self.index },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetTransactionByBlockIdAndIndexRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            index: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: u64,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                index: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                index: object.index.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionByHashRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetTransactionByHashRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetTransactionByHashRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionReceiptRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetTransactionReceiptRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetTransactionReceiptRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for GetTransactionStatusRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for GetTransactionStatusRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for GetTransactionStatusRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SimulateTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            transactions: Field1<'a>,
            simulation_flags: Field2<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a [SimulationFlag],
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
                transactions: Field1 {
                    value: &self.transactions,
                },
                simulation_flags: Field2 {
                    value: &self.simulation_flags,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SimulateTransactionsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
            transactions: Field1<'a>,
            simulation_flags: Field2<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a BlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a [BroadcastedTransaction],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a [SimulationFlag],
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
                transactions: Field1 {
                    value: self.transactions,
                },
                simulation_flags: Field2 {
                    value: self.simulation_flags,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SimulateTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
            transactions: Field1,
            simulation_flags: Field2,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: BlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: Vec<BroadcastedTransaction>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub value: Vec<SimulationFlag>,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field2 = serde_json::from_value::<Field2>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
                transactions: field1.value,
                simulation_flags: field2.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
                transactions: object.transactions.value,
                simulation_flags: object.simulation_flags.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SpecVersionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SpecVersionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let elements = Vec::<()>::deserialize(deserializer)?;
        if !elements.is_empty() {
            return Err(serde::de::Error::custom("invalid sequence length"));
        }
        Ok(Self)
    }
}

impl Serialize for SubscribeEventsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_address: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            keys: Option<Field1<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field2<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field3<'a>>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[Vec<UfeHex>]")]
            pub value: &'a [Vec<Felt>],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field3<'a> {
            pub value: &'a L2TransactionFinalityStatus,
        }

        AsObject::serialize(
            &AsObject {
                from_address: self.from_address.as_ref().map(|f| Field0 { value: f }),
                keys: self.keys.as_ref().map(|f| Field1 { value: f }),
                block_id: self.block_id.as_ref().map(|f| Field2 { value: f }),
                finality_status: self.finality_status.as_ref().map(|f| Field3 { value: f }),
            },
            serializer,
        )
    }
}

impl Serialize for SubscribeEventsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_address: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            keys: Option<Field1<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field2<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field3<'a>>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[Vec<UfeHex>]")]
            pub value: &'a [Vec<Felt>],
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field2<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field3<'a> {
            pub value: &'a L2TransactionFinalityStatus,
        }

        AsObject::serialize(
            &AsObject {
                from_address: self.from_address.as_ref().map(|f| Field0 { value: f }),
                keys: self.keys.as_ref().map(|f| Field1 { value: f }),
                block_id: self.block_id.as_ref().map(|f| Field2 { value: f }),
                finality_status: self.finality_status.as_ref().map(|f| Field3 { value: f }),
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscribeEventsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_address: Option<Field0>,
            #[serde(skip_serializing_if = "Option::is_none")]
            keys: Option<Field1>,
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field2>,
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field3>,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "Vec<Vec<UfeHex>>")]
            pub value: Vec<Vec<Felt>>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field2 {
            pub value: ConfirmedBlockId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field3 {
            pub value: L2TransactionFinalityStatus,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let element_count = elements.len();

            let field3 = if element_count > 3 {
                Some(
                    serde_json::from_value::<Field3>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field2 = if element_count > 2 {
                Some(
                    serde_json::from_value::<Field2>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field1 = if element_count > 1 {
                Some(
                    serde_json::from_value::<Field1>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field0 = if element_count > 0 {
                Some(
                    serde_json::from_value::<Field0>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };

            Ok(Self {
                from_address: field0.map(|f| f.value),
                keys: field1.map(|f| f.value),
                block_id: field2.map(|f| f.value),
                finality_status: field3.map(|f| f.value),
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                from_address: object.from_address.map(|f| f.value),
                keys: object.keys.map(|f| f.value),
                block_id: object.block_id.map(|f| f.value),
                finality_status: object.finality_status.map(|f| f.value),
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscribeNewHeadsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field0<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: self.block_id.as_ref().map(|f| Field0 { value: f }),
            },
            serializer,
        )
    }
}

impl Serialize for SubscribeNewHeadsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field0<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: self.block_id.as_ref().map(|f| Field0 { value: f }),
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscribeNewHeadsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            block_id: Option<Field0>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: ConfirmedBlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let element_count = elements.len();

            let field0 = if element_count > 0 {
                Some(
                    serde_json::from_value::<Field0>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };

            Ok(Self {
                block_id: field0.map(|f| f.value),
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.map(|f| f.value),
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscribeNewTransactionReceiptsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [L2TransactionFinalityStatus],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        AsObject::serialize(
            &AsObject {
                finality_status: self.finality_status.as_ref().map(|f| Field0 { value: f }),
                sender_address: self.sender_address.as_ref().map(|f| Field1 { value: f }),
            },
            serializer,
        )
    }
}

impl Serialize for SubscribeNewTransactionReceiptsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [L2TransactionFinalityStatus],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        AsObject::serialize(
            &AsObject {
                finality_status: self.finality_status.as_ref().map(|f| Field0 { value: f }),
                sender_address: self.sender_address.as_ref().map(|f| Field1 { value: f }),
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscribeNewTransactionReceiptsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: Vec<L2TransactionFinalityStatus>,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "Vec<UfeHex>")]
            pub value: Vec<Felt>,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let element_count = elements.len();

            let field1 = if element_count > 1 {
                Some(
                    serde_json::from_value::<Field1>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field0 = if element_count > 0 {
                Some(
                    serde_json::from_value::<Field0>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };

            Ok(Self {
                finality_status: field0.map(|f| f.value),
                sender_address: field1.map(|f| f.value),
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                finality_status: object.finality_status.map(|f| f.value),
                sender_address: object.sender_address.map(|f| f.value),
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscribeNewTransactionsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [L2TransactionStatus],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        AsObject::serialize(
            &AsObject {
                finality_status: self.finality_status.as_ref().map(|f| Field0 { value: f }),
                sender_address: self.sender_address.as_ref().map(|f| Field1 { value: f }),
            },
            serializer,
        )
    }
}

impl Serialize for SubscribeNewTransactionsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1<'a>>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a [L2TransactionStatus],
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            #[serde_as(as = "[UfeHex]")]
            pub value: &'a [Felt],
        }

        AsObject::serialize(
            &AsObject {
                finality_status: self.finality_status.as_ref().map(|f| Field0 { value: f }),
                sender_address: self.sender_address.as_ref().map(|f| Field1 { value: f }),
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscribeNewTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            finality_status: Option<Field0>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sender_address: Option<Field1>,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: Vec<L2TransactionStatus>,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            #[serde_as(as = "Vec<UfeHex>")]
            pub value: Vec<Felt>,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let element_count = elements.len();

            let field1 = if element_count > 1 {
                Some(
                    serde_json::from_value::<Field1>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };
            let field0 = if element_count > 0 {
                Some(
                    serde_json::from_value::<Field0>(elements.pop().unwrap()).map_err(|err| {
                        serde::de::Error::custom(format!("failed to parse element: {err}"))
                    })?,
                )
            } else {
                None
            };

            Ok(Self {
                finality_status: field0.map(|f| f.value),
                sender_address: field1.map(|f| f.value),
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                finality_status: object.finality_status.map(|f| f.value),
                sender_address: object.sender_address.map(|f| f.value),
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscribeTransactionStatusRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscribeTransactionStatusRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscribeTransactionStatusRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionEventsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a EmittedEventWithFinality,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionEventsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a EmittedEventWithFinality,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionEventsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: EmittedEventWithFinality,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionNewHeadsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockHeader,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionNewHeadsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a BlockHeader,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionNewHeadsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: BlockHeader,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionNewTransactionReceiptsRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a TransactionReceiptWithBlockInfo,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionNewTransactionReceiptsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a TransactionReceiptWithBlockInfo,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionNewTransactionReceiptsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: TransactionReceiptWithBlockInfo,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionNewTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a TransactionWithL2Status,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionNewTransactionRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a TransactionWithL2Status,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionNewTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: TransactionWithL2Status,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionReorgRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a ReorgData,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionReorgRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a ReorgData,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionReorgRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: ReorgData,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for SubscriptionTransactionStatusRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a NewTransactionStatus,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
                result: Field1 {
                    value: &self.result,
                },
            },
            serializer,
        )
    }
}

impl Serialize for SubscriptionTransactionStatusRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
            result: Field1<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field1<'a> {
            pub value: &'a NewTransactionStatus,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
                result: Field1 { value: self.result },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for SubscriptionTransactionStatusRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
            result: Field1,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field1 {
            pub value: NewTransactionStatus,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field1 = serde_json::from_value::<Field1>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
                result: field1.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
                result: object.result.value,
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
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: &self.block_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for TraceBlockTransactionsRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            block_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a ConfirmedBlockId,
        }

        AsObject::serialize(
            &AsObject {
                block_id: Field0 {
                    value: self.block_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for TraceBlockTransactionsRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            block_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: ConfirmedBlockId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                block_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                block_id: object.block_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for TraceTransactionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: &self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl Serialize for TraceTransactionRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            transaction_hash: Field0<'a>,
        }

        #[serde_as]
        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            #[serde_as(as = "UfeHex")]
            pub value: &'a Felt,
        }

        AsObject::serialize(
            &AsObject {
                transaction_hash: Field0 {
                    value: self.transaction_hash,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for TraceTransactionRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            transaction_hash: Field0,
        }

        #[serde_as]
        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            #[serde_as(as = "UfeHex")]
            pub value: Felt,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                transaction_hash: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                transaction_hash: object.transaction_hash.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}

impl Serialize for UnsubscribeRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: &self.subscription_id,
                },
            },
            serializer,
        )
    }
}

impl Serialize for UnsubscribeRequestRef<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct AsObject<'a> {
            subscription_id: Field0<'a>,
        }

        #[derive(Serialize)]
        #[serde(transparent)]
        struct Field0<'a> {
            pub value: &'a SubscriptionId,
        }

        AsObject::serialize(
            &AsObject {
                subscription_id: Field0 {
                    value: self.subscription_id,
                },
            },
            serializer,
        )
    }
}

impl<'de> Deserialize<'de> for UnsubscribeRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct AsObject {
            subscription_id: Field0,
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Field0 {
            pub value: SubscriptionId,
        }

        let temp = serde_json::Value::deserialize(deserializer)?;

        if let Ok(mut elements) = Vec::<serde_json::Value>::deserialize(&temp) {
            let field0 = serde_json::from_value::<Field0>(
                elements
                    .pop()
                    .ok_or_else(|| serde::de::Error::custom("invalid sequence length"))?,
            )
            .map_err(|err| serde::de::Error::custom(format!("failed to parse element: {err}")))?;

            Ok(Self {
                subscription_id: field0.value,
            })
        } else if let Ok(object) = AsObject::deserialize(&temp) {
            Ok(Self {
                subscription_id: object.subscription_id.value,
            })
        } else {
            Err(serde::de::Error::custom("invalid sequence length"))
        }
    }
}
