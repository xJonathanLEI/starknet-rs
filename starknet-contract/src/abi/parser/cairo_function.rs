use starknet_core::types::contract::{AbiNamedMember, AbiOutput, StateMutability};

use super::abi_types::AbiTypeAny;

#[derive(Debug, Clone)]
pub struct CairoFunction {
    pub name: String,
    pub state_mutability: StateMutability,
    pub inputs: Vec<(String, AbiTypeAny)>,
    // For now, only one output type is supported (or none).
    // TODO: investigate the cases where more than one output is
    // present in the ABI.
    pub output: Option<AbiTypeAny>,
}

impl CairoFunction {
    /// Initializes a new instance from the abi name and it's members.
    pub fn new(
        abi_name: &str,
        state_mutability: StateMutability,
        inputs: &[AbiNamedMember],
        outputs: &Vec<AbiOutput>,
    ) -> CairoFunction {
        let name = abi_name.to_string();

        let output = if !outputs.is_empty() {
            // For now, only first output is considered.
            // TODO: investigate when we can have several outputs.
            Some(AbiTypeAny::from_string(&outputs[0].r#type))
        } else {
            None
        };

        let inputs = inputs
            .iter()
            .map(|i| (i.name.clone(), AbiTypeAny::from_string(&i.r#type)))
            .collect();

        CairoFunction {
            name,
            state_mutability,
            inputs,
            output,
        }
    }
}
