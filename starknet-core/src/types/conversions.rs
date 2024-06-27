use super::{
    contract::legacy::{
        RawLegacyAbiEntry, RawLegacyConstructor, RawLegacyEvent, RawLegacyFunction,
        RawLegacyL1Handler, RawLegacyMember, RawLegacyStruct,
    },
    LegacyContractAbiEntry, LegacyFunctionAbiType,
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
