use super::{super::serde::unsigned_field_element::UfeHex, EntryPointType, ExecutionResources};

use ethereum_types::Address;
use serde::Deserialize;
use serde_with::serde_as;
use starknet_crypto::FieldElement;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BlockTraces {
    pub traces: Vec<TransactionTraceWithHash>,
}

/// Represents the trace of a Starknet transaction execution, including internal calls.
#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTrace {
    /// An object describing the invocation of a specific function.
    #[serde(default)]
    pub function_invocation: Option<FunctionInvocation>,
    /// An object describing the invocation of a fee transfer.
    #[serde(default)]
    pub fee_transfer_invocation: Option<FunctionInvocation>,
    /// An object describing the invocation of validation.
    #[serde(default)]
    pub validate_invocation: Option<FunctionInvocation>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionTraceWithHash {
    #[serde(flatten)]
    pub trace: TransactionTrace,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum CallType {
    Call,
    Delegate,
}

/// A lean version of CallInfo class, containing merely the information relevant for the user.
#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FunctionInvocation {
    #[serde_as(as = "UfeHex")]
    pub caller_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
    pub call_type: Option<CallType>,
    // This field is marked optional because it's missing from old transactions. Drop `Option` once
    // it's resolved.
    #[serde_as(as = "Option<UfeHex>")]
    pub class_hash: Option<FieldElement>,
    // This field is marked optional because it's missing from old transactions. Drop `Option` once
    // it's resolved.
    #[serde_as(as = "Option<UfeHex>")]
    pub selector: Option<FieldElement>,
    pub entry_point_type: Option<EntryPointType>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
    pub execution_resources: ExecutionResources,
    pub internal_calls: Vec<FunctionInvocation>,
    pub events: Vec<OrderedEventResponse>,
    pub messages: Vec<OrderedL2ToL1MessageResponse>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct OrderedEventResponse {
    pub order: u64,
    #[serde_as(as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
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
    fn test_block_traces_deser() {
        serde_json::from_str::<BlockTraces>(include_str!(
            "../../test-data/raw_gateway_responses/get_block_traces/1_success.txt"
        ))
        .unwrap();
    }

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

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_trace_deser_with_validation() {
        let trace = serde_json::from_str::<TransactionTrace>(include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_trace/4_with_validation.txt"
        ))
        .unwrap();

        assert!(trace.validate_invocation.is_some());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_trace_deser_new_attributes_0_9_0() {
        // This tx contains new fields introduced in Starknet v0.9.0
        let new_tx: TransactionTrace = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_trace/3_with_call_type.txt"
        ))
        .unwrap();
        match &new_tx.function_invocation.as_ref().unwrap().call_type {
            Some(call_type) => assert_eq!(call_type, &CallType::Call),
            None => panic!("Empty call_type"),
        }
        assert!(&new_tx.function_invocation.unwrap().class_hash.is_some());

        let old_tx: TransactionTrace = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_trace/1_with_messages.txt"
        ))
        .unwrap();
        assert!(&old_tx
            .function_invocation
            .as_ref()
            .unwrap()
            .call_type
            .is_none());
        assert!(&old_tx.function_invocation.unwrap().class_hash.is_none());
    }
}
