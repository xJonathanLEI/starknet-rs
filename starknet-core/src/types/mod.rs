use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;

pub use starknet_ff::*;

mod conversions;

mod serde_impls;

// TODO: better namespacing of exports?
mod codegen;
pub use codegen::{
    BlockStatus, BlockTag, BlockWithTxHashes, BlockWithTxs, BroadcastedDeclareTransactionV1,
    BroadcastedDeclareTransactionV2, BroadcastedDeployAccountTransaction,
    BroadcastedInvokeTransaction, CompressedLegacyContractClass, ContractStorageDiffItem,
    DeclareTransactionReceipt, DeclareTransactionV0, DeclareTransactionV1, DeclareTransactionV2,
    DeclaredClassItem, DeployAccountTransaction, DeployAccountTransactionReceipt,
    DeployTransaction, DeployTransactionReceipt, DeployedContractItem, EmittedEvent,
    EntryPointsByType, Event, EventFilter, EventFilterWithPage, EventsChunk, FeeEstimate,
    FlattenedSierraClass, FunctionCall, FunctionStateMutability, InvokeTransactionReceipt,
    InvokeTransactionV0, InvokeTransactionV1, L1HandlerTransaction, L1HandlerTransactionReceipt,
    LegacyContractEntryPoint, LegacyEntryPointsByType, LegacyEventAbiEntry, LegacyEventAbiType,
    LegacyFunctionAbiEntry, LegacyFunctionAbiType, LegacyStructAbiEntry, LegacyStructAbiType,
    LegacyStructMember, LegacyTypedParameter, MsgFromL1, MsgToL1, NonceUpdate,
    PendingBlockWithTxHashes, PendingBlockWithTxs, PendingDeclareTransactionReceipt,
    PendingDeployAccountTransactionReceipt, PendingDeployTransactionReceipt,
    PendingInvokeTransactionReceipt, PendingL1HandlerTransactionReceipt, PendingStateUpdate,
    ReplacedClassItem, ResultPageRequest, SierraEntryPoint, StarknetError, StateDiff, StateUpdate,
    StorageEntry, SyncStatus, TransactionExecutionStatus, TransactionFinalityStatus,
};

pub mod eth_address;
pub use eth_address::EthAddress;

mod execution_result;
pub use execution_result::ExecutionResult;

// TODO: move generated request code to `starknet-providers`
pub mod requests;

pub mod contract;
pub mod trace;

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
pub enum DeclareTransaction {
    #[serde(rename = "0x0")]
    V0(DeclareTransactionV0),
    #[serde(rename = "0x1")]
    V1(DeclareTransactionV1),
    #[serde(rename = "0x2")]
    V2(DeclareTransactionV2),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BroadcastedDeclareTransaction {
    V1(BroadcastedDeclareTransactionV1),
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

impl MaybePendingBlockWithTxHashes {
    pub fn transactions(&self) -> &[FieldElement] {
        match self {
            MaybePendingBlockWithTxHashes::Block(block) => &block.transactions,
            MaybePendingBlockWithTxHashes::PendingBlock(block) => &block.transactions,
        }
    }
}

impl MaybePendingBlockWithTxs {
    pub fn transactions(&self) -> &[Transaction] {
        match self {
            MaybePendingBlockWithTxs::Block(block) => &block.transactions,
            MaybePendingBlockWithTxs::PendingBlock(block) => &block.transactions,
        }
    }
}

impl Transaction {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            Transaction::Invoke(tx) => tx.transaction_hash(),
            Transaction::L1Handler(tx) => &tx.transaction_hash,
            Transaction::Declare(tx) => tx.transaction_hash(),
            Transaction::Deploy(tx) => &tx.transaction_hash,
            Transaction::DeployAccount(tx) => &tx.transaction_hash,
        }
    }
}

impl InvokeTransaction {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            InvokeTransaction::V0(tx) => &tx.transaction_hash,
            InvokeTransaction::V1(tx) => &tx.transaction_hash,
        }
    }
}

impl DeclareTransaction {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            DeclareTransaction::V0(tx) => &tx.transaction_hash,
            DeclareTransaction::V1(tx) => &tx.transaction_hash,
            DeclareTransaction::V2(tx) => &tx.transaction_hash,
        }
    }
}

impl MaybePendingTransactionReceipt {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            MaybePendingTransactionReceipt::Receipt(receipt) => receipt.transaction_hash(),
            MaybePendingTransactionReceipt::PendingReceipt(receipt) => receipt.transaction_hash(),
        }
    }

    pub fn finality_status(&self) -> &TransactionFinalityStatus {
        match self {
            MaybePendingTransactionReceipt::Receipt(receipt) => receipt.finality_status(),
            MaybePendingTransactionReceipt::PendingReceipt(receipt) => receipt.finality_status(),
        }
    }

    pub fn execution_result(&self) -> &ExecutionResult {
        match self {
            MaybePendingTransactionReceipt::Receipt(receipt) => receipt.execution_result(),
            MaybePendingTransactionReceipt::PendingReceipt(receipt) => receipt.execution_result(),
        }
    }
}

impl TransactionReceipt {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            TransactionReceipt::Invoke(receipt) => &receipt.transaction_hash,
            TransactionReceipt::L1Handler(receipt) => &receipt.transaction_hash,
            TransactionReceipt::Declare(receipt) => &receipt.transaction_hash,
            TransactionReceipt::Deploy(receipt) => &receipt.transaction_hash,
            TransactionReceipt::DeployAccount(receipt) => &receipt.transaction_hash,
        }
    }

    pub fn finality_status(&self) -> &TransactionFinalityStatus {
        match self {
            TransactionReceipt::Invoke(receipt) => &receipt.finality_status,
            TransactionReceipt::L1Handler(receipt) => &receipt.finality_status,
            TransactionReceipt::Declare(receipt) => &receipt.finality_status,
            TransactionReceipt::Deploy(receipt) => &receipt.finality_status,
            TransactionReceipt::DeployAccount(receipt) => &receipt.finality_status,
        }
    }

    pub fn execution_result(&self) -> &ExecutionResult {
        match self {
            TransactionReceipt::Invoke(receipt) => &receipt.execution_result,
            TransactionReceipt::L1Handler(receipt) => &receipt.execution_result,
            TransactionReceipt::Declare(receipt) => &receipt.execution_result,
            TransactionReceipt::Deploy(receipt) => &receipt.execution_result,
            TransactionReceipt::DeployAccount(receipt) => &receipt.execution_result,
        }
    }
}

impl PendingTransactionReceipt {
    pub fn transaction_hash(&self) -> &FieldElement {
        match self {
            PendingTransactionReceipt::Invoke(receipt) => &receipt.transaction_hash,
            PendingTransactionReceipt::L1Handler(receipt) => &receipt.transaction_hash,
            PendingTransactionReceipt::Declare(receipt) => &receipt.transaction_hash,
            PendingTransactionReceipt::Deploy(receipt) => &receipt.transaction_hash,
            PendingTransactionReceipt::DeployAccount(receipt) => &receipt.transaction_hash,
        }
    }

    pub fn finality_status(&self) -> &TransactionFinalityStatus {
        &TransactionFinalityStatus::AcceptedOnL2
    }

    pub fn execution_result(&self) -> &ExecutionResult {
        match self {
            PendingTransactionReceipt::Invoke(receipt) => &receipt.execution_result,
            PendingTransactionReceipt::L1Handler(receipt) => &receipt.execution_result,
            PendingTransactionReceipt::Declare(receipt) => &receipt.execution_result,
            PendingTransactionReceipt::Deploy(receipt) => &receipt.execution_result,
            PendingTransactionReceipt::DeployAccount(receipt) => &receipt.execution_result,
        }
    }
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

impl AsRef<MsgFromL1> for MsgFromL1 {
    fn as_ref(&self) -> &MsgFromL1 {
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
            27 => StarknetError::InvalidTransactionIndex,
            28 => StarknetError::ClassHashNotFound,
            // JSON-RPC v0.4.0 changes this error code from 25 to 29. Technically we should just
            // ignore 25 for good, but since most methods are otherwise identical to v0.3.0,
            // accepting the value of 25 here allows some degree of compatibility, meaning some use
            // cases can still be run against a v0.3.0 endpoint even though the library itself only
            // officially supports v0.4.0. This can be beneficial as v0.3.0 is still widely
            // deployed.
            //
            // TODO: remove this line once JSON-RPC v0.3.0 is phased out
            25 => StarknetError::TransactionHashNotFound,
            29 => StarknetError::TransactionHashNotFound,
            31 => StarknetError::PageSizeTooBig,
            32 => StarknetError::NoBlocks,
            33 => StarknetError::InvalidContinuationToken,
            34 => StarknetError::TooManyKeysInFilter,
            40 => StarknetError::ContractError,
            51 => StarknetError::ClassAlreadyDeclared,
            52 => StarknetError::InvalidTransactionNonce,
            53 => StarknetError::InsufficientMaxFee,
            54 => StarknetError::InsufficientAccountBalance,
            55 => StarknetError::ValidationFailure,
            56 => StarknetError::CompilationFailed,
            57 => StarknetError::ContractClassSizeIsTooLarge,
            58 => StarknetError::NonAccount,
            59 => StarknetError::DuplicateTx,
            60 => StarknetError::CompiledClassHashMismatch,
            61 => StarknetError::UnsupportedTxVersion,
            62 => StarknetError::UnsupportedContractClassVersion,
            63 => StarknetError::UnexpectedError,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_broadcasted_invoke_v1_non_query_deser() {
        let raw = include_str!("../../test-data/serde/broadcasted_invoke_v1_non_query.json");

        let parsed_object = serde_json::from_str::<BroadcastedInvokeTransaction>(raw).unwrap();
        assert!(!parsed_object.is_query);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_broadcasted_invoke_v1_query_deser() {
        let raw = include_str!("../../test-data/serde/broadcasted_invoke_v1_query.json");

        let parsed_object = serde_json::from_str::<BroadcastedInvokeTransaction>(raw).unwrap();
        assert!(parsed_object.is_query);
    }
}
