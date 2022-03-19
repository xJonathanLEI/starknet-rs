use super::{super::serde::unsigned_field_element::UfeHex, EntryPointType, ExecutionResources};

use ethereum_types::Address;
use serde::Deserialize;
use serde_with::serde_as;
use starknet_crypto::FieldElement;

/// Represents the trace of a StarkNet transaction execution, including internal calls.
#[derive(Debug, Deserialize)]
pub struct TransactionTrace {
    /// An object describing the invocation of a specific function.
    pub function_invocation: FunctionInvocation,
    pub signature: Vec<FieldElement>,
}

/// A lean version of CallInfo class, containing merely the information relevant for the user.
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct FunctionInvocation {
    #[serde_as(as = "UfeHex")]
    pub caller_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "Option<UfeHex>")]
    pub code_address: Option<FieldElement>,
    #[serde_as(as = "Option<UfeHex>")]
    pub selector: Option<FieldElement>,
    pub entry_point_type: Option<EntryPointType>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
    pub execution_resources: ExecutionResources,
    pub internal_calls: Vec<FunctionInvocation>,
    pub events: Vec<OrderedEventResponse>,
    pub messages: Vec<OrderedL2ToL1MessageResponse>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct OrderedEventResponse {
    pub order: u64,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct OrderedL2ToL1MessageResponse {
    pub order: u64,
    pub to_address: Address,
    #[serde_as(as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_trace_deser_with_messages() {
        serde_json::from_str::<TransactionTrace>(include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_trace/1_with_messages.txt"
        ))
        .unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_trace_deser_with_events() {
        serde_json::from_str::<TransactionTrace>(include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_trace/2_with_events.txt"
        ))
        .unwrap();
    }
}
