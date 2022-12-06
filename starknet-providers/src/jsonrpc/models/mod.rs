use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::{byte_array::base64, unsigned_field_element::UfeHex},
    types::FieldElement,
};

pub use starknet_core::types::L1Address as EthAddress;

// Not exposed by design
mod serde_impls;
use serde_impls::NumAsHex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxHashes {
    Block(BlockWithTxHashes),
    PendingBlock(PendingBlockWithTxHashes),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxs {
    Block(BlockWithTxs),
    PendingBlock(PendingBlockWithTxs),
}

/// (`EMITTED_EVENT`) An event emitted as a result of transaction execution. Event information
/// decorated with metadata on where it was emitted.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedEvent {
    /// The hash of the block in which the event was emitted
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The number of the block in which the event was emitted
    pub block_number: u64,
    /// The transaction that emitted the event
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

/// (`EVENT`) A StarkNet event
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

/// Repesents params passed to starknet_getEvents. Contains flattened properties from
/// (`EVENT_FILTER`) and (`RESULT_PAGE_REQUEST`) from the spec
#[serde_as]
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<FieldElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<FieldElement>>,
    /// A pointer to the last element of the delivered page, use this token in a subsequent query
    /// to obtain the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    pub chunk_size: u64,
}

/// Represents the result returned from starknet_getEvents. Contains all the event objects matching
/// the filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsPage {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// A pointer to the last element of the delivered page, use this token in a subsequent query
    /// to obtain the next page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// (`BLOCK_ID`) Block hash, number or tag
#[derive(Debug, Clone)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

/// (`BLOCK_TAG`) A tag specifying a dynamic reference to a block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockTag {
    Latest,
    Pending,
}

#[derive(Debug, Clone)]
pub enum SyncStatusType {
    Syncing(SyncStatus),
    NotSyncing,
}

/// (`SYNC_STATUS`) An object describing the node synchronization status
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// (`STATE_UPDATE`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateUpdate {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: FieldElement,
    pub state_diff: StateDiff,
}

/// The change in state applied in this block, given as a mapping of addresses to the new values
/// and/or new contracts
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDiff {
    pub storage_diffs: Vec<ContractStorageDiffItem>,
    /// The hashes of new contracts declared as part of the new state
    #[serde_as(as = "Vec<UfeHex>")]
    pub declared_contract_hashes: Vec<FieldElement>,
    pub deployed_contracts: Vec<DeployedContractItem>,
    pub nonces: Vec<NonceUpdate>,
}

/// (`CONTRACT_STORAGE_DIFF_ITEM`) The changes in the storage per contract address
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The changes in the storage of the contract
    pub storage_entries: Vec<StorageEntry>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    /// The key of the changed value
    #[serde_as(as = "UfeHex")]
    pub key: FieldElement,
    /// The new value applied to the given address
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
}

/// A new contract deployed as part of the new state
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedContractItem {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The hash of the contract code
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// The updated nonce per contract address
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonceUpdate {
    /// The address of the contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The nonce for the given address at the end of the block
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

/// (`BLOCK_WITH_TX_HASHES`) The block object
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// The StarkNet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
}

/// (`BLOCK_WITH_TXS`) The block object
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// The StarkNet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
}

/// (`PENDING_BLOCK_WITH_TX_HASHES`) The dynamic block being constructed by the sequencer. Note
/// that this object will be deprecated upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingBlockWithTxHashes {
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The StarkNet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
}

/// (`PENDING_BLOCK_WITH_TXS`) The dynamic block being constructed by the sequencer. Note that
/// this object will be deprecated upon decentralization.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingBlockWithTxs {
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
    /// The time in which the block was created, encoded in Unix time
    pub timestamp: u64,
    /// The StarkNet identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    pub sequencer_address: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
}

/// (`TXN`) Transaction. The transaction schema, as it appears inside a block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Transaction {
    Invoke(InvokeTransaction),
    Declare(DeclareTransaction),
    Deploy(DeployTransaction),
    L1Handler(L1HandlerTransaction),
    DeployAccount(DeployAccountTransaction),
}

/// (`DEPLOY_ACCOUNT_TXN`) Deploy Account Transaction. Deploys an account contract, charges fee
/// from the pre-funded account addresses
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// (`L1_HANDLER_TXN`) L1 -> L2 message transaction. A call to an l1_handler on an L2 contract
/// induced by a message from L1
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1HandlerTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    /// The L1 -> L2 message nonce field of the SN Core L1 contract at the time the transaction was
    /// sent
    #[serde_as(as = "NumAsHex")]
    pub nonce: u64,
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
}

/// (`DECLARE_TXN`) Declare Contract Transaction
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
}

/// (`DEPLOY_TXN`) Deploy Contract Transaction. The structure of a deploy transaction. Note that
/// this transaction type is deprecated and will no longer be supported in future versions.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The hash of the deployed contract's class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
}

/// (`INVOKE_TXN`) Initiate a transaction from an account
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    #[serde(flatten)]
    pub versioned_properties: InvokeTransactionProperties,
}

/// (`INVOKE_TXN_RECEIPT`) Invoke Transaction Receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
}

/// (`DECLARE_TXN_RECEIPT`) Declare Transaction Receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
}

/// (`DEPLOY_ACCOUNT_TXN_RECEIPT`) Deploy Account Transaction Receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// (`DEPLOY_TXN_RECEIPT`) Deploy Transaction Receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// (`L1_HANDLER_TXN_RECEIPT`) Receipt for l1 handler transaction
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1HandlerTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
}

/// (`TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionReceipt {
    Invoke(InvokeTransactionReceipt),
    L1Handler(L1HandlerTransactionReceipt),
    Declare(DeclareTransactionReceipt),
    Deploy(DeployTransactionReceipt),
    DeployAccount(DeployAccountTransactionReceipt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingTransactionReceipt {
    TransactionReceipt(TransactionReceipt),
    PendingTransactionReceipt(PendingTransactionReceipt),
}

/// (`PENDING_COMMON_RECEIPT_PROPERTIES`) Common properties for a pending transaction receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingInvokeOrDeclareTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    #[serde(default, rename = "type")]
    pub transaction_type: Option<TransactionType>,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// (`PENDING_DEPLOY_TXN_RECEIPT`) Pending deploy transaction receipt
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDeployTransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    #[serde(default, rename = "type")]
    pub transaction_type: Option<TransactionType>,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// (`PENDING_TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PendingTransactionReceipt {
    Deploy(PendingDeployTransactionReceipt),
    /// Used for pending invoke and declare transaction receipts
    InvokeOrDeclare(PendingInvokeOrDeclareTransactionReceipt),
}

/// (`MESSAGE_TO_L1`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgToL1 {
    /// The target L1 address the message is sent to
    #[serde_as(as = "UfeHex")]
    pub to_address: FieldElement,
    /// The payload of the message
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

/// (`TXN_STATUS`) The status of the transaction. May be unknown in case node is not aware of it
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    AcceptedOnL2,
    AcceptedOnL1,
    Rejected,
}

/// (`BLOCK_STATUS`) The status of the block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockStatus {
    Pending,
    AcceptedOnL2,
    AcceptedOnL1,
    Rejected,
}

/// (`FUNCTION_CALL`) Function call information
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
}

/// (`CONTRACT_CLASS`) The definition of a StarkNet contract class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: EntryPointsByType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<ContractAbiEntry>>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EntryPointsByType {
    pub constructor: Vec<ContractEntryPoint>,
    pub external: Vec<ContractEntryPoint>,
    pub l1_handler: Vec<ContractEntryPoint>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEntryPoint {
    /// The offset of the entry point in the program
    #[serde_as(as = "NumAsHex")]
    pub offset: u64,
    /// A unique identifier of the entry point (function) in the program
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
}

/// (`FEE_ESTIMATE`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    /// The Ethereum gas cost of the transaction
    /// (see https://docs.starknet.io/docs/Fees/fee-mechanism for more info)
    #[serde_as(as = "UfeHex")]
    pub gas_consumed: FieldElement,
    /// The gas price (in gwei) that was used in the cost estimation
    #[serde_as(as = "UfeHex")]
    pub gas_price: FieldElement,
    /// The estimated fee for the transaction (in gwei), product of gas_consumed and gas_price
    #[serde_as(as = "UfeHex")]
    pub overall_fee: FieldElement,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
}

/// (`BROADCASTED_TXN`) The transaction's representation when it's sent to the sequencer (but not
/// yet in a block)
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BroadcastedTransaction {
    Invoke(BroadcastedInvokeTransaction),
    Declare(BroadcastedDeclareTransaction),
    Deploy(BroadcastedDeployTransaction),
    DeployAccount(BroadcastedDeployAccountTransaction),
}

/// Possible values for version are 0 or 1. This may effect the possible parameters for StarkNet
/// RPCs and their resulting transaction hashes. (see
/// https://docs.starknet.io/documentation/architecture_and_concepts/Blocks/transactions/ for more
/// info)
#[serde_as]
#[derive(Debug, Clone)]
pub enum TransactionVersion {
    V0,
    V1,
}

/// (`BROADCASTED_INVOKE_TXN`) Mempool representation of an invoke transaction
#[derive(Debug, Clone)]
pub struct BroadcastedInvokeTransaction {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub versioned_properties: InvokeTransactionProperties,
}

/// (`BROADCASTED_DECLARE_TXN`) Mempool representation of a declare transaction.
#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct BroadcastedDeclareTransaction {
    pub version: TransactionVersion,
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    /// The class to be declared
    pub contract_class: ContractClass,
    /// The address of the account contract sending the declaration transaction
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
}

/// (`BROADCASTED_DEPLOY_TXN`) Mempool representation of a deploy transaction. The structure of a
/// deploy transaction. Note that this transaction type is deprecated and will no longer be
/// supported in future versions
#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct BroadcastedDeployTransaction {
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    /// The class of the contract that will be deployed
    pub contract_class: ContractClass,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
}

/// (`BROADCASTED_DEPLOY_ACCOUNT_TXN`) Mempool representation of a deploy account transaction
#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct BroadcastedDeployAccountTransaction {
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    pub version: TransactionVersion,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
    /// The hash of the deployed contract's class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

/// Represents versioned invoke transaction properties. Possible version values are 0 or 1
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InvokeTransactionProperties {
    /// Version 0 invoke transaction properties
    V0 {
        #[serde_as(as = "UfeHex")]
        contract_address: FieldElement,
        #[serde_as(as = "UfeHex")]
        entry_point_selector: FieldElement,
        /// The parameters passed to the function
        #[serde_as(as = "Vec<UfeHex>")]
        calldata: Vec<FieldElement>,
    },
    /// Version 1 invoke transaction properties
    V1 {
        #[serde_as(as = "UfeHex")]
        sender_address: FieldElement,
        /// The data expected by the account's `execute` function (in most usecases, this includes
        /// the called contract address and a function selector)
        #[serde_as(as = "Vec<UfeHex>")]
        calldata: Vec<FieldElement>,
    },
}

/// (`TXN_TYPE`) The type of the transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Declare,
    Deploy,
    DeployAccount,
    Invoke,
    L1Handler,
}

/// (`CONTRACT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContractAbiEntry {
    Struct(StructAbiEntry),
    Event(EventAbiEntry),
    Function(FunctionAbiEntry),
    L1Handler(FunctionAbiEntry),
    Constructor(FunctionAbiEntry),
}

/// (`STRUCT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructAbiEntry {
    /// The struct name
    pub name: String,
    pub size: u64,
    pub members: Vec<StructMember>,
}

/// (`STRUCT_MEMBER`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructMember {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    #[serde(rename = "type")]
    pub parameter_type: String,
    /// Offset of this property within the struct
    pub offset: u64,
}

/// (`EVENT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAbiEntry {
    /// The event name
    pub name: String,
    pub keys: Vec<TypedParameter>,
    pub data: Vec<TypedParameter>,
}

/// (`FUNCTION_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionAbiEntry {
    /// The function name
    pub name: String,
    pub inputs: Vec<TypedParameter>,
    pub outputs: Vec<TypedParameter>,
}

/// (`TYPED PARAMETER`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedParameter {
    /// The parameter's name
    pub name: String,
    /// The parameter's type
    #[serde(rename = "type")]
    pub parameter_type: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionResult {
    /// The hash of the invoke transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareTransactionResult {
    /// The hash of the declare transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransactionResult {
    /// The hash of the deploy transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The address of the new contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct BlockHash {
            #[serde_as(as = "UfeHex")]
            block_hash: FieldElement,
        }

        #[derive(Serialize)]
        struct BlockNumber {
            block_number: u64,
        }

        match self {
            Self::Hash(hash) => BlockHash::serialize(&BlockHash { block_hash: *hash }, serializer),
            Self::Number(number) => BlockNumber::serialize(
                &BlockNumber {
                    block_number: *number,
                },
                serializer,
            ),
            Self::Tag(tag) => BlockTag::serialize(tag, serializer),
        }
    }
}

impl Serialize for BroadcastedInvokeTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            version: TransactionVersion,
            #[serde_as(as = "UfeHex")]
            pub max_fee: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            pub signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            pub nonce: &'a FieldElement,
            #[serde(flatten)]
            pub versioned_properties: &'a InvokeTransactionProperties,
        }

        let versioned = match &self.versioned_properties {
            InvokeTransactionProperties::V0 {
                contract_address: _,
                entry_point_selector: _,
                calldata: _,
            } => Versioned {
                version: TransactionVersion::V0,
                max_fee: &self.max_fee,
                signature: &self.signature,
                nonce: &self.nonce,
                versioned_properties: &self.versioned_properties,
            },
            InvokeTransactionProperties::V1 {
                sender_address: _,
                calldata: _,
            } => Versioned {
                version: TransactionVersion::V1,
                max_fee: &self.max_fee,
                signature: &self.signature,
                nonce: &self.nonce,
                versioned_properties: &self.versioned_properties,
            },
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for TransactionVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            TransactionVersion::V0 => serializer.serialize_str(&format!("{:#x}", 0_u64)),
            TransactionVersion::V1 => serializer.serialize_str(&format!("{:#x}", 1_u64)),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match u64::from_str_radix(&value[2..], 16) {
            Ok(value) => match value {
                0 => Ok(TransactionVersion::V0),
                1 => Ok(TransactionVersion::V1),
                _ => Err(serde::de::Error::custom(format!(
                    "unsupported transaction version: {}",
                    value
                ))),
            },
            Err(err) => Err(serde::de::Error::custom(format!(
                "invalid hex string: {}",
                err
            ))),
        }
    }
}

impl AsRef<FunctionCall> for FunctionCall {
    fn as_ref(&self) -> &FunctionCall {
        self
    }
}

impl AsRef<BroadcastedTransaction> for BroadcastedTransaction {
    fn as_ref(&self) -> &BroadcastedTransaction {
        self
    }
}
