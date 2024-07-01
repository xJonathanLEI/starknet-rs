use alloc::{string::*, vec::*};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serde::unsigned_field_element::UfeHex;

pub use starknet_types_core::felt::*;

mod conversions;

mod serde_impls;

// TODO: better namespacing of exports?
mod codegen;
pub use codegen::{
    BlockStatus, BlockTag, BlockWithReceipts, BlockWithTxHashes, BlockWithTxs,
    BroadcastedDeclareTransactionV1, BroadcastedDeclareTransactionV2,
    BroadcastedDeclareTransactionV3, BroadcastedDeployAccountTransactionV1,
    BroadcastedDeployAccountTransactionV3, BroadcastedInvokeTransactionV1,
    BroadcastedInvokeTransactionV3, CallType, CompressedLegacyContractClass, ComputationResources,
    ContractErrorData, ContractStorageDiffItem, DataAvailabilityMode, DataAvailabilityResources,
    DataResources, DeclareTransactionReceipt, DeclareTransactionTrace, DeclareTransactionV0,
    DeclareTransactionV1, DeclareTransactionV2, DeclareTransactionV3, DeclaredClassItem,
    DeployAccountTransactionReceipt, DeployAccountTransactionTrace, DeployAccountTransactionV1,
    DeployAccountTransactionV3, DeployTransaction, DeployTransactionReceipt, DeployedContractItem,
    EmittedEvent, EntryPointType, EntryPointsByType, Event, EventFilter, EventFilterWithPage,
    EventsChunk, ExecutionResources, FeeEstimate, FeePayment, FlattenedSierraClass, FunctionCall,
    FunctionInvocation, FunctionStateMutability, InvokeTransactionReceipt, InvokeTransactionTrace,
    InvokeTransactionV0, InvokeTransactionV1, InvokeTransactionV3, L1DataAvailabilityMode,
    L1HandlerTransaction, L1HandlerTransactionReceipt, L1HandlerTransactionTrace,
    LegacyContractEntryPoint, LegacyEntryPointsByType, LegacyEventAbiEntry, LegacyEventAbiType,
    LegacyFunctionAbiEntry, LegacyFunctionAbiType, LegacyStructAbiEntry, LegacyStructAbiType,
    LegacyStructMember, LegacyTypedParameter, MsgFromL1, MsgToL1, NoTraceAvailableErrorData,
    NonceUpdate, OrderedEvent, OrderedMessage, PendingBlockWithReceipts, PendingBlockWithTxHashes,
    PendingBlockWithTxs, PendingStateUpdate, PriceUnit, ReplacedClassItem, ResourceBounds,
    ResourceBoundsMapping, ResourcePrice, ResultPageRequest, RevertedInvocation,
    SequencerTransactionStatus, SierraEntryPoint, SimulatedTransaction, SimulationFlag,
    SimulationFlagForEstimateFee, StarknetError, StateDiff, StateUpdate, StorageEntry, SyncStatus,
    TransactionExecutionErrorData, TransactionExecutionStatus, TransactionFinalityStatus,
    TransactionReceiptWithBlockInfo, TransactionTraceWithHash, TransactionWithReceipt,
};

pub mod u256;
pub use u256::U256;

pub mod eth_address;
pub use eth_address::EthAddress;

pub mod hash_256;
pub use hash_256::Hash256;

mod execution_result;
pub use execution_result::ExecutionResult;

mod receipt_block;
pub use receipt_block::ReceiptBlock;

mod msg;
pub use msg::MsgToL2;

// TODO: move generated request code to `starknet-providers`
pub mod requests;

pub mod contract;
pub use contract::ContractArtifact;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxHashes {
    Block(BlockWithTxHashes),
    PendingBlock(PendingBlockWithTxHashes),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithTxs {
    Block(BlockWithTxs),
    PendingBlock(PendingBlockWithTxs),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingBlockWithReceipts {
    Block(BlockWithReceipts),
    PendingBlock(PendingBlockWithReceipts),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePendingStateUpdate {
    Update(StateUpdate),
    PendingUpdate(PendingStateUpdate),
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHashAndNumber {
    #[serde_as(as = "UfeHex")]
    pub block_hash: Felt,
    pub block_number: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatusType {
    Syncing(SyncStatus),
    NotSyncing,
}

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

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvokeTransactionResult {
    /// The hash of the invoke transaction
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
}

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

/// Block hash, number or tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockId {
    Hash(Felt),
    Number(u64),
    Tag(BlockTag),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractClass {
    Sierra(FlattenedSierraClass),
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
    AcceptedOnL2(TransactionExecutionStatus),
    /// Transaction accepted on Layer 1 with a specific execution status.
    AcceptedOnL1(TransactionExecutionStatus),
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastedTransaction {
    #[serde(rename = "INVOKE")]
    Invoke(BroadcastedInvokeTransaction),
    #[serde(rename = "DECLARE")]
    Declare(BroadcastedDeclareTransaction),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(BroadcastedDeployAccountTransaction),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum InvokeTransaction {
    #[serde(rename = "0x0")]
    V0(InvokeTransactionV0),
    #[serde(rename = "0x1")]
    V1(InvokeTransactionV1),
    #[serde(rename = "0x3")]
    V3(InvokeTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeclareTransaction {
    #[serde(rename = "0x0")]
    V0(DeclareTransactionV0),
    #[serde(rename = "0x1")]
    V1(DeclareTransactionV1),
    #[serde(rename = "0x2")]
    V2(DeclareTransactionV2),
    #[serde(rename = "0x3")]
    V3(DeclareTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "version")]
pub enum DeployAccountTransaction {
    #[serde(rename = "0x1")]
    V1(DeployAccountTransactionV1),
    #[serde(rename = "0x3")]
    V3(DeployAccountTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum BroadcastedInvokeTransaction {
    V1(BroadcastedInvokeTransactionV1),
    V3(BroadcastedInvokeTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum BroadcastedDeclareTransaction {
    V1(BroadcastedDeclareTransactionV1),
    V2(BroadcastedDeclareTransactionV2),
    V3(BroadcastedDeclareTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum BroadcastedDeployAccountTransaction {
    V1(BroadcastedDeployAccountTransactionV1),
    V3(BroadcastedDeployAccountTransactionV3),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyContractAbiEntry {
    Function(LegacyFunctionAbiEntry),
    Event(LegacyEventAbiEntry),
    Struct(LegacyStructAbiEntry),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionTrace {
    #[serde(rename = "INVOKE")]
    Invoke(InvokeTransactionTrace),
    #[serde(rename = "DEPLOY_ACCOUNT")]
    DeployAccount(DeployAccountTransactionTrace),
    #[serde(rename = "L1_HANDLER")]
    L1Handler(L1HandlerTransactionTrace),
    #[serde(rename = "DECLARE")]
    Declare(DeclareTransactionTrace),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum ExecuteInvocation {
    Success(FunctionInvocation),
    Reverted(RevertedInvocation),
}

mod errors {
    use core::fmt::{Display, Formatter, Result};

    #[derive(Debug, PartialEq)]
    pub enum ParseMsgToL2Error {
        EmptyCalldata,
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
    pub fn transactions(&self) -> &[Felt] {
        match self {
            MaybePendingBlockWithTxHashes::Block(block) => &block.transactions,
            MaybePendingBlockWithTxHashes::PendingBlock(block) => &block.transactions,
        }
    }

    pub fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            MaybePendingBlockWithTxHashes::Block(block) => &block.l1_gas_price,
            MaybePendingBlockWithTxHashes::PendingBlock(block) => &block.l1_gas_price,
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

    pub fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            MaybePendingBlockWithTxs::Block(block) => &block.l1_gas_price,
            MaybePendingBlockWithTxs::PendingBlock(block) => &block.l1_gas_price,
        }
    }
}

impl MaybePendingBlockWithReceipts {
    pub fn transactions(&self) -> &[TransactionWithReceipt] {
        match self {
            MaybePendingBlockWithReceipts::Block(block) => &block.transactions,
            MaybePendingBlockWithReceipts::PendingBlock(block) => &block.transactions,
        }
    }

    pub fn l1_gas_price(&self) -> &ResourcePrice {
        match self {
            MaybePendingBlockWithReceipts::Block(block) => &block.l1_gas_price,
            MaybePendingBlockWithReceipts::PendingBlock(block) => &block.l1_gas_price,
        }
    }
}

impl TransactionStatus {
    pub fn finality_status(&self) -> SequencerTransactionStatus {
        match self {
            TransactionStatus::Received => SequencerTransactionStatus::Received,
            TransactionStatus::Rejected => SequencerTransactionStatus::Rejected,
            TransactionStatus::AcceptedOnL2(_) => SequencerTransactionStatus::AcceptedOnL2,
            TransactionStatus::AcceptedOnL1(_) => SequencerTransactionStatus::AcceptedOnL1,
        }
    }
}

impl Transaction {
    pub fn transaction_hash(&self) -> &Felt {
        match self {
            Transaction::Invoke(tx) => tx.transaction_hash(),
            Transaction::L1Handler(tx) => &tx.transaction_hash,
            Transaction::Declare(tx) => tx.transaction_hash(),
            Transaction::Deploy(tx) => &tx.transaction_hash,
            Transaction::DeployAccount(tx) => tx.transaction_hash(),
        }
    }
}

impl InvokeTransaction {
    pub fn transaction_hash(&self) -> &Felt {
        match self {
            InvokeTransaction::V0(tx) => &tx.transaction_hash,
            InvokeTransaction::V1(tx) => &tx.transaction_hash,
            InvokeTransaction::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl DeclareTransaction {
    pub fn transaction_hash(&self) -> &Felt {
        match self {
            DeclareTransaction::V0(tx) => &tx.transaction_hash,
            DeclareTransaction::V1(tx) => &tx.transaction_hash,
            DeclareTransaction::V2(tx) => &tx.transaction_hash,
            DeclareTransaction::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl DeployAccountTransaction {
    pub fn transaction_hash(&self) -> &Felt {
        match self {
            DeployAccountTransaction::V1(tx) => &tx.transaction_hash,
            DeployAccountTransaction::V3(tx) => &tx.transaction_hash,
        }
    }
}

impl TransactionReceipt {
    pub fn transaction_hash(&self) -> &Felt {
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

impl L1HandlerTransaction {
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
    fn test_broadcasted_invoke_v1_non_query_deser() {
        let raw = include_str!("../../test-data/serde/broadcasted_invoke_v1_non_query.json");

        let parsed_object = serde_json::from_str::<BroadcastedInvokeTransactionV1>(raw).unwrap();
        assert!(!parsed_object.is_query);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_broadcasted_invoke_v1_query_deser() {
        let raw = include_str!("../../test-data/serde/broadcasted_invoke_v1_query.json");

        let parsed_object = serde_json::from_str::<BroadcastedInvokeTransactionV1>(raw).unwrap();
        assert!(parsed_object.is_query);
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
