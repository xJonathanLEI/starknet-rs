use super::{
    contract::legacy::{
        RawLegacyAbiEntry, RawLegacyConstructor, RawLegacyEvent, RawLegacyFunction,
        RawLegacyL1Handler, RawLegacyMember, RawLegacyStruct,
    },
    *,
};

impl From<LegacyContractAbiEntry> for RawLegacyAbiEntry {
    fn from(value: LegacyContractAbiEntry) -> Self {
        match value {
            LegacyContractAbiEntry::Function(inner) => match inner.r#type {
                LegacyFunctionAbiType::Function => Self::Function(RawLegacyFunction {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                    state_mutability: inner.state_mutability,
                }),
                LegacyFunctionAbiType::L1Handler => Self::L1Handler(RawLegacyL1Handler {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                }),
                LegacyFunctionAbiType::Constructor => Self::Constructor(RawLegacyConstructor {
                    inputs: inner.inputs,
                    name: inner.name,
                    outputs: inner.outputs,
                }),
            },
            LegacyContractAbiEntry::Event(inner) => Self::Event(RawLegacyEvent {
                data: inner.data,
                keys: inner.keys,
                name: inner.name,
            }),
            LegacyContractAbiEntry::Struct(inner) => Self::Struct(RawLegacyStruct {
                members: inner
                    .members
                    .into_iter()
                    .map(|item| RawLegacyMember {
                        name: item.name,
                        offset: item.offset,
                        r#type: item.r#type,
                    })
                    .collect(),
                name: inner.name,
                size: inner.size,
            }),
        }
    }
}

impl From<Transaction> for TransactionContent {
    fn from(value: Transaction) -> Self {
        match value {
            Transaction::Invoke(inner) => Self::Invoke(inner.into()),
            Transaction::L1Handler(inner) => Self::L1Handler(inner.into()),
            Transaction::Declare(inner) => Self::Declare(inner.into()),
            Transaction::Deploy(inner) => Self::Deploy(inner.into()),
            Transaction::DeployAccount(inner) => Self::DeployAccount(inner.into()),
        }
    }
}

impl From<InvokeTransaction> for InvokeTransactionContent {
    fn from(value: InvokeTransaction) -> Self {
        match value {
            InvokeTransaction::V0(inner) => Self::V0(inner.into()),
            InvokeTransaction::V1(inner) => Self::V1(inner.into()),
            InvokeTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

impl From<L1HandlerTransaction> for L1HandlerTransactionContent {
    fn from(value: L1HandlerTransaction) -> Self {
        Self {
            version: value.version,
            nonce: value.nonce,
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        }
    }
}

impl From<DeclareTransaction> for DeclareTransactionContent {
    fn from(value: DeclareTransaction) -> Self {
        match value {
            DeclareTransaction::V0(inner) => Self::V0(inner.into()),
            DeclareTransaction::V1(inner) => Self::V1(inner.into()),
            DeclareTransaction::V2(inner) => Self::V2(inner.into()),
            DeclareTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

impl From<DeployTransaction> for DeployTransactionContent {
    fn from(value: DeployTransaction) -> Self {
        Self {
            version: value.version,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeployAccountTransaction> for DeployAccountTransactionContent {
    fn from(value: DeployAccountTransaction) -> Self {
        match value {
            DeployAccountTransaction::V1(inner) => Self::V1(inner.into()),
            DeployAccountTransaction::V3(inner) => Self::V3(inner.into()),
        }
    }
}

impl From<InvokeTransactionV0> for InvokeTransactionV0Content {
    fn from(value: InvokeTransactionV0) -> Self {
        Self {
            max_fee: value.max_fee,
            signature: value.signature,
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        }
    }
}

impl From<InvokeTransactionV1> for InvokeTransactionV1Content {
    fn from(value: InvokeTransactionV1) -> Self {
        Self {
            sender_address: value.sender_address,
            calldata: value.calldata,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
        }
    }
}

impl From<InvokeTransactionV3> for InvokeTransactionV3Content {
    fn from(value: InvokeTransactionV3) -> Self {
        Self {
            sender_address: value.sender_address,
            calldata: value.calldata,
            signature: value.signature,
            nonce: value.nonce,
            resource_bounds: value.resource_bounds,
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            account_deployment_data: value.account_deployment_data,
            nonce_data_availability_mode: value.nonce_data_availability_mode,
            fee_data_availability_mode: value.fee_data_availability_mode,
        }
    }
}

impl From<DeclareTransactionV0> for DeclareTransactionV0Content {
    fn from(value: DeclareTransactionV0) -> Self {
        Self {
            sender_address: value.sender_address,
            max_fee: value.max_fee,
            signature: value.signature,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeclareTransactionV1> for DeclareTransactionV1Content {
    fn from(value: DeclareTransactionV1) -> Self {
        Self {
            sender_address: value.sender_address,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeclareTransactionV2> for DeclareTransactionV2Content {
    fn from(value: DeclareTransactionV2) -> Self {
        Self {
            sender_address: value.sender_address,
            compiled_class_hash: value.compiled_class_hash,
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeclareTransactionV3> for DeclareTransactionV3Content {
    fn from(value: DeclareTransactionV3) -> Self {
        Self {
            sender_address: value.sender_address,
            compiled_class_hash: value.compiled_class_hash,
            signature: value.signature,
            nonce: value.nonce,
            class_hash: value.class_hash,
            resource_bounds: value.resource_bounds,
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            account_deployment_data: value.account_deployment_data,
            nonce_data_availability_mode: value.nonce_data_availability_mode,
            fee_data_availability_mode: value.fee_data_availability_mode,
        }
    }
}

impl From<DeployAccountTransactionV1> for DeployAccountTransactionV1Content {
    fn from(value: DeployAccountTransactionV1) -> Self {
        Self {
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            class_hash: value.class_hash,
        }
    }
}

impl From<DeployAccountTransactionV3> for DeployAccountTransactionV3Content {
    fn from(value: DeployAccountTransactionV3) -> Self {
        Self {
            signature: value.signature,
            nonce: value.nonce,
            contract_address_salt: value.contract_address_salt,
            constructor_calldata: value.constructor_calldata,
            class_hash: value.class_hash,
            resource_bounds: value.resource_bounds,
            tip: value.tip,
            paymaster_data: value.paymaster_data,
            nonce_data_availability_mode: value.nonce_data_availability_mode,
            fee_data_availability_mode: value.fee_data_availability_mode,
        }
    }
}
