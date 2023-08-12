use serde::Deserialize;
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::{UfeHex, UfePendingBlockHash},
    types::FieldElement,
};

use super::{L1Address, TransactionFailureReason};

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Receipt {
    #[serde(default)]
    #[serde_as(as = "UfePendingBlockHash")]
    pub block_hash: Option<FieldElement>,
    pub block_number: Option<u64>,
    pub events: Vec<Event>,
    #[serde(default)]
    pub execution_resources: Option<ExecutionResources>,
    pub l1_to_l2_consumed_message: Option<L1ToL2Message>,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
    #[serde(default)]
    pub execution_status: Option<TransactionExecutionStatus>,
    #[serde(default)]
    pub revert_error: Option<String>,
    // This field is actually always present since v0.12.1, but we're keeping it optional until
    // mainnet is upgraded.
    #[serde(default)]
    pub finality_status: Option<TransactionFinalityStatus>,
    pub status: TransactionStatus,
    pub transaction_failure_reason: Option<TransactionFailureReason>,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    pub transaction_index: Option<u64>,
    #[serde_as(as = "Option<UfeHex>")]
    pub actual_fee: Option<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ConfirmedReceipt {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    pub transaction_index: u64,
    // This field is actually always present since v0.12.1, but we're keeping it optional until
    // mainnet is upgraded.
    #[serde(default)]
    pub execution_status: Option<TransactionExecutionStatus>,
    #[serde(default)]
    pub revert_error: Option<String>,
    #[serde(default)]
    pub execution_resources: Option<ExecutionResources>,
    pub l1_to_l2_consumed_message: Option<L1ToL2Message>,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
    pub events: Vec<Event>,
    #[serde_as(as = "UfeHex")]
    pub actual_fee: FieldElement,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionStatus {
    /// Transaction has not been received yet (i.e. not written to storage)
    NotReceived,
    /// Transaction was received by the sequenced
    Received,
    /// Transaction passed teh validation and entered the pending block
    Pending,
    /// The transaction failed validation and was skipped (applies both to a
    /// pending and actual created block)
    Rejected,
    Reverted,
    /// Transaction passed teh validation and entered a created block
    AcceptedOnL2,
    /// Transaction was accepted on-chain
    AcceptedOnL1,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionExecutionStatus {
    Succeeded,
    Reverted,
    Rejected,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionFinalityStatus {
    NotReceived,
    Received,
    AcceptedOnL2,
    AcceptedOnL1,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ExecutionResources {
    pub n_steps: u64,
    pub n_memory_holes: u64,
    pub builtin_instance_counter: BuiltinInstanceCounter,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BuiltinInstanceCounter {
    pub pedersen_builtin: Option<u64>,
    pub range_check_builtin: Option<u64>,
    pub bitwise_builtin: Option<u64>,
    pub output_builtin: Option<u64>,
    pub ecdsa_builtin: Option<u64>,
    pub ec_op_builtin: Option<u64>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1ToL2Message {
    pub from_address: L1Address,
    #[serde_as(as = "UfeHex")]
    pub to_address: FieldElement,
    #[serde_as(deserialize_as = "UfeHex")]
    pub selector: FieldElement,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
    #[serde_as(deserialize_as = "Option<UfeHex>")]
    pub nonce: Option<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L2ToL1Message {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    pub to_address: L1Address,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub payload: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    #[serde_as(as = "UfeHex")]
    pub from_address: FieldElement,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub keys: Vec<FieldElement>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub data: Vec<FieldElement>,
}

#[cfg(test)]
mod tests {
    use super::{super::TransactionStatusInfo, *};

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_accepted() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/1_accepted.txt"
        );

        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.status, TransactionStatus::AcceptedOnL1);
        assert_eq!(receipt.block_number, Some(39207));
        assert_eq!(receipt.execution_resources.unwrap().n_steps, 489);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_not_received() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/2_not_received.txt"
        );
        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.status, TransactionStatus::NotReceived);
        assert_eq!(
            receipt.transaction_hash,
            FieldElement::from_hex_be(
                "0x0000000000000000000000000000000000000000000000000000000000000000"
            )
            .unwrap()
        );
        assert_eq!(receipt.block_hash, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_with_events() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/3_with_events.txt"
        );
        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.events[0].data.len(), 2);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_failure() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/4_failure.txt"
        );
        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.status, TransactionStatus::Rejected);
        assert!(receipt.transaction_failure_reason.is_some());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_declare_v1() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/5_declare_v1.txt"
        );
        serde_json::from_str::<Receipt>(raw).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_declare_v2() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/6_declare_v2.txt"
        );
        serde_json::from_str::<Receipt>(raw).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_receipt_deser_reverted_tx() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_receipt/7_reverted.txt"
        );
        serde_json::from_str::<Receipt>(raw).unwrap();
    }
}
