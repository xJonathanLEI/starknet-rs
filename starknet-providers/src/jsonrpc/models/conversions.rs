use super::*;

use starknet_core as core;

impl From<core::types::BlockId> for BlockId {
    fn from(value: core::types::BlockId) -> Self {
        match value {
            core::types::BlockId::Hash(hash) => Self::Hash(hash),
            core::types::BlockId::Number(num) => Self::Number(num),
            core::types::BlockId::Pending => Self::Tag(BlockTag::Pending),
            core::types::BlockId::Latest => Self::Tag(BlockTag::Latest),
        }
    }
}

impl From<FeeEstimate> for core::types::FeeEstimate {
    fn from(value: FeeEstimate) -> Self {
        Self {
            overall_fee: value.overall_fee,
            unit: core::types::FeeUnit::Wei,
            gas_price: value.gas_price,
            gas_usage: value.gas_consumed,
        }
    }
}

impl From<core::types::DeclareTransactionRequest> for BroadcastedDeclareTransaction {
    fn from(value: core::types::DeclareTransactionRequest) -> Self {
        Self {
            max_fee: value.max_fee,
            version: 1,
            signature: value.signature,
            nonce: value.nonce,
            contract_class: value.contract_class.as_ref().clone().into(),
            sender_address: value.sender_address,
        }
    }
}

impl From<core::types::DeclareTransactionRequest> for BroadcastedTransaction {
    fn from(value: core::types::DeclareTransactionRequest) -> Self {
        Self::Declare(value.into())
    }
}

impl From<core::types::InvokeFunctionTransactionRequest> for BroadcastedInvokeTransactionV1 {
    fn from(value: core::types::InvokeFunctionTransactionRequest) -> Self {
        Self {
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            sender_address: value.contract_address,
            calldata: value.calldata,
        }
    }
}

impl From<core::types::InvokeFunctionTransactionRequest> for BroadcastedInvokeTransaction {
    fn from(value: core::types::InvokeFunctionTransactionRequest) -> Self {
        Self::V1(value.into())
    }
}

impl From<core::types::InvokeFunctionTransactionRequest> for BroadcastedTransaction {
    fn from(value: core::types::InvokeFunctionTransactionRequest) -> Self {
        Self::Invoke(value.into())
    }
}

impl From<core::types::DeployAccountTransactionRequest> for BroadcastedDeployAccountTransaction {
    fn from(value: core::types::DeployAccountTransactionRequest) -> Self {
        Self {
            max_fee: value.max_fee,
            version: 1,
            signature: value.signature,
            nonce: value.nonce,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            class_hash: value.class_hash,
        }
    }
}

impl From<core::types::DeployAccountTransactionRequest> for BroadcastedTransaction {
    fn from(value: core::types::DeployAccountTransactionRequest) -> Self {
        Self::DeployAccount(value.into())
    }
}

impl From<core::types::AccountTransaction> for BroadcastedTransaction {
    fn from(value: core::types::AccountTransaction) -> Self {
        match value {
            core::types::AccountTransaction::Declare(tx) => tx.into(),
            core::types::AccountTransaction::InvokeFunction(tx) => tx.into(),
            core::types::AccountTransaction::DeployAccount(tx) => tx.into(),
        }
    }
}

impl From<DeclareTransactionResult> for core::types::AddTransactionResult {
    fn from(value: DeclareTransactionResult) -> Self {
        Self {
            code: core::types::AddTransactionResultCode::TransactionReceived,
            transaction_hash: value.transaction_hash,
            address: None,
            class_hash: Some(value.class_hash),
        }
    }
}

impl From<InvokeTransactionResult> for core::types::AddTransactionResult {
    fn from(value: InvokeTransactionResult) -> Self {
        Self {
            code: core::types::AddTransactionResultCode::TransactionReceived,
            transaction_hash: value.transaction_hash,
            address: None,
            class_hash: None,
        }
    }
}

impl From<DeployAccountTransactionResult> for core::types::AddTransactionResult {
    fn from(value: DeployAccountTransactionResult) -> Self {
        Self {
            code: core::types::AddTransactionResultCode::TransactionReceived,
            transaction_hash: value.transaction_hash,
            address: Some(value.contract_address),
            class_hash: None,
        }
    }
}

impl From<core::types::ContractDefinition> for ContractClass {
    fn from(value: core::types::ContractDefinition) -> Self {
        Self {
            program: value.program,
            entry_points_by_type: value.entry_points_by_type.into(),
            abi: value
                .abi
                .map(|abi| abi.into_iter().map(|item| item.into()).collect()),
        }
    }
}

impl From<core::types::EntryPointsByType> for EntryPointsByType {
    fn from(value: core::types::EntryPointsByType) -> Self {
        Self {
            constructor: value
                .constructor
                .into_iter()
                .map(|item| item.into())
                .collect(),
            external: value.external.into_iter().map(|item| item.into()).collect(),
            l1_handler: value
                .l1_handler
                .into_iter()
                .map(|item| item.into())
                .collect(),
        }
    }
}

impl From<core::types::EntryPoint> for ContractEntryPoint {
    fn from(value: core::types::EntryPoint) -> Self {
        Self {
            offset: value.offset,
            selector: value.selector,
        }
    }
}

impl From<core::types::AbiEntry> for ContractAbiEntry {
    fn from(value: core::types::AbiEntry) -> Self {
        match value {
            core::types::AbiEntry::Constructor(entry) => Self::Function(entry.into()),
            core::types::AbiEntry::Function(entry) => Self::Function(entry.into()),
            core::types::AbiEntry::Struct(entry) => Self::Struct(entry.into()),
            core::types::AbiEntry::L1Handler(entry) => Self::Function(entry.into()),
            core::types::AbiEntry::Event(entry) => Self::Event(entry.into()),
        }
    }
}

impl From<core::types::AbiConstructorEntry> for FunctionAbiEntry {
    fn from(value: core::types::AbiConstructorEntry) -> Self {
        Self {
            r#type: FunctionAbiType::Constructor,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: None,
        }
    }
}

impl From<core::types::AbiFunctionEntry> for FunctionAbiEntry {
    fn from(value: core::types::AbiFunctionEntry) -> Self {
        Self {
            r#type: FunctionAbiType::Function,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: value.state_mutability,
        }
    }
}

impl From<core::types::AbiStructEntry> for StructAbiEntry {
    fn from(value: core::types::AbiStructEntry) -> Self {
        Self {
            r#type: StructAbiType::Struct,
            name: value.name,
            size: value.size,
            members: value.members.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<core::types::AbiL1HandlerEntry> for FunctionAbiEntry {
    fn from(value: core::types::AbiL1HandlerEntry) -> Self {
        Self {
            r#type: FunctionAbiType::L1Handler,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: None,
        }
    }
}

impl From<core::types::AbiEventEntry> for EventAbiEntry {
    fn from(value: core::types::AbiEventEntry) -> Self {
        Self {
            r#type: EventAbiType::Event,
            name: value.name,
            keys: value.keys.into_iter().map(|item| item.into()).collect(),
            data: value.data.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<core::types::AbiStructMember> for StructMember {
    fn from(value: core::types::AbiStructMember) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
            offset: value.offset,
        }
    }
}

impl From<core::types::AbiInput> for TypedParameter {
    fn from(value: core::types::AbiInput) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}

impl From<core::types::AbiOutput> for TypedParameter {
    fn from(value: core::types::AbiOutput) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}

impl From<core::types::AbiEventData> for TypedParameter {
    fn from(value: core::types::AbiEventData) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}
