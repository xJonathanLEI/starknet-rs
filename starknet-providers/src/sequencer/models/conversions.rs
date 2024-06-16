use std::sync::Arc;

use starknet_core::types::{self as core, contract::legacy as contract_legacy, Felt};

use super::{
    state_update::{DeclaredContract, DeployedContract, StateDiff, StorageDiff},
    trace::{
        CallType, FunctionInvocation, OrderedEventResponse, OrderedL2ToL1MessageResponse,
        TransactionTraceWithHash,
    },
    transaction::{DataAvailabilityMode, ResourceBounds, ResourceBoundsMapping},
    *,
};

#[derive(Debug, thiserror::Error)]
#[error("unable to convert type")]
pub struct ConversionError;

pub(crate) struct ConfirmedReceiptWithContext {
    pub receipt: ConfirmedTransactionReceipt,
    pub transaction: TransactionType,
    pub finality: BlockStatus,
}

pub(crate) struct OrderedL2ToL1MessageResponseWithFromAddress {
    pub message: OrderedL2ToL1MessageResponse,
    pub from: Felt,
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
                    l1_gas_price: value.l1_gas_price,
                    l1_data_gas_price: value.l1_data_gas_price,
                    l1_da_mode: value.l1_da_mode,
                    starknet_version: value.starknet_version.ok_or(ConversionError)?,
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
                l1_gas_price: value.l1_gas_price,
                l1_data_gas_price: value.l1_data_gas_price,
                l1_da_mode: value.l1_da_mode,
                starknet_version: value.starknet_version.ok_or(ConversionError)?,
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
                    l1_gas_price: value.l1_gas_price,
                    l1_data_gas_price: value.l1_data_gas_price,
                    l1_da_mode: value.l1_da_mode,
                    starknet_version: value.starknet_version.ok_or(ConversionError)?,
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
                l1_gas_price: value.l1_gas_price,
                l1_data_gas_price: value.l1_data_gas_price,
                l1_da_mode: value.l1_da_mode,
                starknet_version: value.starknet_version.ok_or(ConversionError)?,
            })),
            // Unknown combination
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<Block> for core::MaybePendingBlockWithReceipts {
    type Error = ConversionError;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        if value.transactions.len() != value.transaction_receipts.len() {
            return Err(ConversionError);
        }

        let mut transactions = vec![];

        for (tx, receipt) in value
            .transactions
            .into_iter()
            .zip(value.transaction_receipts.into_iter())
        {
            let core_tx = tx.clone().try_into()?;

            let tx_with_receipt = ConfirmedReceiptWithContext {
                receipt,
                transaction: tx,
                finality: value.status,
            };

            transactions.push(core::TransactionWithReceipt {
                transaction: core_tx,
                receipt: tx_with_receipt.try_into()?,
            });
        }

        match (value.block_hash, value.block_number, value.state_root) {
            // Confirmed block
            (Some(block_hash), Some(block_number), Some(state_root)) => {
                Ok(Self::Block(core::BlockWithReceipts {
                    status: value.status.try_into()?,
                    block_hash,
                    parent_hash: value.parent_block_hash,
                    block_number,
                    new_root: state_root,
                    timestamp: value.timestamp,
                    sequencer_address: value.sequencer_address.unwrap_or_default(),
                    l1_gas_price: value.l1_gas_price,
                    l1_data_gas_price: value.l1_data_gas_price,
                    l1_da_mode: value.l1_da_mode,
                    starknet_version: value.starknet_version.ok_or(ConversionError)?,
                    transactions,
                }))
            }
            // Pending block
            (None, None, None) => Ok(Self::PendingBlock(core::PendingBlockWithReceipts {
                transactions,
                timestamp: value.timestamp,
                sequencer_address: value.sequencer_address.unwrap_or_default(),
                parent_hash: value.parent_block_hash,
                l1_gas_price: value.l1_gas_price,
                l1_data_gas_price: value.l1_data_gas_price,
                l1_da_mode: value.l1_da_mode,
                starknet_version: value.starknet_version.ok_or(ConversionError)?,
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
            TransactionType::DeployAccount(inner) => Ok(Self::DeployAccount(inner.try_into()?)),
            TransactionType::InvokeFunction(inner) => Ok(Self::Invoke(inner.try_into()?)),
            TransactionType::L1Handler(inner) => Ok(Self::L1Handler(inner.try_into()?)),
        }
    }
}

impl TryFrom<DeclareTransaction> for core::DeclareTransaction {
    type Error = ConversionError;

    fn try_from(value: DeclareTransaction) -> Result<Self, Self::Error> {
        if value.version == Felt::ZERO {
            Ok(Self::V0(core::DeclareTransactionV0 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                class_hash: value.class_hash,
                sender_address: value.sender_address,
            }))
        } else if value.version == Felt::ONE {
            Ok(Self::V1(core::DeclareTransactionV1 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                nonce: value.nonce,
                class_hash: value.class_hash,
                sender_address: value.sender_address,
            }))
        } else if value.version == Felt::TWO {
            Ok(Self::V2(core::DeclareTransactionV2 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                nonce: value.nonce,
                class_hash: value.class_hash,
                compiled_class_hash: value.compiled_class_hash.ok_or(ConversionError)?,
                sender_address: value.sender_address,
            }))
        } else if value.version == Felt::THREE {
            Ok(Self::V3(core::DeclareTransactionV3 {
                transaction_hash: value.transaction_hash,
                sender_address: value.sender_address,
                compiled_class_hash: value.compiled_class_hash.ok_or(ConversionError)?,
                signature: value.signature,
                nonce: value.nonce,
                class_hash: value.class_hash,
                resource_bounds: value.resource_bounds.ok_or(ConversionError)?.into(),
                tip: value.tip.ok_or(ConversionError)?,
                paymaster_data: value.paymaster_data.ok_or(ConversionError)?,
                account_deployment_data: value.account_deployment_data.ok_or(ConversionError)?,
                nonce_data_availability_mode: value
                    .nonce_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
                fee_data_availability_mode: value
                    .fee_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
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
            version: value.version,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
        })
    }
}

impl TryFrom<DeployAccountTransaction> for core::DeployAccountTransaction {
    type Error = ConversionError;

    fn try_from(value: DeployAccountTransaction) -> Result<Self, Self::Error> {
        if value.version == Felt::ONE {
            Ok(Self::V1(core::DeployAccountTransactionV1 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                nonce: value.nonce,
                contract_address_salt: value.contract_address_salt,
                constructor_calldata: value.constructor_calldata,
                class_hash: value.class_hash,
            }))
        } else if value.version == Felt::THREE {
            Ok(Self::V3(core::DeployAccountTransactionV3 {
                transaction_hash: value.transaction_hash,
                signature: value.signature,
                nonce: value.nonce,
                contract_address_salt: value.contract_address_salt,
                constructor_calldata: value.constructor_calldata,
                class_hash: value.class_hash,
                resource_bounds: value.resource_bounds.ok_or(ConversionError)?.into(),
                tip: value.tip.ok_or(ConversionError)?,
                paymaster_data: value.paymaster_data.ok_or(ConversionError)?,
                nonce_data_availability_mode: value
                    .nonce_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
                fee_data_availability_mode: value
                    .fee_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
            }))
        } else {
            Err(ConversionError)
        }
    }
}

impl TryFrom<InvokeFunctionTransaction> for core::InvokeTransaction {
    type Error = ConversionError;

    fn try_from(value: InvokeFunctionTransaction) -> Result<Self, Self::Error> {
        if value.version == Felt::ZERO {
            Ok(Self::V0(core::InvokeTransactionV0 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                contract_address: value.sender_address,
                entry_point_selector: value.entry_point_selector.ok_or(ConversionError)?,
                calldata: value.calldata,
            }))
        } else if value.version == Felt::ONE {
            Ok(Self::V1(core::InvokeTransactionV1 {
                transaction_hash: value.transaction_hash,
                max_fee: value.max_fee.ok_or(ConversionError)?,
                signature: value.signature,
                nonce: value.nonce.ok_or(ConversionError)?,
                sender_address: value.sender_address,
                calldata: value.calldata,
            }))
        } else if value.version == Felt::THREE {
            Ok(Self::V3(core::InvokeTransactionV3 {
                transaction_hash: value.transaction_hash,
                sender_address: value.sender_address,
                calldata: value.calldata,
                signature: value.signature,
                nonce: value.nonce.ok_or(ConversionError)?,
                resource_bounds: value.resource_bounds.ok_or(ConversionError)?.into(),
                tip: value.tip.ok_or(ConversionError)?,
                paymaster_data: value.paymaster_data.ok_or(ConversionError)?,
                account_deployment_data: value.account_deployment_data.ok_or(ConversionError)?,
                nonce_data_availability_mode: value
                    .nonce_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
                fee_data_availability_mode: value
                    .fee_data_availability_mode
                    .ok_or(ConversionError)?
                    .into(),
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
            version: value.version,
            nonce: {
                // TODO: remove this when a proper u64 conversion is implemented for `Felt`
                let nonce_bytes = value.nonce.unwrap_or_default().to_bytes_le();
                if nonce_bytes.iter().skip(8).any(|&x| x != 0) {
                    return Err(ConversionError);
                }
                u64::from_le_bytes(nonce_bytes[..8].try_into().unwrap())
            },
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        })
    }
}

impl From<ResourceBoundsMapping> for core::ResourceBoundsMapping {
    fn from(value: ResourceBoundsMapping) -> Self {
        Self {
            l1_gas: value.l1_gas.into(),
            l2_gas: value.l2_gas.into(),
        }
    }
}

impl From<core::ResourceBoundsMapping> for ResourceBoundsMapping {
    fn from(value: core::ResourceBoundsMapping) -> Self {
        Self {
            l1_gas: value.l1_gas.into(),
            l2_gas: value.l2_gas.into(),
        }
    }
}

impl From<ResourceBounds> for core::ResourceBounds {
    fn from(value: ResourceBounds) -> Self {
        Self {
            max_amount: value.max_amount,
            max_price_per_unit: value.max_price_per_unit,
        }
    }
}

impl From<core::ResourceBounds> for ResourceBounds {
    fn from(value: core::ResourceBounds) -> Self {
        Self {
            max_amount: value.max_amount,
            max_price_per_unit: value.max_price_per_unit,
        }
    }
}

impl From<DataAvailabilityMode> for core::DataAvailabilityMode {
    fn from(value: DataAvailabilityMode) -> Self {
        match value {
            DataAvailabilityMode::L1 => Self::L1,
            DataAvailabilityMode::L2 => Self::L2,
        }
    }
}

impl From<core::DataAvailabilityMode> for DataAvailabilityMode {
    fn from(value: core::DataAvailabilityMode) -> Self {
        match value {
            core::DataAvailabilityMode::L1 => Self::L1,
            core::DataAvailabilityMode::L2 => Self::L2,
        }
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

impl From<L2ToL1Message> for core::MsgToL1 {
    fn from(value: L2ToL1Message) -> Self {
        Self {
            from_address: value.from_address,
            // Unwrapping here is safe
            to_address: Felt::from_bytes_be_slice(&value.to_address.0),
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

impl From<core::BroadcastedInvokeTransaction> for InvokeFunctionTransactionRequest {
    fn from(value: core::BroadcastedInvokeTransaction) -> Self {
        match value {
            core::BroadcastedInvokeTransaction::V1(inner) => Self::V1(inner.into()),
            core::BroadcastedInvokeTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

impl From<core::BroadcastedInvokeTransactionV1> for InvokeFunctionV1TransactionRequest {
    fn from(value: core::BroadcastedInvokeTransactionV1) -> Self {
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

impl From<core::BroadcastedInvokeTransactionV3> for InvokeFunctionV3TransactionRequest {
    fn from(value: core::BroadcastedInvokeTransactionV3) -> Self {
        Self {
            sender_address: value.sender_address,
            calldata: value.calldata,
            signature: value.signature,
            nonce: value.nonce,
            nonce_data_availability_mode: value.nonce_data_availability_mode.into(),
            fee_data_availability_mode: value.fee_data_availability_mode.into(),
            resource_bounds: value.resource_bounds.into(),
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            account_deployment_data: value.account_deployment_data,
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
            core::BroadcastedDeclareTransaction::V3(inner) => Ok(Self::V3(inner.try_into()?)),
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

impl TryFrom<core::BroadcastedDeclareTransactionV3> for DeclareV3TransactionRequest {
    type Error = ConversionError;

    fn try_from(value: core::BroadcastedDeclareTransactionV3) -> Result<Self, Self::Error> {
        Ok(Self {
            contract_class: Arc::new(
                contract::CompressedSierraClass::from_flattened(&value.contract_class)
                    .map_err(|_| ConversionError)?,
            ),
            compiled_class_hash: value.compiled_class_hash,
            sender_address: value.sender_address,
            signature: value.signature,
            nonce: value.nonce,
            nonce_data_availability_mode: value.nonce_data_availability_mode.into(),
            fee_data_availability_mode: value.fee_data_availability_mode.into(),
            resource_bounds: value.resource_bounds.into(),
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            account_deployment_data: value.account_deployment_data,
            is_query: value.is_query,
        })
    }
}

impl From<core::BroadcastedDeployAccountTransaction> for DeployAccountTransactionRequest {
    fn from(value: core::BroadcastedDeployAccountTransaction) -> Self {
        match value {
            core::BroadcastedDeployAccountTransaction::V1(inner) => Self::V1(inner.into()),
            core::BroadcastedDeployAccountTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

impl From<core::BroadcastedDeployAccountTransactionV1> for DeployAccountV1TransactionRequest {
    fn from(value: core::BroadcastedDeployAccountTransactionV1) -> Self {
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

impl From<core::BroadcastedDeployAccountTransactionV3> for DeployAccountV3TransactionRequest {
    fn from(value: core::BroadcastedDeployAccountTransactionV3) -> Self {
        Self {
            class_hash: value.class_hash,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            signature: value.signature,
            nonce: value.nonce,
            nonce_data_availability_mode: value.nonce_data_availability_mode.into(),
            fee_data_availability_mode: value.fee_data_availability_mode.into(),
            resource_bounds: value.resource_bounds.into(),
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            is_query: value.is_query,
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

impl TryFrom<FunctionInvocation> for core::FunctionInvocation {
    type Error = ConversionError;

    fn try_from(value: FunctionInvocation) -> Result<Self, Self::Error> {
        Ok(Self {
            contract_address: value.contract_address,
            entry_point_selector: value.selector.ok_or(ConversionError)?,
            calldata: value.calldata,
            caller_address: value.caller_address,
            class_hash: value.class_hash.ok_or(ConversionError)?,
            entry_point_type: value.entry_point_type.ok_or(ConversionError)?.into(),
            call_type: value.call_type.ok_or(ConversionError)?.into(),
            result: value.result,
            calls: value
                .internal_calls
                .into_iter()
                .map(|call| call.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            events: value.events.into_iter().map(|event| event.into()).collect(),
            messages: value
                .messages
                .into_iter()
                .map(|message| {
                    OrderedL2ToL1MessageResponseWithFromAddress {
                        message,
                        from: value.contract_address,
                    }
                    .into()
                })
                .collect(),
            execution_resources: value.execution_resources.into(),
        })
    }
}

impl From<EntryPointType> for core::EntryPointType {
    fn from(value: EntryPointType) -> Self {
        match value {
            EntryPointType::External => Self::External,
            EntryPointType::L1Handler => Self::L1Handler,
            EntryPointType::Constructor => Self::Constructor,
        }
    }
}

impl From<CallType> for core::CallType {
    fn from(value: CallType) -> Self {
        match value {
            CallType::Call => Self::Call,
            CallType::Delegate => Self::LibraryCall,
        }
    }
}

impl From<OrderedEventResponse> for core::OrderedEvent {
    fn from(value: OrderedEventResponse) -> Self {
        Self {
            keys: value.keys,
            data: value.data,
            order: value.order,
        }
    }
}

impl From<OrderedL2ToL1MessageResponseWithFromAddress> for core::OrderedMessage {
    fn from(value: OrderedL2ToL1MessageResponseWithFromAddress) -> Self {
        Self {
            from_address: value.from,
            // Unwrapping is safe here as H160 is only 20 bytes
            to_address: Felt::from_bytes_be_slice(&value.message.to_address.to_fixed_bytes()),
            payload: value.message.payload,
            order: value.message.order,
        }
    }
}

impl From<ExecutionResources> for core::ComputationResources {
    fn from(value: ExecutionResources) -> Self {
        Self {
            steps: value.n_steps,
            memory_holes: Some(value.n_memory_holes),
            range_check_builtin_applications: value.builtin_instance_counter.range_check_builtin,
            pedersen_builtin_applications: value.builtin_instance_counter.pedersen_builtin,
            poseidon_builtin_applications: value.builtin_instance_counter.poseidon_builtin,
            ec_op_builtin_applications: value.builtin_instance_counter.ec_op_builtin,
            ecdsa_builtin_applications: value.builtin_instance_counter.ecdsa_builtin,
            bitwise_builtin_applications: value.builtin_instance_counter.bitwise_builtin,
            keccak_builtin_applications: value.builtin_instance_counter.keccak_builtin,
            segment_arena_builtin: value.builtin_instance_counter.segment_arena_builtin,
        }
    }
}

impl TryFrom<TransactionStatusInfo> for core::TransactionStatus {
    type Error = ConversionError;

    fn try_from(value: TransactionStatusInfo) -> Result<Self, Self::Error> {
        if let TransactionStatus::Rejected = value.status {
            return Ok(Self::Rejected);
        }

        let exec_status = match value.execution_status.ok_or(ConversionError)? {
            TransactionExecutionStatus::Succeeded => {
                Some(core::TransactionExecutionStatus::Succeeded)
            }
            TransactionExecutionStatus::Reverted => {
                Some(core::TransactionExecutionStatus::Reverted)
            }
            TransactionExecutionStatus::Rejected => None,
        };

        match value.finality_status {
            Some(TransactionFinalityStatus::Received) => Ok(Self::Received),
            Some(TransactionFinalityStatus::AcceptedOnL2) => {
                Ok(Self::AcceptedOnL2(exec_status.ok_or(ConversionError)?))
            }
            Some(TransactionFinalityStatus::AcceptedOnL1) => {
                Ok(Self::AcceptedOnL1(exec_status.ok_or(ConversionError)?))
            }
            // `NotReceived` must be handled on the caller before converting
            _ => Err(ConversionError),
        }
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::TransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        match value.transaction {
            TransactionType::Declare(_) => Ok(Self::Declare(value.try_into()?)),
            TransactionType::Deploy(_) => Ok(Self::Deploy(value.try_into()?)),
            TransactionType::DeployAccount(_) => Ok(Self::DeployAccount(value.try_into()?)),
            TransactionType::InvokeFunction(_) => Ok(Self::Invoke(value.try_into()?)),
            TransactionType::L1Handler(_) => Ok(Self::L1Handler(value.try_into()?)),
        }
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::DeclareTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: core::FeePayment {
                amount: value.receipt.actual_fee,
                unit: core::PriceUnit::Wei,
            },
            finality_status: value.finality.try_into()?,
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
            execution_resources: value
                .receipt
                .execution_resources
                .ok_or(ConversionError)?
                .try_into()?,
            execution_result: convert_execution_result(
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::DeployTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: core::FeePayment {
                amount: value.receipt.actual_fee,
                unit: core::PriceUnit::Wei,
            },
            finality_status: value.finality.try_into()?,
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
            contract_address: match value.transaction {
                TransactionType::Deploy(inner) => inner.contract_address,
                _ => return Err(ConversionError),
            },
            execution_resources: value
                .receipt
                .execution_resources
                .ok_or(ConversionError)?
                .try_into()?,
            execution_result: convert_execution_result(
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::DeployAccountTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: core::FeePayment {
                amount: value.receipt.actual_fee,
                unit: core::PriceUnit::Wei,
            },
            finality_status: value.finality.try_into()?,
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
            contract_address: match value.transaction {
                TransactionType::DeployAccount(inner) => {
                    inner.contract_address.ok_or(ConversionError)?
                }
                _ => return Err(ConversionError),
            },
            execution_resources: value
                .receipt
                .execution_resources
                .ok_or(ConversionError)?
                .try_into()?,
            execution_result: convert_execution_result(
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::InvokeTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: core::FeePayment {
                amount: value.receipt.actual_fee,
                unit: core::PriceUnit::Wei,
            },
            finality_status: value.finality.try_into()?,
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
            execution_resources: value
                .receipt
                .execution_resources
                .ok_or(ConversionError)?
                .try_into()?,
            execution_result: convert_execution_result(
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
        })
    }
}

impl TryFrom<ConfirmedReceiptWithContext> for core::L1HandlerTransactionReceipt {
    type Error = ConversionError;

    fn try_from(value: ConfirmedReceiptWithContext) -> Result<Self, Self::Error> {
        // The sequencer never serves the message hash, so we have to compute it ourselves.
        let l1_handler_tx: core::L1HandlerTransaction = match value.transaction {
            TransactionType::L1Handler(tx) => tx.try_into().map_err(|_| ConversionError)?,
            _ => return Err(ConversionError),
        };
        let msg_to_l2 = l1_handler_tx
            .parse_msg_to_l2()
            .map_err(|_| ConversionError)?;

        Ok(Self {
            transaction_hash: value.receipt.transaction_hash,
            actual_fee: core::FeePayment {
                amount: value.receipt.actual_fee,
                unit: core::PriceUnit::Wei,
            },
            finality_status: value.finality.try_into()?,
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
            execution_resources: value
                .receipt
                .execution_resources
                .ok_or(ConversionError)?
                .try_into()?,
            execution_result: convert_execution_result(
                value.receipt.execution_status,
                value.receipt.revert_error,
            )?,
            message_hash: msg_to_l2.hash(),
        })
    }
}

impl TryFrom<BlockStatus> for core::TransactionFinalityStatus {
    type Error = ConversionError;

    fn try_from(value: BlockStatus) -> Result<Self, Self::Error> {
        match value {
            // Transactions in pending blocks are considered "accepted on L2" now
            BlockStatus::Pending | BlockStatus::AcceptedOnL2 => Ok(Self::AcceptedOnL2),
            BlockStatus::AcceptedOnL1 => Ok(Self::AcceptedOnL1),
            BlockStatus::Aborted | BlockStatus::Reverted => Err(ConversionError),
        }
    }
}

impl TryFrom<ExecutionResources> for core::ExecutionResources {
    type Error = ConversionError;

    fn try_from(value: ExecutionResources) -> Result<Self, Self::Error> {
        Ok(Self {
            computation_resources: core::ComputationResources {
                steps: value.n_steps,
                memory_holes: Some(value.n_memory_holes),
                range_check_builtin_applications: value
                    .builtin_instance_counter
                    .range_check_builtin,
                pedersen_builtin_applications: value.builtin_instance_counter.pedersen_builtin,
                poseidon_builtin_applications: value.builtin_instance_counter.poseidon_builtin,
                ec_op_builtin_applications: value.builtin_instance_counter.ec_op_builtin,
                ecdsa_builtin_applications: value.builtin_instance_counter.ecdsa_builtin,
                bitwise_builtin_applications: value.builtin_instance_counter.bitwise_builtin,
                keccak_builtin_applications: value.builtin_instance_counter.keccak_builtin,
                segment_arena_builtin: value.builtin_instance_counter.segment_arena_builtin,
            },
            data_resources: core::DataResources {
                data_availability: value.data_availability.ok_or(ConversionError)?,
            },
        })
    }
}

fn convert_execution_result(
    execution_status: Option<TransactionExecutionStatus>,
    revert_error: Option<String>,
) -> Result<core::ExecutionResult, ConversionError> {
    match (execution_status, revert_error) {
        (None, None) => {
            // This is a response from pre-v0.12.1. Pre-v0.12.1 transactions are always successful
            // as long as they're in a block.
            //
            // After feeder deprecation, the only way to fetch transaction receipts is by fetching
            // them as part of a block. Therefore, it's always the case that this tx in question is
            // in a block.

            Ok(core::ExecutionResult::Succeeded)
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
