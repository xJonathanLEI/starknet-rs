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
    // Using `fromBlock` instead of `from_block` for now due to pathfinder bug:
    //   https://github.com/eqlabs/pathfinder/issues/536
    #[serde(rename = "fromBlock", skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    // Using `toBlock` instead of `to_block` for now due to pathfinder bug:
    //   https://github.com/eqlabs/pathfinder/issues/536
    #[serde(rename = "toBlock", skip_serializing_if = "Option::is_none")]
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
    /// The change in state applied in this block, given as a mapping of addresses to the new values
    /// and/or new contracts
    pub state_diff: StateDiff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDiff {
    pub storage_diffs: Vec<StorageDiffItem>,
    pub declared_contracts: Vec<DeclaredContractItem>,
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

/// A new contract declared as part of the new state
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredContractItem {
    /// The hash of the contract code
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
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

/// A change in a single storage item
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDiffItem {
    /// The contract address for which the state changed
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    /// The key of the changed value
    #[serde_as(as = "UfeHex")]
    pub key: FieldElement,
    /// The new value applied to the given address
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
}

/// Transaction (`TXN`)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Transaction {
    Invoke(InvokeTransaction),
    Declare(DeclareTransaction),
    Deploy(DeployTransaction),
    L1Handler(L1HandlerTransaction),
}

/// The `COMMON_TXN_PROPERTIES` type in the specification
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMeta {
    /// The hash identifying the transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The maximal fee that can be charged for including the transaction
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Version of the transaction scheme
    #[serde_as(as = "NumAsHex")]
    pub version: u64,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

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
/// The structure of a deploy transaction. Note that this transaction type is deprecated and will no
/// longer be supported in future versions.
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
    #[serde_as(as = "NumAsHex")]
    pub version: u64,
    /// The address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    /// The salt for the address of the deployed contract
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    /// The parameters passed to the constructor
    #[serde_as(as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
}

/// Invoke Contract Transaction (`INVOKE_TXN`)
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransaction {
    #[serde(flatten)]
    pub meta: TransactionMeta,
    /// The function the transaction invokes
    #[serde(flatten)]
    pub function_call: FunctionCall,
}

/// Common properties for a transaction receipt (`COMMON_TXN_PROPERTIES`)
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
    /// Extra information pertaining to the status
    pub status_data: Option<String>,
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
}

/// Properties specific to invoke transaction (`INVOKE_TXN_RECEIPT_PROPERTIES`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionReceiptData {
    pub messages_sent: Vec<MsgToL1>,
    /// In case this transaction was an L1 handler, this is the original message that invoked it
    pub l1_origin_message: Option<MsgToL2>,
    /// The events emitted as part of this transaction
    pub events: Vec<Event>,
}

/// Invoke transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
    #[serde(flatten)]
    pub data: InvokeTransactionReceiptData,
}

/// A special type that covers both `DECLARE_TXN_RECEIPT` and `DEPLOY_TXN_RECEIPT` in the spec.
/// This type exists because there's no way to distinguish between the 2 underlying types over the
/// wire. The issue will be fixed in spec v0.2.0, upon which this type shall be removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclareOrDeployTransactionReceipt {
    #[serde(flatten)]
    pub meta: TransactionReceiptMeta,
}

/// The `TXN_RECEIPT` type in the specification, except without the `PENDING_TXN_RECEIPT` variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionReceipt {
    Invoke(InvokeTransactionReceipt),
    DeclareOrDeploy(DeclareOrDeployTransactionReceipt),
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
}

/// Pending invoke transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingInvokeTransactionReceipt {
    #[serde(flatten)]
    pub meta: PendingTransactionReceiptMeta,
    #[serde(flatten)]
    pub data: InvokeTransactionReceiptData,
}

/// Used for deploy and declare transaction receipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDeclareOrDeployTransactionReceipt {
    #[serde(flatten)]
    pub meta: PendingTransactionReceiptMeta,
}

/// The `PENDING_TXN_RECEIPT` type in the specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PendingTransactionReceipt {
    Invoke(PendingInvokeTransactionReceipt),
    DeclareOrDeploy(PendingDeclareOrDeployTransactionReceipt),
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
    /// The returned page number
    pub page_number: u64,
    /// A flag indicating whether this is the end of the stream of events
    pub is_last_page: bool,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
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
