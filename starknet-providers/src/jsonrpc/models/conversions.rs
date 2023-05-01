use std::io::Read;

use flate2::read::GzDecoder;
use starknet_core::{self as core, types::contract::legacy as legacy_contract};

use super::*;

#[derive(Debug, thiserror::Error)]
#[error("Unable to decompress Sierra class")]
pub struct DecompressionError;

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

impl From<core::types::CallFunction> for FunctionCall {
    fn from(value: core::types::CallFunction) -> Self {
        Self {
            contract_address: value.contract_address,
            entry_point_selector: value.entry_point_selector,
            calldata: value.calldata,
        }
    }
}

impl From<TransactionStatus> for core::types::TransactionStatus {
    fn from(value: TransactionStatus) -> Self {
        match value {
            TransactionStatus::Pending => Self::Pending,
            TransactionStatus::AcceptedOnL2 => Self::AcceptedOnL2,
            TransactionStatus::AcceptedOnL1 => Self::AcceptedOnL1,
            TransactionStatus::Rejected => Self::Rejected,
        }
    }
}

impl From<MaybePendingTransactionReceipt> for core::types::TransactionStatusInfo {
    fn from(value: MaybePendingTransactionReceipt) -> Self {
        match value {
            MaybePendingTransactionReceipt::Receipt(receipt) => Self {
                block_hash: Some(match &receipt {
                    TransactionReceipt::Invoke(receipt) => receipt.block_hash,
                    TransactionReceipt::L1Handler(receipt) => receipt.block_hash,
                    TransactionReceipt::Declare(receipt) => receipt.block_hash,
                    TransactionReceipt::Deploy(receipt) => receipt.block_hash,
                    TransactionReceipt::DeployAccount(receipt) => receipt.block_hash,
                }),
                status: match receipt {
                    TransactionReceipt::Invoke(receipt) => receipt.status.into(),
                    TransactionReceipt::L1Handler(receipt) => receipt.status.into(),
                    TransactionReceipt::Declare(receipt) => receipt.status.into(),
                    TransactionReceipt::Deploy(receipt) => receipt.status.into(),
                    TransactionReceipt::DeployAccount(receipt) => receipt.status.into(),
                },
                transaction_failure_reason: None,
            },
            MaybePendingTransactionReceipt::PendingReceipt(_) => Self {
                block_hash: None,
                status: starknet_core::types::TransactionStatus::Pending,
                transaction_failure_reason: None,
            },
        }
    }
}

impl From<core::types::DeclareV1TransactionRequest> for BroadcastedDeclareTransactionV1 {
    fn from(value: core::types::DeclareV1TransactionRequest) -> Self {
        Self {
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            contract_class: value.contract_class.as_ref().clone().into(),
            sender_address: value.sender_address,
        }
    }
}

impl TryFrom<core::types::DeclareV2TransactionRequest> for BroadcastedDeclareTransactionV2 {
    type Error = DecompressionError;

    fn try_from(value: core::types::DeclareV2TransactionRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            contract_class: value.contract_class.as_ref().clone().try_into()?,
            compiled_class_hash: value.compiled_class_hash,
            sender_address: value.sender_address,
        })
    }
}

impl TryFrom<core::types::DeclareTransactionRequest> for BroadcastedDeclareTransaction {
    type Error = DecompressionError;

    fn try_from(value: core::types::DeclareTransactionRequest) -> Result<Self, Self::Error> {
        Ok(match value {
            core::types::DeclareTransactionRequest::V1(value) => Self::V1(value.into()),
            core::types::DeclareTransactionRequest::V2(value) => Self::V2(value.try_into()?),
        })
    }
}

impl TryFrom<core::types::DeclareTransactionRequest> for BroadcastedTransaction {
    type Error = DecompressionError;

    fn try_from(value: core::types::DeclareTransactionRequest) -> Result<Self, Self::Error> {
        Ok(Self::Declare(value.try_into()?))
    }
}

impl From<core::types::InvokeFunctionTransactionRequest> for BroadcastedInvokeTransactionV1 {
    fn from(value: core::types::InvokeFunctionTransactionRequest) -> Self {
        Self {
            max_fee: value.max_fee,
            signature: value.signature,
            nonce: value.nonce,
            sender_address: value.sender_address,
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

impl TryFrom<core::types::AccountTransaction> for BroadcastedTransaction {
    type Error = DecompressionError;

    fn try_from(value: core::types::AccountTransaction) -> Result<Self, Self::Error> {
        Ok(match value {
            core::types::AccountTransaction::Declare(tx) => tx.try_into()?,
            core::types::AccountTransaction::InvokeFunction(tx) => tx.into(),
            core::types::AccountTransaction::DeployAccount(tx) => tx.into(),
        })
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

impl From<SierraContractClass> for ContractClass {
    fn from(value: SierraContractClass) -> Self {
        Self::Sierra(value)
    }
}

impl TryFrom<core::types::contract::CompressedSierraClass> for SierraContractClass {
    type Error = DecompressionError;

    fn try_from(value: core::types::contract::CompressedSierraClass) -> Result<Self, Self::Error> {
        // Unfortunately we need to decompress the bytecodes back to Vec because currently the
        // `Provider` interface is modeled after the sequencer gateway API. This should no longer
        // be needed after this resolves:
        //
        //   https://github.com/xJonathanLEI/starknet-rs/issues/173

        #[serde_as]
        #[derive(Serialize)]
        struct SierraProgram<'a>(#[serde_as(as = "Vec<UfeHex>")] &'a Vec<FieldElement>);

        // Use best compression level to optimize for payload size
        let mut gzip_decoder = GzDecoder::new(&value.sierra_program[..]);
        let mut program_json = String::new();
        gzip_decoder
            .read_to_string(&mut program_json)
            .map_err(|_| DecompressionError)?;

        Ok(SierraContractClass {
            sierra_program: serde_json::from_str(&program_json).map_err(|_| DecompressionError)?,
            entry_points_by_type: value.entry_points_by_type.into(),
            abi: value.abi,
            contract_class_version: value.contract_class_version,
        })
    }
}

impl From<core::types::contract::EntrypointList<core::types::contract::SierraClassEntrypoint>>
    for EntryPointsByType
{
    fn from(
        value: core::types::contract::EntrypointList<core::types::contract::SierraClassEntrypoint>,
    ) -> Self {
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

impl From<core::types::contract::SierraClassEntrypoint> for ContractEntryPoint {
    fn from(value: core::types::contract::SierraClassEntrypoint) -> Self {
        Self {
            function_idx: value.function_idx,
            selector: value.selector,
        }
    }
}

impl From<LegacyContractClass> for ContractClass {
    fn from(value: LegacyContractClass) -> Self {
        Self::Legacy(value)
    }
}

impl From<legacy_contract::CompressedLegacyContractClass> for LegacyContractClass {
    fn from(value: legacy_contract::CompressedLegacyContractClass) -> Self {
        LegacyContractClass {
            program: value.program,
            entry_points_by_type: value.entry_points_by_type.into(),
            abi: value
                .abi
                .map(|abi| abi.into_iter().map(|item| item.into()).collect()),
        }
    }
}

impl From<legacy_contract::LegacyEntryPoints> for LegacyEntryPointsByType {
    fn from(value: legacy_contract::LegacyEntryPoints) -> Self {
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

impl From<legacy_contract::LegacyEntryPoint> for LegacyContractEntryPoint {
    fn from(value: legacy_contract::LegacyEntryPoint) -> Self {
        Self {
            offset: value.offset.into(),
            selector: value.selector,
        }
    }
}

impl From<legacy_contract::LegacyAbiEntry> for ContractAbiEntry {
    fn from(value: legacy_contract::LegacyAbiEntry) -> Self {
        match value {
            legacy_contract::LegacyAbiEntry::Constructor(entry) => Self::Function(entry.into()),
            legacy_contract::LegacyAbiEntry::Function(entry) => Self::Function(entry.into()),
            legacy_contract::LegacyAbiEntry::Struct(entry) => Self::Struct(entry.into()),
            legacy_contract::LegacyAbiEntry::L1Handler(entry) => Self::Function(entry.into()),
            legacy_contract::LegacyAbiEntry::Event(entry) => Self::Event(entry.into()),
        }
    }
}

impl From<legacy_contract::LegacyConstructor> for FunctionAbiEntry {
    fn from(value: legacy_contract::LegacyConstructor) -> Self {
        Self {
            r#type: FunctionAbiType::Constructor,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: None,
        }
    }
}

impl From<legacy_contract::LegacyFunction> for FunctionAbiEntry {
    fn from(value: legacy_contract::LegacyFunction) -> Self {
        Self {
            r#type: FunctionAbiType::Function,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: value.state_mutability,
        }
    }
}

impl From<legacy_contract::LegacyStruct> for StructAbiEntry {
    fn from(value: legacy_contract::LegacyStruct) -> Self {
        Self {
            r#type: StructAbiType::Struct,
            name: value.name,
            size: value.size,
            members: value.members.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<legacy_contract::LegacyL1Handler> for FunctionAbiEntry {
    fn from(value: legacy_contract::LegacyL1Handler) -> Self {
        Self {
            r#type: FunctionAbiType::L1Handler,
            name: value.name,
            inputs: value.inputs.into_iter().map(|item| item.into()).collect(),
            outputs: value.outputs.into_iter().map(|item| item.into()).collect(),
            state_mutability: None,
        }
    }
}

impl From<legacy_contract::LegacyEvent> for EventAbiEntry {
    fn from(value: legacy_contract::LegacyEvent) -> Self {
        Self {
            r#type: EventAbiType::Event,
            name: value.name,
            keys: value.keys.into_iter().map(|item| item.into()).collect(),
            data: value.data.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<legacy_contract::LegacyMember> for StructMember {
    fn from(value: legacy_contract::LegacyMember) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
            offset: value.offset,
        }
    }
}

impl From<legacy_contract::LegacyInput> for TypedParameter {
    fn from(value: legacy_contract::LegacyInput) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}

impl From<legacy_contract::LegacyOutput> for TypedParameter {
    fn from(value: legacy_contract::LegacyOutput) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}

impl From<legacy_contract::LegacyEventData> for TypedParameter {
    fn from(value: legacy_contract::LegacyEventData) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type,
        }
    }
}
