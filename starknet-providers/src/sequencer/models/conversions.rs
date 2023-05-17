use starknet_core::types::{self as core, FieldElement};

use super::{
    state_update::{DeployedContract, StateDiff, StorageDiff},
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
        if value.version == FieldElement::ONE {
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
                nonce: value.nonce.unwrap_or_default(),
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

impl From<StateUpdate> for core::StateUpdate {
    fn from(value: StateUpdate) -> Self {
        Self {
            block_hash: value.block_hash,
            new_root: value.new_root,
            old_root: value.old_root,
            state_diff: value.state_diff.into(),
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
            declared_contract_hashes: value
                .declared_classes
                .into_iter()
                .map(|item| item.class_hash)
                .collect(),
            deployed_contracts: value
                .deployed_contracts
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

impl From<DeployedContract> for core::DeployedContractItem {
    fn from(value: DeployedContract) -> Self {
        Self {
            address: value.address,
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
            TransactionStatus::Rejected
            | TransactionStatus::AcceptedOnL2
            | TransactionStatus::AcceptedOnL1 => Ok(Self::Receipt(value.try_into()?)),
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
        })
    }
}

impl TryFrom<TransactionReceipt> for core::DeclareTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            status: value.status.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
        })
    }
}

impl TryFrom<TransactionWithReceipt> for core::DeployTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: value.receipt.actual_fee.ok_or(ConversionError)?,
            status: value.receipt.status.try_into()?,
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
        })
    }
}

impl TryFrom<TransactionWithReceipt> for core::DeployAccountTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionWithReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: value.receipt.actual_fee.ok_or(ConversionError)?,
            status: value.receipt.status.try_into()?,
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
        })
    }
}

impl TryFrom<TransactionReceipt> for core::InvokeTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            status: value.status.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
        })
    }
}

impl TryFrom<TransactionReceipt> for core::L1HandlerTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: TransactionReceipt) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.transaction_hash,
            actual_fee: value.actual_fee.ok_or(ConversionError)?,
            status: value.status.try_into()?,
            block_hash: value.block_hash.ok_or(ConversionError)?,
            block_number: value.block_number.ok_or(ConversionError)?,
            messages_sent: value
                .l2_to_l1_messages
                .into_iter()
                .map(|item| item.into())
                .collect(),
            events: value.events.into_iter().map(|item| item.into()).collect(),
        })
    }
}

impl From<L2ToL1Message> for core::MsgToL1 {
    fn from(value: L2ToL1Message) -> Self {
        Self {
            // Unwrapping here is safe
            to_address: FieldElement::from_byte_slice_be(&value.to_address.0).unwrap(),
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

impl TryFrom<TransactionStatus> for core::TransactionStatus {
    type Error = ConversionError;

    fn try_from(value: TransactionStatus) -> Result<Self, Self::Error> {
        match value {
            TransactionStatus::NotReceived | TransactionStatus::Received => Err(ConversionError),
            TransactionStatus::Pending => Ok(Self::Pending),
            TransactionStatus::Rejected => Ok(Self::Rejected),
            TransactionStatus::AcceptedOnL2 => Ok(Self::AcceptedOnL2),
            TransactionStatus::AcceptedOnL1 => Ok(Self::AcceptedOnL1),
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
