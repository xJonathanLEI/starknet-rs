use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;
use crate::stdlib::string::String;
use crate::stdlib::vec::Vec;

// Re-export commonly used upstream types
pub use ethereum_types::Address as L1Address;

pub use starknet_ff::*;

mod conversions;

mod serde_impls;

// TODO: better namespacing of exports?
mod codegen;
pub use codegen::{
    BlockStatus, BlockTag, BlockWithTxHashes, BlockWithTxs, BroadcastedDeclareTransactionV1,
    BroadcastedDeclareTransactionV2, BroadcastedDeployAccountTransaction,
    BroadcastedInvokeTransactionV0, BroadcastedInvokeTransactionV1, CompressedLegacyContractClass,
    ContractStorageDiffItem, DeclareTransactionReceipt, DeclareTransactionV1, DeclareTransactionV2,
    DeclaredClassItem, DeployAccountTransaction, DeployAccountTransactionReceipt,
    DeployTransaction, DeployTransactionReceipt, DeployedContractItem, EmittedEvent,
    EntryPointsByType, Event, EventFilter, EventFilterWithPage, EventsChunk, FeeEstimate,
    FlattenedSierraClass, FunctionCall, FunctionStateMutability, InvokeTransactionReceipt,
    InvokeTransactionV0, InvokeTransactionV1, L1HandlerTransaction, L1HandlerTransactionReceipt,
    LegacyContractEntryPoint, LegacyEntryPointsByType, LegacyEventAbiEntry, LegacyEventAbiType,
    LegacyFunctionAbiEntry, LegacyFunctionAbiType, LegacyStructAbiEntry, LegacyStructAbiType,
    LegacyStructMember, LegacyTypedParameter, MsgToL1, NonceUpdate, PendingBlockWithTxHashes,
    PendingBlockWithTxs, PendingDeclareTransactionReceipt, PendingDeployAccountTransactionReceipt,
    PendingDeployTransactionReceipt, PendingInvokeTransactionReceipt,
    PendingL1HandlerTransactionReceipt, PendingStateUpdate, ReplacedClassItem, ResultPageRequest,
    SierraEntryPoint, StarknetError, StateDiff, StateUpdate, StorageEntry, SyncStatus,
    TransactionStatus,
};

// TODO: move generated request code to `starknet-providers`
pub mod requests;

pub mod contract;
pub use contract::ContractArtifact;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingStateUpdate {
    Update(StateUpdate),
    PendingUpdate(PendingStateUpdate),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    #[serde_as(as = "UfeHex")]
    pub block_hash: FieldElement,
    pub block_number: u64,
}

#[derive(Debug, Clone)]
pub enum SyncStatusType {
    Syncing(SyncStatus),
    NotSyncing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsPage {
    /// Matching events
    pub events: Vec<EmittedEvent>,
    /// A pointer to the last element of the delivered page, use this token in a subsequent query to
    /// obtain the next page
    pub continuation_token: Option<String>,
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAccountTransactionResult {
    /// The hash of the deploy transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    /// The address of the new contract
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
}

/// Block hash, number or tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractClass {
    Sierra(FlattenedSierraClass),
    Legacy(CompressedLegacyContractClass),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransaction),
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransaction),
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransaction),
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransaction),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransaction),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastedTransaction {
    #[serde(rename = "INVOKE")]
    Invoke(BroadcastedInvokeTransaction),
    #[serde(rename = "DECLARE")]
    Declare(BroadcastedDeclareTransaction),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(BroadcastedDeployAccountTransaction),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "version")]
pub enum InvokeTransaction {
    #[serde(rename = "0x0")]
    V0(InvokeTransactionV0),
    #[serde(rename = "0x1")]
    V1(InvokeTransactionV1),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "version")]
pub enum BroadcastedInvokeTransaction {
    #[serde(rename = "0x0")]
    V0(BroadcastedInvokeTransactionV0),
    #[serde(rename = "0x1")]
    V1(BroadcastedInvokeTransactionV1),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "version")]
pub enum DeclareTransaction {
    #[serde(rename = "0x1")]
    V1(DeclareTransactionV1),
    #[serde(rename = "0x2")]
    V2(DeclareTransactionV2),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "version")]
pub enum BroadcastedDeclareTransaction {
    #[serde(rename = "0x1")]
    V1(BroadcastedDeclareTransactionV1),
    #[serde(rename = "0x2")]
    V2(BroadcastedDeclareTransactionV2),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionReceipt {
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransactionReceipt),
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransactionReceipt),
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransactionReceipt),
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransactionReceipt),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransactionReceipt),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum PendingTransactionReceipt {
    #[serde(rename = "INVOKE")]
    Invoke(PendingInvokeTransactionReceipt),
    #[serde(rename = "L1_HANDLER")]
    L1Handler(PendingL1HandlerTransactionReceipt),
    #[serde(rename = "DECLARE")]
    Declare(PendingDeclareTransactionReceipt),
    #[serde(rename = "DEPLOY")]
    Deploy(PendingDeployTransactionReceipt),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(PendingDeployAccountTransactionReceipt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyContractAbiEntry {
    Function(LegacyFunctionAbiEntry),
    Event(LegacyEventAbiEntry),
    Struct(LegacyStructAbiEntry),
}

impl AsRef<BlockId> for BlockId {
    fn as_ref(&self) -> &BlockId {
        self
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

impl AsRef<BroadcastedInvokeTransaction> for BroadcastedInvokeTransaction {
    fn as_ref(&self) -> &BroadcastedInvokeTransaction {
        self
    }
}

impl AsRef<BroadcastedDeclareTransaction> for BroadcastedDeclareTransaction {
    fn as_ref(&self) -> &BroadcastedDeclareTransaction {
        self
    }
}

impl AsRef<BroadcastedDeployAccountTransaction> for BroadcastedDeployAccountTransaction {
    fn as_ref(&self) -> &BroadcastedDeployAccountTransaction {
        self
    }
}

impl TryFrom<i64> for StarknetError {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => StarknetError::FailedToReceiveTransaction,
            20 => StarknetError::ContractNotFound,
            24 => StarknetError::BlockNotFound,
            25 => StarknetError::TransactionHashNotFound,
            27 => StarknetError::InvalidTransactionIndex,
            28 => StarknetError::ClassHashNotFound,
            31 => StarknetError::PageSizeTooBig,
            32 => StarknetError::NoBlocks,
            33 => StarknetError::InvalidContinuationToken,
            34 => StarknetError::TooManyKeysInFilter,
            40 => StarknetError::ContractError,
            50 => StarknetError::InvalidContractClass,
            51 => StarknetError::ClassAlreadyDeclared,
            _ => return Err(()),
        })
    }
}
