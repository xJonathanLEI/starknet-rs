use super::{
    FieldElement,
    {
        super::serde::unsigned_field_element::{UfeHex, UfePendingBlockHash},
        TransactionStatus,
    },
};

use serde::Deserialize;
use serde_with::serde_as;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionType {
    Declare(DeclareTransaction),
    Deploy(DeployTransaction),
    DeployAccount(DeployAccountTransaction),
    InvokeFunction(InvokeFunctionTransaction),
    L1Handler(L1HandlerTransaction),
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionStatusInfo {
    #[serde(default)]
    #[serde_as(as = "UfePendingBlockHash")]
    pub block_hash: Option<FieldElement>,
    #[serde(alias = "tx_status")]
    pub status: TransactionStatus,
    #[serde(alias = "tx_failure_reason")]
    pub transaction_failure_reason: Option<TransactionFailureReason>,
}
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionFailureReason {
    pub code: String,
    pub error_message: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionInfo {
    #[serde(default)]
    #[serde_as(as = "UfePendingBlockHash")]
    pub block_hash: Option<FieldElement>,
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
    #[serde(rename(deserialize = "transaction"))]
    pub r#type: Option<TransactionType>,
    pub transaction_failure_reason: Option<TransactionFailureReason>,
    pub transaction_index: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum EntryPointType {
    External,
    L1Handler,
    Constructor,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclareTransaction {
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "Option<UfeHex>")]
    pub compiled_class_hash: Option<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub version: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployTransaction {
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub version: FieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployAccountTransaction {
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub version: FieldElement,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InvokeFunctionTransaction {
    #[serde_as(as = "UfeHex")]
    // Need this alias because older blocks still use `contract_address`
    #[serde(alias = "contract_address")]
    pub sender_address: FieldElement,
    #[serde_as(as = "Option<UfeHex>")]
    pub entry_point_selector: Option<FieldElement>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    #[serde_as(as = "Option<UfeHex>")]
    pub nonce: Option<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub version: FieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1HandlerTransaction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde_as(as = "Option<UfeHex>")]
    pub nonce: Option<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub version: FieldElement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_full_invoke_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/1_invoke.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39099));
        if let TransactionType::InvokeFunction(invoke) = tx.r#type.unwrap() {
            assert_eq!(invoke.signature.len(), 2);
        } else {
            panic!("Did not deserialize TransactionType::InvokeFunction properly")
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_full_deploy_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/2_deploy.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39181));
        if let TransactionType::Deploy(deploy) = tx.r#type.unwrap() {
            assert_eq!(deploy.constructor_calldata.len(), 2);
        } else {
            panic!("Did not deserialize TransactionType::Deploy properly");
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction/3_not_received.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, None);
        assert_eq!(tx.status, TransactionStatus::NotReceived);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_failure() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/4_failure.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert!(tx.transaction_failure_reason.is_some());
        let failure_reason = tx.transaction_failure_reason.unwrap();
        assert_eq!(failure_reason.code, "TRANSACTION_FAILED");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_declare_v1_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/5_declare_v1.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::Declare(_) => {}
            _ => panic!("Did not deserialize TransactionType::Declare properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_declare_v2_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/6_declare_v2.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::Declare(_) => {}
            _ => panic!("Did not deserialize TransactionType::Declare properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_accepted() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/1_accepted.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::AcceptedOnL1);
        assert_eq!(
            tx.block_hash,
            Some(
                FieldElement::from_hex_be(
                    "0xca6e3e44d58747b398a0b4e882245c6bc9f5cd666674824e14929708fb8d09"
                )
                .unwrap()
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/2_not_received.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::NotReceived);
        assert!(tx.block_hash.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_failure() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/3_failure.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::Rejected);
        assert!(tx.block_hash.is_none());
        assert!(tx.transaction_failure_reason.is_some());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_pending() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/manual/1_pending.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::Pending);
        assert!(tx.block_hash.is_none());
        assert!(tx.transaction_failure_reason.is_none());
    }
}
