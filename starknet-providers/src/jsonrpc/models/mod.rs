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
pub struct EventsPage {
    pub events: Vec<EmittedEvent>,
    /// The returned page number
    pub page_number: u64,
    /// A flag indicating whether this is the end of the stream of events
    pub is_last_page: bool,
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

/// The status of the block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockStatus {
    Pending,
    Proven,
    AcceptedOnL2,
    AcceptedOnL1,
    Rejected,
}

/// Function call information
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

/// The definition of a StarkNet contract class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClass {
    /// A base64 representation of the compressed program code
    #[serde(with = "base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: EntryPointsByType,
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

/// An event filter/query
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventFilter {
    #[serde(rename = "fromBlock", skip_serializing_if = "Option::is_none")]
    pub from_block: Option<u64>,
    #[serde(rename = "toBlock", skip_serializing_if = "Option::is_none")]
    pub to_block: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub address: Option<FieldElement>,
    /// The values used to filter the events
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub keys: Option<Vec<FieldElement>>,
}

/// Block hash or tag
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockHashOrTag {
    Hash(#[serde_as(as = "UfeHex")] FieldElement),
    Tag(BlockTag),
}

/// Block number or tag
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockNumOrTag {
    Number(u64),
    Tag(BlockTag),
}

/// A tag specifying a dynamic reference to a block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockTag {
    Latest,
    // The current spec doesn't allow `pending` but is probably inaccurate:
    //   https://github.com/starkware-libs/starknet-specs/blob/bcce12075ef4cde19fc62b47ed2162292e0ed70d/api/starknet_api_openrpc.json#L821-L828
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(flatten)]
    pub metadata: BlockMeta,
    #[serde_as(as = "Vec<UfeHex>")]
    pub transactions: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockWithTxns {
    #[serde(flatten)]
    pub metadata: BlockMeta,
    pub transactions: Vec<Transaction>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockWithReceipts {
    #[serde(flatten)]
    pub metadata: BlockMeta,
    pub transactions: Vec<TransactionWithReceipt>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMeta {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    /// The hash of this block's parent
    #[serde_as(as = "UfeHex")]
    pub parent_hash: FieldElement,
    /// The block number (its height)
    pub block_number: u64,
    pub status: BlockStatus,
    /// The identity of the sequencer submitting this block
    #[serde_as(as = "UfeHex")]
    // This should be `ETH_ADDRESS` according to spec but it seems to be wrong
    pub sequencer: FieldElement,
    /// The new global state root
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
    /// The previous global state root
    #[serde_as(as = "UfeHex")]
    pub old_root: FieldElement,
    /// When the block was accepted on L1. Formatted as...
    pub accepted_time: u64,
}

// Cannot use `#[serde(flatten)]` here due to `tx_hash` field collision, so unfortunately we have
// to write duplicate code.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionWithReceipt {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    // None for deploy txs
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub entry_point_selector: Option<FieldElement>,
    // None for deploy txs
    #[serde(default)]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub calldata: Option<Vec<FieldElement>>,
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub txn_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction (None for non-invoke
    /// transactions).
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub max_fee: Option<FieldElement>,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    /// Extra information pertaining to the status
    pub status_data: String,
    pub messages_sent: Vec<MsgToL1>,
    /// In case this transaction was an L1 handler, this is the original message that invoked it
    pub l1_origin_message: Option<MsgToL2>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

// Manually porting the `FunctionCall` fields here instead of flattening like the specification
// suggests because `entry_point_selector` and `calldata` can be `None` for non-invoke transactions
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    // None for deploy txs
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub entry_point_selector: Option<FieldElement>,
    // None for deploy txs
    #[serde(default)]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub calldata: Option<Vec<FieldElement>>,
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub txn_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction (None for non-invoke
    /// transactions).
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub max_fee: Option<FieldElement>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub txn_hash: FieldElement,
    /// The fee that was charged by the sequencer
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
    pub status: TransactionStatus,
    /// Extra information pertaining to the status
    pub status_data: String,
    pub messages_sent: Vec<MsgToL1>,
    /// In case this transaction was an L1 handler, this is the original message that invoked it
    pub l1_origin_message: Option<MsgToL2>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgToL2 {
    /// The originating L1 contract that sent the message
    pub from_address: EthAddress,
    /// The payload of the meesage. The call data to the L1 handler
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

/// The status of the transaction. May be unknown in case node is not aware of it
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Unknown,
    Received,
    Pending,
    AcceptedOnL2,
    AcceptedOnL1,
    Rejected,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    /// The Ethereum gas cost of the transaction
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

impl AsRef<FunctionCall> for FunctionCall {
    fn as_ref(&self) -> &FunctionCall {
        self
    }
}
