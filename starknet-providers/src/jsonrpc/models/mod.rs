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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingTransactionReceipt {
    Receipt(TransactionReceipt),
    PendingReceipt(PendingTransactionReceipt),
}

/// An event emitted as a result of transaction execution
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
    /// The event information
    #[serde(flatten)]
    pub event: Event,
}

/// A StarkNet event
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde(flatten)]
    pub content: EventContent,
}

/// The content of an event
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContent {
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

/// An event filter/query
#[serde_as]
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub address: Option<FieldElement>,
    /// The values used to filter the events
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub keys: Option<Vec<FieldElement>>,
}

/// Block hash, number or tag
#[derive(Debug, Clone)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

/// A tag specifying a dynamic reference to a block
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

/// An object describing the node synchronization status
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
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
}

/// The block object
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockWithTxHashes {
    pub status: BlockStatus,
    #[serde(flatten)]
    pub header: BlockHeader,
    /// The hashes of the transactions included in this block
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
}

/// The block object
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockWithTxs {
    pub status: BlockStatus,
    #[serde(flatten)]
    pub header: BlockHeader,
    /// The transactions in this block
    pub transactions: Vec<Transaction>,
}

/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
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

/// The dynamic block being constructed by the sequencer. Note that this object will be deprecated
/// upon decentralization.
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

/// The changes in the storage per contract address
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStorageDiffItem {
    /// The contract address for which the storage changed
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    pub storage_entries: Vec<StorageEntry>,
}

/// The changes in the storage of the contract
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

/// Transaction (`TXN`) The transaction schema, as it appears inside a block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Transaction {
    Invoke(InvokeTransaction),
    Declare(DeclareTransaction),
    Deploy(DeployTransaction),
    L1Handler(L1HandlerTransaction),
    DeployAccount(DeployAccountTransaction),
}

/// The `COMMON_TXN_PROPERTIES` type in the specification
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMeta {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde(flatten)]
    pub common_properties: BroadcastedTransactionCommonProperties,
}

/// Deploys an account contract, charges fee from the pre-funded account addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransaction {
    #[serde(flatten)]
    pub meta: TransactionMeta,
    #[serde(flatten)]
    pub deploy_properties: DeployAccountTransactionProperties,
}

/// l1-->l2 message transaction (`L1_HANDLER_TXN`)
/// A call to an l1_handler on an L2 contract induced by a message from L1
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1HandlerTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// Version of the transaction scheme
    #[serde_as(as = "NumAsHex")]
    pub version: u64,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    /// The L1->L2 message nonce field of the SN Core L1 contract at the time the transaction was
    /// sent
    #[serde_as(as = "NumAsHex")]
    pub nonce: u64,
    #[serde(flatten)]
    pub function_call: FunctionCall,
}

/// Declare Contract Transaction (`DECLARE_TXN`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareTransaction {
    #[serde(flatten)]
    pub meta: TransactionMeta,
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
}

/// Deploy Contract Transaction (`DEPLOY_TXN`)
///
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will
/// no longer be supported in future versions.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransaction {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The hash of the deployed contract's class
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde(flatten)]
    pub deploy_transaction_properties: DeployTransactionProperties,
}

/// Initiate a transaction from an account (`INVOKE_TXN`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransaction {
    #[serde(flatten)]
    pub meta: TransactionMeta,
    #[serde(flatten)]
    pub invoke_transaction: InvokeTransactionVersion,
}

/// Common properties for a transaction receipt (`COMMON_RECEIPT_PROPERTIES`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceiptMeta {
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
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub messages_sent: Vec<MsgToL1>,
    pub events: Vec<Event>,
}

/// Invoke Transaction Receipt (`INVOKE_TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
}

/// Declare Transaction Receipt (`DECLARE_TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
}

/// Deploy Account Transaction Receipt (`DEPLOY_ACCOUNT_TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransactionReceipt {
    #[serde(flatten)]
    pub meta: DeployTransactionReceipt,
}

/// Deploy Transaction Receipt (`DEPLOY_TXN_RECEIPT`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// receipt for l1 handler transaction (`L1_HANDLER_TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1HandlerTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
}

/// (`TXN_RECEIPT`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionReceipt {
    Invoke(InvokeTransactionReceipt),
    L1Handler(L1HandlerTransactionReceipt),
    Declare(DeclareTransactionReceipt),
    Deploy(DeployTransactionReceipt),
    DeployAccount(DeployAccountTransactionReceipt),
    PendingTransaction(PendingTransactionReceipt),
}

/// Common properties for a pending transaction receipt (`PENDING_COMMON_RECEIPT_PROPERTIES`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransactionReceiptMeta {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionType>,
    pub messages_sent: Vec<MsgToL1>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Pending deploy transaction receipt (`PENDING_DEPLOY_TXN_RECEIPT`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDeployTransactionReceipt {
    #[serde(flatten)]
    pub meta: PendingTransactionReceiptMeta,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// The `PENDING_TXN_RECEIPT` type in the specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PendingTransactionReceipt {
    Deploy(PendingDeployTransactionReceipt),
    /// Used for pending invoke and declare transaction receipts
    TransactionMeta(PendingTransactionReceiptMeta),
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

/// The status of the block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockStatus {
    Pending,
    AcceptedOnL2,
    AcceptedOnL1,
    Rejected,
}

/// Function call information (`FUNCTION_CALL`)
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

/// The definition of a StarkNet contract class (`CONTRACT_CLASS`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: EntryPointsByType,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub abi: Option<ContractABI>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsPage {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// a pointer to the last element of the delivered page, use this token in a subsequent query
    /// to obtain the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
}

/// (`BROADCASTED_TXN`)
/// the transaction's representation when it's sent to the sequencer (but not yet in a block)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastedTransaction {
    Invoke(BroadcastedInvokeTransaction),
    Declare(BroadcastedDeclareTransaction),
    Deploy(BroadcastedDeployTransaction),
    DeployAccount(BroadcastedDeployAccountTransaction),
}

/// (`BROADCASTED_INVOKE_TXN`) mempool representation of an invoke transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastedInvokeTransaction {
    #[serde(flatten)]
    pub common_properties: BroadcastedTransactionCommonProperties,
    #[serde(flatten)]
    pub invoke_transaction: InvokeTransactionVersion,
}

/// (`BROADCASTED_DECLARE_TXN`) mempool representation of a declare transaction
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastedDeclareTransaction {
    #[serde(flatten)]
    pub common_properties: BroadcastedTransactionCommonProperties,
    /// The class to be declared
    pub contract_class: ContractClass,
    /// The address of the account contract sending the declaration transaction
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
}

/// (`BROADCASTED_DEPLOY_TXN`) mempool representation of a deploy transaction
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will
/// no longer be supported in future versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastedDeployTransaction {
    /// The class of the contract that will be deployed
    pub contract_class: ContractClass,
    #[serde(flatten)]
    pub deploy_properties: DeployTransactionProperties,
}

/// (`BROADCASTED_DEPLOY_ACCOUNT_TXN`) Mempool representation of a deploy account transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastedDeployAccountTransaction {
    #[serde(flatten)]
    pub common_properties: BroadcastedTransactionCommonProperties,
    #[serde(flatten)]
    pub deploy_account_properties: DeployAccountTransactionProperties,
}

/// (`DEPLOY_TXN_PROPERTIES`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTransactionProperties {
    /// Version of the transaction scheme
    #[serde_as(as = "NumAsHex")]
    pub version: u64,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
}
/// (`DEPLOY_ACCOUNT_TXN_PROPERTIES`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransactionProperties {
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

/// (`BROADCASTED_TXN_COMMON_PROPERTIES`)
/// common properties of a transaction that is sent to the sequencer (but is not yet in a block)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastedTransactionCommonProperties {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    #[serde_as(as = "NumAsHex")]
    pub version: u64,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InvokeTransactionVersion {
    V0(FunctionCall),
    V1(InvokeTransactionV1),
}

/// version 1 invoke transaction (`INVOKE_TXN_V1`)
/// initiates a transaction from a given account
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionV1 {
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
    /// The data expected by the account's `execute` function (in most usecases, this includes the
    /// called contract address and a function selector)
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
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

/// (`CONTRACT_ABI`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractABI {
    pub entries: Vec<ContractABIEntry>,
}

/// (`CONTRACT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractABIEntry {
    Struct(StructABIEntry),
    Event(EventABIEntry),
    Function(FunctionABIEntry),
}

/// (`STRUCT_ABI_TYPE`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructABIType {
    Struct,
}

/// (`EVENT_ABI_TYPE`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventABIType {
    Event,
}

/// (`FUNCTION_ABI_TYPE`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionABIType {
    Function,
    L1Handler,
    Constructor,
}

/// (`STRUCT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructABIEntry {
    #[serde(rename = "type")]
    pub abi_type: StructABIType,
    /// The struct name
    pub name: String,
    pub size: u64,
    pub members: Vec<StructMember>,
}

/// (`STRUCT_MEMBER`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructMember {
    #[serde(flatten)]
    pub typed_param: TypedParameter,
    /// offset of this property within the struct
    pub offset: u64,
}

/// (`EVENT_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventABIEntry {
    #[serde(rename = "type")]
    pub abi_type: EventABIType,
    /// The event name
    pub name: String,
    pub keys: Vec<TypedParameter>,
    pub data: Vec<TypedParameter>,
}

/// (`FUNCTION_ABI_ENTRY`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionABIEntry {
    #[serde(rename = "type")]
    pub abi_type: FunctionABIType,
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

impl AsRef<FunctionCall> for FunctionCall {
    fn as_ref(&self) -> &FunctionCall {
        self
    }
}
