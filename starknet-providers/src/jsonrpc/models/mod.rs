use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::UfeHex,
    types::{FieldElement, StarknetError},
};

pub use starknet_core::types::L1Address as EthAddress;

mod serde_impls;

/// Temporary module before JSON-RPC becomes the de-facto provider:
///
/// https://github.com/xJonathanLEI/starknet-rs/issues/77#issuecomment-1150184364
mod conversions;

mod codegen;
pub use codegen::*;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractClass {
    Sierra(SierraContractClass),
    Legacy(LegacyContractClass),
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
    #[serde(rename = "DEPLOY")]
    Deploy(BroadcastedDeployTransaction),
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
pub enum ContractAbiEntry {
    Function(FunctionAbiEntry),
    Event(EventAbiEntry),
    Struct(StructAbiEntry),
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

impl From<ErrorCode> for StarknetError {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::FailedToReceiveTransaction => Self::FailedToReceiveTxn,
            ErrorCode::ContractNotFound => Self::ContractNotFound,
            ErrorCode::InvalidMessageSelector => Self::InvalidMessageSelector,
            ErrorCode::InvalidCallData => Self::InvalidCallData,
            ErrorCode::BlockNotFound => Self::BlockNotFound,
            ErrorCode::TransactionHashNotFound => Self::TxnHashNotFound,
            ErrorCode::InvalidTransactionIndex => Self::InvalidTxnIndex,
            ErrorCode::ClassHashNotFound => Self::ClassHashNotFound,
            ErrorCode::PageSizeTooBig => Self::PageSizeTooBig,
            ErrorCode::NoBlocks => Self::NoBlocks,
            ErrorCode::InvalidContinuationToken => Self::InvalidContinuationToken,
            ErrorCode::ContractError => Self::ContractError,
        }
    }
}
