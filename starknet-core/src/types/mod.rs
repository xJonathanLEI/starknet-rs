use alloc::{string::*, vec::*};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;

pub use starknet_types_core::felt::*;

mod conversions;

mod serde_impls;

/// SNIP-12 typed data.
pub mod typed_data;
pub use typed_data::TypedData;

// TODO: better namespacing of exports?
mod codegen;
pub use codegen::{
    BinaryNode, BlockStatus, BlockTag, BlockWithReceipts, BlockWithTxHashes, BlockWithTxs,
    BroadcastedDeclareTransaction, BroadcastedDeclareTransactionV3,
    BroadcastedDeployAccountTransaction, BroadcastedDeployAccountTransactionV3,
    BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV3, CallType,
    CompressedLegacyContractClass, ContractErrorData, ContractLeafData, ContractStorageDiffItem,
    ContractStorageKeys, ContractsProof, DataAvailabilityMode, DeclareTransactionReceipt,
    DeclareTransactionTrace, DeclareTransactionV0, DeclareTransactionV0Content,
    DeclareTransactionV1, DeclareTransactionV1Content, DeclareTransactionV2,
    DeclareTransactionV2Content, DeclareTransactionV3, DeclareTransactionV3Content,
    DeclaredClassItem, DeployAccountTransactionReceipt, DeployAccountTransactionTrace,
    DeployAccountTransactionV1, DeployAccountTransactionV1Content, DeployAccountTransactionV3,
    DeployAccountTransactionV3Content, DeployTransaction, DeployTransactionContent,
    DeployTransactionReceipt, DeployedContractItem, EdgeNode, EmittedEvent, EntryPointType,
    EntryPointsByType, Event, EventFilter, EventFilterWithPage, EventsChunk, ExecutionResources,
    FeeEstimate, FeePayment, FlattenedSierraClass, FunctionCall, FunctionInvocation,
    FunctionStateMutability, GlobalRoots, InnerCallExecutionResources, InnerContractExecutionError,
    InvokeTransactionReceipt, InvokeTransactionTrace, InvokeTransactionV0,
    InvokeTransactionV0Content, InvokeTransactionV1, InvokeTransactionV1Content,
    InvokeTransactionV3, InvokeTransactionV3Content, L1DataAvailabilityMode, L1HandlerTransaction,
    L1HandlerTransactionContent, L1HandlerTransactionReceipt, L1HandlerTransactionTrace,
    LegacyContractEntryPoint, LegacyEntryPointsByType, LegacyEventAbiEntry, LegacyEventAbiType,
    LegacyFunctionAbiEntry, LegacyFunctionAbiType, LegacyStructAbiEntry, LegacyStructAbiType,
    LegacyStructMember, LegacyTypedParameter, MsgFromL1, MsgToL1, NoTraceAvailableErrorData,
    NonceUpdate, OrderedEvent, OrderedMessage, PendingBlockWithReceipts, PendingBlockWithTxHashes,
    PendingBlockWithTxs, PendingStateUpdate, PriceUnit, ReplacedClassItem, ResourceBounds,
    ResourceBoundsMapping, ResourcePrice, ResultPageRequest, RevertedInvocation,
    SequencerTransactionStatus, SierraEntryPoint, SimulatedTransaction, SimulationFlag,
    SimulationFlagForEstimateFee, StarknetError, StateDiff, StateUpdate, StorageEntry,
    StorageProof, SyncStatus, TransactionExecutionErrorData, TransactionExecutionStatus,
    TransactionFinalityStatus, TransactionReceiptWithBlockInfo, TransactionTraceWithHash,
    TransactionWithReceipt,
};

/// Module containing the [`U256`] type.
pub mod u256;
pub use u256::U256;

/// Module containing the [`EthAddress`] type.
pub mod eth_address;
pub use eth_address::EthAddress;

/// Module containing the [`Hash256`] type.
pub mod hash_256;
pub use hash_256::Hash256;

mod execution_result;
pub use execution_result::ExecutionResult;

mod receipt_block;
pub use receipt_block::ReceiptBlock;

mod msg;
pub use msg::MsgToL2;

mod call;
pub use call::Call;

mod byte_array;
pub use byte_array::ByteArray;

// TODO: move generated request code to `starknet-providers`
/// Module containing JSON-RPC request types.
pub mod requests;

/// Module containing types related to Starknet contracts/classes.
pub mod contract;
pub use contract::ContractArtifact;

/// A block with transaction hashes that may or may not be pending.
///
/// A pending block lacks certain information on the block header compared to a non-pending block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxHashes {
    /// A confirmed, non-pending block.
    Block(BlockWithTxHashes),
    /// A pending block.
    PendingBlock(PendingBlockWithTxHashes),
}

/// A block with full transactions that may or may not be pending.
///
/// A pending block lacks certain information on the block header compared to a non-pending block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxs {
    /// A confirmed, non-pending block.
    Block(BlockWithTxs),
    /// A pending block.
    PendingBlock(PendingBlockWithTxs),
}

/// A block with full transactions and receipts that may or may not be pending.
///
/// A pending block lacks certain information on the block header compared to a non-pending block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithReceipts {
    /// A confirmed, non-pending block.
    Block(BlockWithReceipts),
    /// A pending block.
    PendingBlock(PendingBlockWithReceipts),
}

/// State update of a block that may or may not be pending.
///
/// State update for a pending block lacks certain information compared to that of a non-pending
/// block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingStateUpdate {
    /// The state update is for a confirmed, non-pending block.
    Update(StateUpdate),
    /// The state update is for a pending block.
    PendingUpdate(PendingStateUpdate),
}

/// The hash and number (height) for a block.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    /// The block's hash.
    #[serde_as(as = "UfeHex")]
    pub block_hash: Felt,
    /// The block's number (height).
    pub block_number: u64,
}

/// A Starknet client node's synchronization status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatusType {
    /// The node is synchronizing.
    Syncing(SyncStatus),
    /// The node is not synchronizing.
    NotSyncing,
}

/// A "page" of events in a cursor-based pagniation system.
///
/// This type is usually returned from the `starknet_getEvents` RPC method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventsPage {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// A pointer to the last element of the delivered page, use this token in a subsequent query to
    /// obtain the next page. If the value is `None`, don't add it to the response as clients might
    /// use `contains_key` as a check for the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// Response for broadcasting an `INVOKE` transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvokeTransactionResult {
    /// The hash of the invoke transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
}

/// Response for broadcasting a `DECLARE` transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclareTransactionResult {
    /// The hash of the declare transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    /// The hash of the declared class
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
}

/// Response for broadcasting a `DEPLOY` transaction.
///
/// Note that `DEPLOY` transactions have been deprecated and disabled on all public Starknet
/// networks.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeployTransactionResult {
    /// The hash of the deploy transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    /// The address of the new contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
}

/// Response for broadcasting a `DEPLOY_ACCOUNT` transaction.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeployAccountTransactionResult {
    /// The hash of the deploy transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    /// The address of the new contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
}

/// Block identifier in the form of hash, number or tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockId {
    /// Block hash.
    Hash(Felt),
    /// Block number (height).
    Number(u64),
    /// Block tag.
    Tag(BlockTag),
}

/// A "processed" contract class representation that's circulated in the network. This is different
/// from the class representation of compiler output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractClass {
    /// A "processed" Sierra (Cairo 1) class.
    Sierra(FlattenedSierraClass),
    /// A "processed" legacy (Cairo 0) class.
    Legacy(CompressedLegacyContractClass),
}

/// Represents the status of a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    /// Transaction received and awaiting processing.
    Received,
    /// Transaction rejected due to validation or other reasons.
    Rejected,
    /// Transaction accepted on Layer 2 with a specific execution status.
    AcceptedOnL2(ExecutionResult),
    /// Transaction accepted on Layer 1 with a specific execution status.
    AcceptedOnL1(ExecutionResult),
}

impl TransactionStatus {
    /// Returns `true` if the transaction status is `Received`.
    pub const fn is_received(&self) -> bool {
        matches!(self, Self::Received)
    }

    /// Returns `true` if the transaction status is `Rejected`.
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns `true` if the transaction status is `AcceptedOnL2`.
    pub const fn is_accepted_on_l2(&self) -> bool {
        matches!(self, Self::AcceptedOnL2(_))
    }

    /// Returns `true` if the transaction status is `AcceptedOnL1`.
    pub const fn is_accepted_on_l1(&self) -> bool {
        matches!(self, Self::AcceptedOnL1(_))
    }
}

/// A Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum Transaction {
    /// An `INVOKE` transaction.
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransaction),
    /// An `L1_HANDLER` transaction.
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransaction),
    /// A `DECLARE` transaction.
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransaction),
    /// A `DEPLOY` transaction.
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransaction),
    /// A `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransaction),
}

/// Content of a Starknet transaction without an annotated transaction hash.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionContent {
    /// An `INVOKE` transaction.
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransactionContent),
    /// An `L1_HANDLER` transaction.
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransactionContent),
    /// A `DECLARE` transaction.
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransactionContent),
    /// A `DEPLOY` transaction.
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransactionContent),
    /// A `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransactionContent),
}

/// A Starknet transaction in its "mempool" representation that's broadcast by a client.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastedTransaction {
    /// An `INVOKE` transaction.
    #[serde(rename = "INVOKE")]
    Invoke(BroadcastedInvokeTransaction),
    /// A `DECLARE` transaction.
    #[serde(rename = "DECLARE")]
    Declare(BroadcastedDeclareTransaction),
    /// A `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(BroadcastedDeployAccountTransaction),
}

/// An `INVOKE` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum InvokeTransaction {
    /// Version 0 `INVOKE` transaction.
    #[serde(rename = "0x0")]
    V0(InvokeTransactionV0),
    /// Version 1 `INVOKE` transaction.
    #[serde(rename = "0x1")]
    V1(InvokeTransactionV1),
    /// Version 3 `INVOKE` transaction.
    #[serde(rename = "0x3")]
    V3(InvokeTransactionV3),
}

/// Content of an `INVOKE` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum InvokeTransactionContent {
    /// Version 0 `INVOKE` transaction.
    #[serde(rename = "0x0")]
    V0(InvokeTransactionV0Content),
    /// Version 1 `INVOKE` transaction.
    #[serde(rename = "0x1")]
    V1(InvokeTransactionV1Content),
    /// Version 3 `INVOKE` transaction.
    #[serde(rename = "0x3")]
    V3(InvokeTransactionV3Content),
}

/// A `DECLARE` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeclareTransaction {
    /// Version 0 `DECLARE` transaction.
    #[serde(rename = "0x0")]
    V0(DeclareTransactionV0),
    /// Version 1 `DECLARE` transaction.
    #[serde(rename = "0x1")]
    V1(DeclareTransactionV1),
    /// Version 2 `DECLARE` transaction.
    #[serde(rename = "0x2")]
    V2(DeclareTransactionV2),
    /// Version 3 `DECLARE` transaction.
    #[serde(rename = "0x3")]
    V3(DeclareTransactionV3),
}

/// Content of a `DECLARE` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeclareTransactionContent {
    /// Version 0 `DECLARE` transaction.
    #[serde(rename = "0x0")]
    V0(DeclareTransactionV0Content),
    /// Version 1 `DECLARE` transaction.
    #[serde(rename = "0x1")]
    V1(DeclareTransactionV1Content),
    /// Version 2 `DECLARE` transaction.
    #[serde(rename = "0x2")]
    V2(DeclareTransactionV2Content),
    /// Version 3 `DECLARE` transaction.
    #[serde(rename = "0x3")]
    V3(DeclareTransactionV3Content),
}

/// A `DEPLOY_ACCOUNT` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeployAccountTransaction {
    /// Version 1 `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "0x1")]
    V1(DeployAccountTransactionV1),
    /// Version 3 `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "0x3")]
    V3(DeployAccountTransactionV3),
}

/// Content of a `DEPLOY_ACCOUNT` Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeployAccountTransactionContent {
    /// Version 1 `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "0x1")]
    V1(DeployAccountTransactionV1Content),
    /// Version 3 `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "0x3")]
    V3(DeployAccountTransactionV3Content),
}

/// Starknet transaction receipt containing execution results.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionReceipt {
    /// Receipt for an `INVOKE` transaction.
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransactionReceipt),
    /// Receipt for an `L1_HANDLER` transaction.
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransactionReceipt),
    /// Receipt for a `DECLARE` transaction.
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransactionReceipt),
    /// Receipt for a `DEPLOY` transaction.
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransactionReceipt),
    /// Receipt for a `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransactionReceipt),
}

/// ABI entry item for legacy (Cairo 0) contract classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyContractAbiEntry {
    /// ABI entry representing a Cairo function.
    Function(LegacyFunctionAbiEntry),
    /// ABI entry representing a Starknet event.
    Event(LegacyEventAbiEntry),
    /// ABI entry representing a Cairo struct.
    Struct(LegacyStructAbiEntry),
}

/// Execution trace of a Starknet transaction.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionTrace {
    /// Trace for an `INVOKE` transaction.
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransactionTrace),
    /// Trace for a `DEPLOY_ACCOUNT` transaction.
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransactionTrace),
    /// Trace for an `L1_HANDLER` transaction.
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransactionTrace),
    /// Trace for a `DECLARE` transaction.
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransactionTrace),
}

/// The execution result of a function invocation.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum ExecuteInvocation {
    /// Successful invocation.
    Success(FunctionInvocation),
    /// Failed and reverted invocation.
    Reverted(RevertedInvocation),
}

/// A node in the Merkle-Patricia tree, can be a leaf, binary node, or an edge node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MerkleNode {
    /// Binary/branch node.
    BinaryNode(BinaryNode),
    /// Edge/leaf node.
    EdgeNode(EdgeNode),
}

/// Structured error that can later be processed by wallets or sdks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractExecutionError {
    /// Nested contract call stack trace frame.
    Nested(InnerContractExecutionError),
    /// Terminal error message.
    Message(String),
}

mod errors {
    use core::fmt::{Display, Formatter, Result};

    /// Errors parsing an L1-to-L2 message from transaction calldata.
    #[derive(Debug, PartialEq, Eq)]
    pub enum ParseMsgToL2Error {
        /// The transaction calldata is empty.
        EmptyCalldata,
        /// The L1 sender address is longer than 20 bytes.
        FromAddressOutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for ParseMsgToL2Error {}

    impl Display for ParseMsgToL2Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::EmptyCalldata => {
                    write!(
                        f,
                        "calldata must contain at least 1 element for from_address"
                    )
                }
                Self::FromAddressOutOfRange => {
                    write!(f, "from_address is larger than 20 bytes")
                }
            }
        }
    }
}
pub use errors::ParseMsgToL2Error;

impl MaybePendingBlockWithTxHashes {
    /// Gets a reference to the list of transaction hashes.
    pub fn transactions(&self) -> &[Felt] {
        match self {
            Self::Block(block) => &block.transactions,
            Self::PendingBlock(block) => &block.transactions,
        }
    }

    /// Gets a reference to the L1 gas price.
    pub const fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            Self::Block(block) => &block.l1_gas_price,
            Self::PendingBlock(block) => &block.l1_gas_price,
        }
    }

    /// Gets a reference to the L2 gas price.
    pub const fn l2_gas_price(&self) -> &ResourcePrice {
        match self {
            Self::Block(block) => &block.l2_gas_price,
            Self::PendingBlock(block) => &block.l2_gas_price,
        }
    }

    /// Gets a reference to the L1 data gas price.
    pub const fn l1_data_gas_price(&self) -> &ResourcePrice {
        match self {
            Self::Block(block) => &block.l1_data_gas_price,
            Self::PendingBlock(block) => &block.l1_data_gas_price,
        }
    }
}

impl MaybePendingBlockWithTxs {
    /// Gets a reference to the list of transactions.
    pub fn transactions(&self) -> &[Transaction] {
        match self {
            Self::Block(block) => &block.transactions,
            Self::PendingBlock(block) => &block.transactions,
        }
    }

    /// Gets a reference to the L1 gas price.
    pub const fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            Self::Block(block) => &block.l1_gas_price,
            Self::PendingBlock(block) => &block.l1_gas_price,
        }
    }
}

impl MaybePendingBlockWithReceipts {
    /// Gets a reference to the list of transactions with receipts.
    pub fn transactions(&self) -> &[TransactionWithReceipt] {
        match self {
            Self::Block(block) => &block.transactions,
            Self::PendingBlock(block) => &block.transactions,
        }
    }

    /// Gets a reference to the L1 gas price.
    pub const fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            Self::Block(block) => &block.l1_gas_price,
            Self::PendingBlock(block) => &block.l1_gas_price,
        }
    }
}

impl TransactionStatus {
    /// Gets a reference to the transaction's finality status.
    pub const fn finality_status(&self) -> SequencerTransactionStatus {
        match self {
            Self::Received => SequencerTransactionStatus::Received,
            Self::Rejected => SequencerTransactionStatus::Rejected,
            Self::AcceptedOnL2(_) => SequencerTransactionStatus::AcceptedOnL2,
            Self::AcceptedOnL1(_) => SequencerTransactionStatus::AcceptedOnL1,
        }
    }
}

impl Transaction {
    /// Gets a reference to the transaction's hash.
    pub const fn transaction_hash(&self) -> &Felt {
        match self {
            Self::Invoke(tx) => tx.transaction_hash(),
            Self::L1Handler(tx) => &tx.transaction_hash,
            Self::Declare(tx) => tx.transaction_hash(),
            Self::Deploy(tx) => &tx.transaction_hash,
            Self::DeployAccount(tx) => tx.transaction_hash(),
        }
    }
}

impl InvokeTransaction {
    /// Gets a reference to the transaction's hash.
    pub const fn transaction_hash(&self) -> &Felt {
        match self {
            Self::V0(tx) => &tx.transaction_hash,
            Self::V1(tx) => &tx.transaction_hash,
            Self::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl DeclareTransaction {
    /// Gets a reference to the transaction's hash.
    pub const fn transaction_hash(&self) -> &Felt {
        match self {
            Self::V0(tx) => &tx.transaction_hash,
            Self::V1(tx) => &tx.transaction_hash,
            Self::V2(tx) => &tx.transaction_hash,
            Self::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl DeployAccountTransaction {
    /// Gets a reference to the transaction's hash.
    pub const fn transaction_hash(&self) -> &Felt {
        match self {
            Self::V1(tx) => &tx.transaction_hash,
            Self::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl TransactionReceipt {
    /// Gets a reference to the transaction's hash.
    pub const fn transaction_hash(&self) -> &Felt {
        match self {
            Self::Invoke(receipt) => &receipt.transaction_hash,
            Self::L1Handler(receipt) => &receipt.transaction_hash,
            Self::Declare(receipt) => &receipt.transaction_hash,
            Self::Deploy(receipt) => &receipt.transaction_hash,
            Self::DeployAccount(receipt) => &receipt.transaction_hash,
        }
    }

    /// Gets a reference to the transaction's finality status.
    pub const fn finality_status(&self) -> &TransactionFinalityStatus {
        match self {
            Self::Invoke(receipt) => &receipt.finality_status,
            Self::L1Handler(receipt) => &receipt.finality_status,
            Self::Declare(receipt) => &receipt.finality_status,
            Self::Deploy(receipt) => &receipt.finality_status,
            Self::DeployAccount(receipt) => &receipt.finality_status,
        }
    }

    /// Gets a reference to the transaction's execution result.
    pub const fn execution_result(&self) -> &ExecutionResult {
        match self {
            Self::Invoke(receipt) => &receipt.execution_result,
            Self::L1Handler(receipt) => &receipt.execution_result,
            Self::Declare(receipt) => &receipt.execution_result,
            Self::Deploy(receipt) => &receipt.execution_result,
            Self::DeployAccount(receipt) => &receipt.execution_result,
        }
    }

    /// Gets a reference to the transaction's emitted events.
    pub fn events(&self) -> &[Event] {
        match self {
            Self::Invoke(receipt) => &receipt.events,
            Self::L1Handler(receipt) => &receipt.events,
            Self::Declare(receipt) => &receipt.events,
            Self::Deploy(receipt) => &receipt.events,
            Self::DeployAccount(receipt) => &receipt.events,
        }
    }
}

impl L1HandlerTransaction {
    /// Parses [`MsgToL2`] from the transaction's calldata. This should not never fail on a genuine
    /// `L1_HANDLER` transaction.
    pub fn parse_msg_to_l2(&self) -> Result<MsgToL2, ParseMsgToL2Error> {
        self.calldata.split_first().map_or(
            Err(ParseMsgToL2Error::EmptyCalldata),
            |(from_address, payload)| {
                Ok(MsgToL2 {
                    from_address: (*from_address)
                        .try_into()
                        .map_err(|_| ParseMsgToL2Error::FromAddressOutOfRange)?,
                    to_address: self.contract_address,
                    selector: self.entry_point_selector,
                    payload: payload.into(),
                    nonce: self.nonce,
                })
            },
        )
    }
}

impl AsRef<Self> for BlockId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for FunctionCall {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for MsgFromL1 {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for BroadcastedTransaction {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for BroadcastedInvokeTransaction {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for BroadcastedDeclareTransaction {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Self> for BroadcastedDeployAccountTransaction {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl TryFrom<&L1HandlerTransaction> for MsgToL2 {
    type Error = ParseMsgToL2Error;

    fn try_from(value: &L1HandlerTransaction) -> Result<Self, Self::Error> {
        value.parse_msg_to_l2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_felt_to_string() {
        let felt = Felt::from_dec_str("123456").unwrap();

        assert_eq!(format!("{}", felt), "123456");
        assert_eq!(format!("{:x}", felt), "1e240");
        assert_eq!(format!("{:X}", felt), "1E240");
        assert_eq!(format!("{:#x}", felt), "0x1e240");
        assert_eq!(format!("{:#X}", felt), "0x1E240");
        assert_eq!(format!("{:010x}", felt), "000001e240");
        assert_eq!(format!("{:010X}", felt), "000001E240");
        assert_eq!(format!("{:#010x}", felt), "0x000001e240");
        assert_eq!(format!("{:#010X}", felt), "0x000001E240");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_msg_to_l2() {
        let l1_handler_tx = L1HandlerTransaction {
            transaction_hash: Felt::from_hex(
                "0x374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
            )
            .unwrap(),
            version: Felt::ZERO,
            nonce: 775628,
            contract_address: Felt::from_hex(
                "0x73314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
            )
            .unwrap(),
            entry_point_selector: Felt::from_hex(
                "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
            )
            .unwrap(),
            calldata: vec![
                Felt::from_hex("0xc3511006c04ef1d78af4c8e0e74ec18a6e64ff9e").unwrap(),
                Felt::from_hex("0x689ead7d814e51ed93644bc145f0754839b8dcb340027ce0c30953f38f55d7")
                    .unwrap(),
                Felt::from_hex("0x2c68af0bb140000").unwrap(),
                Felt::from_hex("0x0").unwrap(),
            ],
        };

        let msg_to_l2 = l1_handler_tx.parse_msg_to_l2().unwrap();

        let expected_hash =
            Hash256::from_hex("c51a543ef9563ad2545342b390b67edfcddf9886aa36846cf70382362fc5fab3")
                .unwrap();

        assert_eq!(msg_to_l2.hash(), expected_hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_msg_to_l2_empty_calldata_error() {
        let l1_handler_tx = L1HandlerTransaction {
            transaction_hash: Felt::from_hex(
                "0x374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
            )
            .unwrap(),
            version: Felt::ZERO,
            nonce: 775628,
            contract_address: Felt::from_hex(
                "0x73314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
            )
            .unwrap(),
            entry_point_selector: Felt::from_hex(
                "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
            )
            .unwrap(),
            calldata: Vec::new(), // Empty calldata
        };

        let result = l1_handler_tx.parse_msg_to_l2();

        assert_eq!(result.unwrap_err(), ParseMsgToL2Error::EmptyCalldata);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_parse_msg_to_l2_from_address_out_of_range_error() {
        let l1_handler_tx = L1HandlerTransaction {
            transaction_hash: Felt::from_hex(
                "0x374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c",
            )
            .unwrap(),
            version: Felt::ZERO,
            nonce: 775628,
            contract_address: Felt::from_hex(
                "0x73314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
            )
            .unwrap(),
            entry_point_selector: Felt::from_hex(
                "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
            )
            .unwrap(),
            calldata: vec![
                // Incorrect from address format, causing the conversion error
                // Max address + 1
                Felt::from_hex("0x10000000000000000000000000000000000000000").unwrap(),
                Felt::from_hex("0x689ead7d814e51ed93644bc145f0754839b8dcb340027ce0c30953f38f55d7")
                    .unwrap(),
                Felt::from_hex("0x2c68af0bb140000").unwrap(),
                Felt::from_hex("0x0").unwrap(),
            ],
        };

        let result = l1_handler_tx.parse_msg_to_l2();

        assert_eq!(
            result.unwrap_err(),
            ParseMsgToL2Error::FromAddressOutOfRange
        );
    }
}
