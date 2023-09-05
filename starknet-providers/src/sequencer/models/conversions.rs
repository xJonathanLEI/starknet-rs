use std::sync::Arc;

use starknet_core::types::{self as core, contract::legacy as contract_legacy, FieldElement};

use super::{
    state_update::{DeclaredContract, DeployedContract, StateDiff, StorageDiff},
    *,
};

#[derive(Debug, thiserror::Error)]
#[error("unable to convert type")]
pub struct ConversionError;

pub(crate) struct TransactionWithReceipt {
    pub transaction: TransactionInfo,
    pub receipt: TransactionReceipt,
}

impl From<core::BlockId> for BlockId {
    fn from(value: core::BlockId) -> Self {
        match value {
            core::BlockId::Hash(hash) => Self::Hash(hash),
            core::BlockId::Number(num) => Self::Number(num),
            core::BlockId::Tag(core::BlockTag::Latest) => Self::Latest,
            core::BlockId::Tag(core::BlockTag::Pending) => Self::Pending,
        }
    }
}

impl TryFrom<Block> for core::MaybePendingBlockWithTxHashes {
    type Error = ConversionError;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match (value.block_hash, value.block_number, value.state_root) {
            // Confirmed block
            (Some(block_hash), Some(block_number), Some(state_root)) => {
                Ok(Self::Block(core::BlockWithTxHashes {
                    status: value.status.try_into()?,
                    block_hash,
                    parent_hash: value.parent_block_hash,
                    block_number,
                    new_root: state_root,
                    timestamp: value.timestamp,
                    sequencer_address: value.sequencer_address.unwrap_or_default(),
                    transactions: value
                        .transactions
                        .iter()
                        .map(|tx| tx.transaction_hash())
                        .collect(),
                }))
            }
            // Pending block
            (None, None, None) => Ok(Self::PendingBlock(core::PendingBlockWithTxHashes {
                transactions: value
                    .transactions
                    .iter()
                    .map(|tx| tx.transaction_hash())
                    .collect(),
                timestamp: value.timestamp,
                sequencer_address: value.sequencer_address.unwrap_or_default(),
                parent_hash: value.parent_block_hash,
            })),
            // Unknown combination
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<Block> for core::MaybePendingBlockWithTxs {
    type Error = ConversionError;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match (value.block_hash, value.block_number, value.state_root) {
            // Confirmed block
            (Some(block_hash), Some(block_number), Some(state_root)) => {
                Ok(Self::Block(core::BlockWithTxs {
                    status: value.status.try_into()?,
                    block_hash,
                    parent_hash: value.parent_block_hash,
                    block_number,
                    new_root: state_root,
                    timestamp: value.timestamp,
                    sequencer_address: value.sequencer_address.unwrap_or_default(),
                    transactions: value
                        .transactions
                        .into_iter()
                        .map(|tx| tx.try_into())
                        .collect::<Result<_, _>>()?,
                }))
            }
            // Pending block
            (None, None, None) => Ok(Self::PendingBlock(core::PendingBlockWithTxs {
                transactions: value
                    .transactions
                    .into_iter()
                    .map(|tx| tx.try_into())
                    .collect::<Result<_, _>>()?,
                timestamp: value.timestamp,
                sequencer_address: value.sequencer_address.unwrap_or_default(),
                parent_hash: value.parent_block_hash,
            })),
            // Unknown combination
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<BlockStatus> for core::BlockStatus {
    type Error = ConversionError;

    fn try_from(value: BlockStatus) -> Result<Self, Self::Error> {
        match value {
            BlockStatus::Pending => Ok(Self::Pending),
            BlockStatus::Aborted => Err(ConversionError),
            BlockStatus::Reverted => Ok(Self::Rejected),
            BlockStatus::AcceptedOnL2 => Ok(Self::AcceptedOnL2),
            BlockStatus::AcceptedOnL1 => Ok(Self::AcceptedOnL1),
        }
    }
}

impl TryFrom<TransactionType> for core::Transaction {
    type Error = ConversionError;

    fn try_from(value: TransactionType) -> Result<Self, Self::Error> {
        match value {
            TransactionType::Declare(inner) => Ok(Self::Declare(inner.try_into()?)),
            TransactionType::Deploy(inner) => Ok(Self::Deploy(inner.try_into()?)),
            TransactionType::DeployAccount(inner) => Ok(Self::DeployAccount(inner.into())),
            TransactionType::InvokeFunction(inner) => Ok(Self::Invoke(inner.try_into()?)),
            TransactionType::L1Handler(inner) => Ok(Self::L1Handler(inner.try_into()?)),
        }
    }
}

impl TryFrom<DeclareTransaction> for core::DeclareTransaction {
    type Error = ConversionError;

    fn try_from(value: DeclareTransaction) -> Result<Self, Self::Error> {
        if value.version == FieldElement::ZERO {
            Ok(Self::V0(core::DeclareTransactionV0 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee,
                signature: value.signature,
                class_hash: value.class_hash,
                sender_address: value.sender_address,
            }))
        } else if value.version == FieldElement::ONE {
            Ok(Self::V1(core::DeclareTransactionV1 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee,
                signature: value.signature,
                nonce: value.nonce,
                class_hash: value.class_hash,
                sender_address: value.sender_address,
            }))
        } else if value.version == FieldElement::TWO {
            Ok(Self::V2(core::DeclareTransactionV2 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee,
                signature: value.signature,
                nonce: value.nonce,
                class_hash: value.class_hash,
                compiled_class_hash: value.compiled_class_hash.ok_or(ConversionError)?,
                sender_address: value.sender_address,
            }))
        } else {
            Err(ConversionError)
        }
    }
}

impl TryFrom<DeployTransaction> for core::DeployTransaction {
    type Error = ConversionError;

    fn try_from(value: DeployTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            class_hash: value.class_hash,
            version: value.version.try_into().map_err(|_| ConversionError)?,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
        })
    }
}

impl From<DeployAccountTransaction> for core::DeployAccountTransaction {
    fn from(value: DeployAccountTransaction) -> Self {
        Self {
            transaction_hash: value.transaction_hash,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            class_hash: value.class_hash,
        }
    }
}

impl TryFrom<InvokeFunctionTransaction> for core::InvokeTransaction {
    type Error = ConversionError;

    fn try_from(value: InvokeFunctionTransaction) -> Result<Self, Self::Error> {
        if value.version == FieldElement::ZERO {
            Ok(Self::V0(core::InvokeTransactionV0 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee,
                signature: value.signature,
                contract_address: value.sender_address,
                entry_point_selector: value.entry_point_selector.ok_or(ConversionError)?,
                calldata: value.calldata,
            }))
        } else if value.version == FieldElement::ONE {
            Ok(Self::V1(core::InvokeTransactionV1 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee,
                signature: value.signature,
                nonce: value.nonce.ok_or(ConversionError)?,
                sender_address: value.sender_address,
                calldata: value.calldata,
            }))
        } else {
            Err(ConversionError)
        }
    }
}

impl TryFrom<L1HandlerTransaction> for core::L1HandlerTransaction {
    type Error = ConversionError;

    fn try_from(value: L1HandlerTransaction) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            version: value.version.try_into().map_err(|_| ConversionError)?,
            nonce: value
                .nonce
                .unwrap_or_default()
                .try_into()
                .map_err(|_| ConversionError)?,
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        })
    }
}

impl TryFrom<StateUpdate> for core::MaybePendingStateUpdate {
    type Error = ConversionError;

    fn try_from(value: StateUpdate) -> Result<Self, Self::Error> {
        match (value.block_hash, value.new_root) {
            (Some(block_hash), Some(new_root)) => Ok(Self::Update(core::StateUpdate {
                block_hash,
                new_root,
                old_root: value.old_root,
                state_diff: value.state_diff.into(),
            })),
            (None, None) => Ok(Self::PendingUpdate(core::PendingStateUpdate {
                old_root: value.old_root,
                state_diff: value.state_diff.into(),
            })),
            _ => Err(ConversionError),
        }
    }
}

impl From<StateDiff> for core::StateDiff {
    fn from(value: StateDiff) -> Self {
        Self {
            storage_diffs: value
                .storage_diffs
                .into_iter()
                .map(|(key, value)| core::ContractStorageDiffItem {
                    address: key,
                    storage_entries: value.into_iter().map(|item| item.into()).collect(),
                })
                .collect(),
            deprecated_declared_classes: value.old_declared_contracts,
            declared_classes: value
                .declared_classes
                .into_iter()
                .map(|item| item.into())
                .collect(),
            deployed_contracts: value
                .deployed_contracts
                .into_iter()
                .map(|item| item.into())
                .collect(),
            replaced_classes: value
                .replaced_classes
                .into_iter()
                .map(|item| item.into())
                .collect(),
            nonces: value
                .nonces
                .into_iter()
                .map(|(key, value)| core::NonceUpdate {
                    contract_address: key,
                    nonce: value,
                })
                .collect(),
        }
    }
}

impl From<StorageDiff> for core::StorageEntry {
    fn from(value: StorageDiff) -> Self {
        Self {
            key: value.key,
            value: value.value,
        }
    }
}

impl From<DeclaredContract> for core::DeclaredClassItem {
    fn from(value: DeclaredContract) -> Self {
        Self {
            class_hash: value.class_hash,
            compiled_class_hash: value.compiled_class_hash,
        }
    }
}

impl From<DeployedContract> for core::DeployedContractItem {
    fn from(value: DeployedContract) -> Self {
        Self {
            address: value.address,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeployedContract> for core::ReplacedClassItem {
    fn from(value: DeployedContract) -> Self {
        Self {
            contract_address: value.address,
            class_hash: value.class_hash,
        }
    }
}

impl TryFrom<TransactionInfo> for core::Transaction {
    type Error = ConversionError;

    fn try_from(value: TransactionInfo) -> Result<Self, Self::Error> {
        match value.r#type {
            Some(tx) => tx.try_into(),
            None => Err(ConversionError),
        }
    }
}

impl TryFrom<TransactionWithReceipt> for core::MaybePendingTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        match value.receipt.status {
            // The caller should have directly returned a tx not found error instead in these cases
            TransactionStatus::NotReceived | TransactionStatus::Received => Err(ConversionError),
            TransactionStatus::Pending => Ok(Self::PendingReceipt(value.try_into()?)),
            // Starting from Starknet v0.12.0 transactions no longer have a `PENDING` state.
            // Transactions with `AcceptedOnL2` can live in a pending block and thus not have block
            // hash. The JSON-RPC side requires `block_hash` to be present so we can only interpret
            // it as pending if `block_hash` is missing.
            TransactionStatus::AcceptedOnL2 | TransactionStatus::Reverted => {
                if value.receipt.block_hash.is_some() {
                    Ok(Self::Receipt(value.try_into()?))
                } else {
                    Ok(Self::PendingReceipt(value.try_into()?))
                }
            }
            TransactionStatus::Rejected | TransactionStatus::AcceptedOnL1 => {
                Ok(Self::Receipt(value.try_into()?))
            }
        }
    }
}

impl TryFrom<TransactionWithReceipt> for core::PendingTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        match value.transaction.r#type.as_ref().ok_or(ConversionError)? {
            TransactionType::Declare(_) => Ok(Self::Declare(value.receipt.try_into()?)),
            TransactionType::Deploy(_) => Ok(Self::Deploy(value.try_into()?)),
            TransactionType::DeployAccount(_) => Ok(Self::DeployAccount(value.receipt.try_into()?)),
            TransactionType::InvokeFunction(_) => Ok(Self::Invoke(value.receipt.try_into()?)),
            TransactionType::L1Handler(_) => Ok(Self::L1Handler(value.receipt.try_into()?)),
        }
    }
}

impl TryFrom<TransactionWithReceipt> for core::TransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        match value.transaction.r#type.as_ref().ok_or(ConversionError)? {
            TransactionType::Declare(_) => Ok(Self::Declare(value.receipt.try_into()?)),
            TransactionType::Deploy(_) => Ok(Self::Deploy(value.try_into()?)),
            TransactionType::DeployAccount(_) => Ok(Self::DeployAccount(value.try_into()?)),
            TransactionType::InvokeFunction(_) => Ok(Self::Invoke(value.receipt.try_into()?)),
            TransactionType::L1Handler(_) => Ok(Self::L1Handler(value.receipt.try_into()?)),
        }
    }
}

impl TryFrom<TransactionReceipt> for core::PendingDeclareTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionWithReceipt> for core::PendingDeployTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: value.receipt.actual_fee.ok_or(ConversionError)?,
            messages_sent: value
                .receipt
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value
                .receipt
                .events
                .into_iter()
                .map(|item| item.into())
                .collect(),
            contract_address: match value.transaction.r#type {
                Some(TransactionType::Deploy(inner)) => inner.contract_address,
                _ => return Err(ConversionError),
            },
            execution_result: convert_execution_result(
                value.receipt.status,
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::PendingDeployAccountTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::PendingInvokeTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::PendingL1HandlerTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::DeclareTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            finality_status: value.finality_status.ok_or(ConversionError)?.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionWithReceipt> for core::DeployTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: value.receipt.actual_fee.ok_or(ConversionError)?,
            finality_status: value
                .receipt
                .finality_status
                .ok_or(ConversionError)?
                .try_into()?,
            block_hash: value.receipt.block_hash.ok_or(ConversionError)?,
            block_number: value.receipt.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .receipt
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value
                .receipt
                .events
                .into_iter()
                .map(|item| item.into())
                .collect(),
            contract_address: match value.transaction.r#type {
                Some(TransactionType::Deploy(inner)) => inner.contract_address,
                _ => return Err(ConversionError),
            },
            execution_result: convert_execution_result(
                value.receipt.status,
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionWithReceipt> for core::DeployAccountTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: value.receipt.actual_fee.ok_or(ConversionError)?,
            finality_status: value
                .receipt
                .finality_status
                .ok_or(ConversionError)?
                .try_into()?,
            block_hash: value.receipt.block_hash.ok_or(ConversionError)?,
            block_number: value.receipt.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .receipt
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value
                .receipt
                .events
                .into_iter()
                .map(|item| item.into())
                .collect(),
            contract_address: match value.transaction.r#type {
                Some(TransactionType::DeployAccount(inner)) => inner.contract_address,
                _ => return Err(ConversionError),
            },
            execution_result: convert_execution_result(
                value.receipt.status,
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::InvokeTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            finality_status: value.finality_status.ok_or(ConversionError)?.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl TryFrom<TransactionReceipt> for core::L1HandlerTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            finality_status: value.finality_status.ok_or(ConversionError)?.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
            execution_result: convert_execution_result(
                value.status,
                value.execution_status,
                value.revert_error,
            )?,
        })
    }
}

impl From<L2ToL1Message> for core::MsgToL1 {
    fn from(value: L2ToL1Message) -> Self {
        Self {
            from_address: value.from_address,
            // Unwrapping here is safe
            to_address: FieldElement::from_byte_slice_be(&value.to_address.0).unwrap(),
            payload: value.payload,
        }
    }
}

impl From<core::MsgFromL1> for CallL1Handler {
    fn from(value: core::MsgFromL1) -> Self {
        Self {
            from_address: L1Address::from_slice(value.from_address.as_bytes()),
            to_address: value.to_address,
            entry_point_selector: value.entry_point_selector,
            payload: value.payload,
        }
    }
}

impl From<Event> for core::Event {
    fn from(value: Event) -> Self {
        Self {
            from_address: value.from_address,
            keys: value.keys,
            data: value.data,
        }
    }
}

impl TryFrom<TransactionExecutionStatus> for core::TransactionExecutionStatus {
    type Error = ConversionError;

    fn try_from(value: TransactionExecutionStatus) -> Result<Self, Self::Error> {
        match value {
            TransactionExecutionStatus::Succeeded => Ok(Self::Succeeded),
            TransactionExecutionStatus::Reverted => Ok(Self::Reverted),
            TransactionExecutionStatus::Rejected => Err(ConversionError),
        }
    }
}

impl TryFrom<TransactionFinalityStatus> for core::TransactionFinalityStatus {
    type Error = ConversionError;

    fn try_from(value: TransactionFinalityStatus) -> Result<Self, Self::Error> {
        match value {
            TransactionFinalityStatus::NotReceived => Err(ConversionError),
            TransactionFinalityStatus::Received => Err(ConversionError),
            TransactionFinalityStatus::AcceptedOnL2 => Ok(Self::AcceptedOnL2),
            TransactionFinalityStatus::AcceptedOnL1 => Ok(Self::AcceptedOnL1),
        }
    }
}

impl From<core::FunctionCall> for CallFunction {
    fn from(value: core::FunctionCall) -> Self {
        Self {
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        }
    }
}

impl TryFrom<core::BroadcastedTransaction> for AccountTransaction {
    type Error = ConversionError;

    fn try_from(value: core::BroadcastedTransaction) -> Result<Self, Self::Error> {
        match value {
            core::BroadcastedTransaction::Invoke(inner) => Ok(Self::InvokeFunction(inner.into())),
            core::BroadcastedTransaction::Declare(inner) => Ok(Self::Declare(inner.try_into()?)),
            core::BroadcastedTransaction::DeployAccount(inner) => {
                Ok(Self::DeployAccount(inner.into()))
            }
        }
    }
}

impl From<core::BroadcastedInvokeTransaction> for InvokeFunctionTransactionRequest {
    fn from(value: core::BroadcastedInvokeTransaction) -> Self {
        Self {
            sender_address: value.sender_address,
            calldata: value.calldata,
            signature: value.signature,
            max_fee: value.max_fee,
            nonce: value.nonce,
            is_query: value.is_query,
        }
    }
}

impl TryFrom<core::BroadcastedDeclareTransaction> for DeclareTransactionRequest {
    type Error = ConversionError;

    fn try_from(value: core::BroadcastedDeclareTransaction) -> Result<Self, Self::Error> {
        match value {
            core::BroadcastedDeclareTransaction::V1(inner) => Ok(Self::V1(inner.into())),
            core::BroadcastedDeclareTransaction::V2(inner) => Ok(Self::V2(inner.try_into()?)),
        }
    }
}

impl From<core::BroadcastedDeclareTransactionV1> for DeclareV1TransactionRequest {
    fn from(value: core::BroadcastedDeclareTransactionV1) -> Self {
        Self {
            contract_class: Arc::new((*value.contract_class).clone().into()),
            sender_address: value.sender_address,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            is_query: value.is_query,
        }
    }
}

impl TryFrom<core::BroadcastedDeclareTransactionV2> for DeclareV2TransactionRequest {
    type Error = ConversionError;

    fn try_from(value: core::BroadcastedDeclareTransactionV2) -> Result<Self, Self::Error> {
        Ok(Self {
            contract_class: Arc::new(
                contract::CompressedSierraClass::from_flattened(&value.contract_class)
                    .map_err(|_| ConversionError)?,
            ),
            compiled_class_hash: value.compiled_class_hash,
            sender_address: value.sender_address,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            is_query: value.is_query,
        })
    }
}

impl From<core::BroadcastedDeployAccountTransaction> for DeployAccountTransactionRequest {
    fn from(value: core::BroadcastedDeployAccountTransaction) -> Self {
        Self {
            class_hash: value.class_hash,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            is_query: value.is_query,
        }
    }
}

impl From<FeeEstimate> for core::FeeEstimate {
    fn from(value: FeeEstimate) -> Self {
        Self {
            gas_consumed: value.gas_usage,
            gas_price: value.gas_price,
            overall_fee: value.overall_fee,
        }
    }
}

impl From<core::CompressedLegacyContractClass> for CompressedLegacyContractClass {
    fn from(value: core::CompressedLegacyContractClass) -> Self {
        Self {
            program: value.program,
            entry_points_by_type: contract_legacy::RawLegacyEntryPoints {
                constructor: value
                    .entry_points_by_type
                    .constructor
                    .into_iter()
                    .map(convert_legacy_entry_point)
                    .collect(),
                external: value
                    .entry_points_by_type
                    .external
                    .into_iter()
                    .map(convert_legacy_entry_point)
                    .collect(),
                l1_handler: value
                    .entry_points_by_type
                    .l1_handler
                    .into_iter()
                    .map(convert_legacy_entry_point)
                    .collect(),
            },
            abi: value
                .abi
                .map(|abi| abi.into_iter().map(|item| item.into()).collect()),
        }
    }
}

impl TryFrom<DeployedClass> for core::ContractClass {
    type Error = ConversionError;

    fn try_from(value: DeployedClass) -> Result<Self, Self::Error> {
        match value {
            DeployedClass::SierraClass(inner) => Ok(Self::Sierra(inner)),
            DeployedClass::LegacyClass(inner) => {
                Ok(Self::Legacy(inner.compress().map_err(|_| ConversionError)?))
            }
        }
    }
}

fn convert_execution_result(
    status: TransactionStatus,
    execution_status: Option<TransactionExecutionStatus>,
    revert_error: Option<String>,
) -> Result<core::ExecutionResult, ConversionError> {
    match (execution_status, revert_error) {
        (None, None) => {
            // This is a response from pre-v0.12.1
            match status {
                TransactionStatus::Pending
                | TransactionStatus::AcceptedOnL2
                | TransactionStatus::AcceptedOnL1 => {
                    // Pre-v0.12.1 transactions are always successful as long as they're in a block
                    Ok(core::ExecutionResult::Succeeded)
                }
                TransactionStatus::NotReceived
                | TransactionStatus::Received
                | TransactionStatus::Rejected
                | TransactionStatus::Reverted => {
                    // Otherwise it's a status not representable in JSON-RPC
                    Err(ConversionError)
                }
            }
        }
        (Some(TransactionExecutionStatus::Succeeded), None) => Ok(core::ExecutionResult::Succeeded),
        (Some(TransactionExecutionStatus::Reverted), Some(revert_error)) => {
            Ok(core::ExecutionResult::Reverted {
                reason: revert_error,
            })
        }
        // All other combinations are illegal
        _ => Err(ConversionError),
    }
}

fn convert_legacy_entry_point(
    value: core::LegacyContractEntryPoint,
) -> contract_legacy::RawLegacyEntryPoint {
    // WARNING: this causes pre-0.11.0 contract declaration to fail due to `offset` issue
    // TODO: support declaring pre-0.11.0 contracts here (do we even care though?)
    contract_legacy::RawLegacyEntryPoint {
        offset: contract_legacy::LegacyEntrypointOffset::U64AsInt(value.offset),
        selector: value.selector,
    }
}
