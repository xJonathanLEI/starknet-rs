use serde::{de::Visitor, Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::{UfeHex, UfePendingBlockHash},
    types::Felt,
};

use super::{
    serde_impls::{u128_hex, u64_hex, u64_hex_opt},
    transaction_receipt::{TransactionExecutionStatus, TransactionFinalityStatus},
    TransactionStatus,
};

#[derive(Debug, Clone, Deserialize)]
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
    pub block_hash: Option<Felt>,
    #[serde(alias = "tx_status")]
    pub status: TransactionStatus,
    // This field is actually always present since v0.12.1, but we're keeping it optional until
    // mainnet is upgraded.
    #[serde(default)]
    pub finality_status: Option<TransactionFinalityStatus>,
    #[serde(default)]
    #[serde(alias = "tx_revert_reason")]
    pub transaction_revert_reason: Option<String>,
    #[serde(default)]
    pub execution_status: Option<TransactionExecutionStatus>,
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
    pub block_hash: Option<Felt>,
    pub block_number: Option<u64>,
    pub status: TransactionStatus,
    // This field is actually always present since v0.12.1, but we're keeping it optional until
    // mainnet is upgraded.
    #[serde(default)]
    pub finality_status: Option<TransactionFinalityStatus>,
    #[serde(default)]
    pub revert_error: Option<String>,
    #[serde(default)]
    pub execution_status: Option<TransactionExecutionStatus>,
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
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclareTransaction {
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde_as(as = "Option<UfeHex>")]
    pub compiled_class_hash: Option<Felt>,
    #[serde_as(as = "UfeHex")]
    pub sender_address: Felt,
    #[serde_as(as = "UfeHex")]
    pub nonce: Felt,
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub max_fee: Option<Felt>,
    #[serde_as(as = "UfeHex")]
    pub version: Felt,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<Felt>,
    pub nonce_data_availability_mode: Option<DataAvailabilityMode>,
    pub fee_data_availability_mode: Option<DataAvailabilityMode>,
    pub resource_bounds: Option<ResourceBoundsMapping>,
    #[serde(default, with = "u64_hex_opt")]
    pub tip: Option<u64>,
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub paymaster_data: Option<Vec<Felt>>,
    #[serde_as(deserialize_as = "Option<Vec<UfeHex>>")]
    pub account_deployment_data: Option<Vec<Felt>>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployTransaction {
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<Felt>,
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: Felt,
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub version: Felt,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployAccountTransaction {
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub constructor_calldata: Vec<Felt>,
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub contract_address: Option<Felt>,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: Felt,
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub nonce: Felt,
    #[serde_as(as = "UfeHex")]
    pub version: Felt,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<Felt>,
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub max_fee: Option<Felt>,
    pub nonce_data_availability_mode: Option<DataAvailabilityMode>,
    pub fee_data_availability_mode: Option<DataAvailabilityMode>,
    pub resource_bounds: Option<ResourceBoundsMapping>,
    #[serde(default, with = "u64_hex_opt")]
    pub tip: Option<u64>,
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub paymaster_data: Option<Vec<Felt>>,
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub sender_address: Option<Felt>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct InvokeFunctionTransaction {
    #[serde_as(as = "UfeHex")]
    // Need this alias because older blocks still use `contract_address`
    #[serde(alias = "contract_address")]
    pub sender_address: Felt,
    #[serde_as(as = "Option<UfeHex>")]
    pub entry_point_selector: Option<Felt>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub calldata: Vec<Felt>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub signature: Vec<Felt>,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    #[serde(default)]
    #[serde_as(as = "Option<UfeHex>")]
    pub max_fee: Option<Felt>,
    #[serde_as(as = "Option<UfeHex>")]
    pub nonce: Option<Felt>,
    pub nonce_data_availability_mode: Option<DataAvailabilityMode>,
    pub fee_data_availability_mode: Option<DataAvailabilityMode>,
    pub resource_bounds: Option<ResourceBoundsMapping>,
    #[serde(default, with = "u64_hex_opt")]
    pub tip: Option<u64>,
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub paymaster_data: Option<Vec<Felt>>,
    #[serde_as(deserialize_as = "Option<Vec<UfeHex>>")]
    pub account_deployment_data: Option<Vec<Felt>>,
    #[serde_as(as = "UfeHex")]
    pub version: Felt,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1HandlerTransaction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: Felt,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: Felt,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub calldata: Vec<Felt>,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    #[serde_as(as = "Option<UfeHex>")]
    pub nonce: Option<Felt>,
    #[serde_as(as = "UfeHex")]
    pub version: Felt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ResourceBoundsMapping {
    #[serde(default)]
    pub l1_data_gas: ResourceBounds,
    pub l1_gas: ResourceBounds,
    pub l2_gas: ResourceBounds,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ResourceBounds {
    #[serde(with = "u64_hex")]
    pub max_amount: u64,
    #[serde(with = "u128_hex")]
    pub max_price_per_unit: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum DataAvailabilityMode {
    L1,
    L2,
}

struct DataAvailabilityModeVisitor;

impl TransactionType {
    pub const fn transaction_hash(&self) -> Felt {
        match self {
            Self::Declare(inner) => inner.transaction_hash,
            Self::Deploy(inner) => inner.transaction_hash,
            Self::DeployAccount(inner) => inner.transaction_hash,
            Self::InvokeFunction(inner) => inner.transaction_hash,
            Self::L1Handler(inner) => inner.transaction_hash,
        }
    }
}

impl Serialize for DataAvailabilityMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(match self {
            Self::L1 => 0,
            Self::L2 => 1,
        })
    }
}

impl<'de> Deserialize<'de> for DataAvailabilityMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(DataAvailabilityModeVisitor)
    }
}

impl Visitor<'_> for DataAvailabilityModeVisitor {
    type Value = DataAvailabilityMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(DataAvailabilityMode::L1),
            1 => Ok(DataAvailabilityMode::L2),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(v),
                &"0 or 1",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_full_invoke_transaction() {
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_transaction/1_invoke.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(5));
        if let TransactionType::InvokeFunction(invoke) = tx.r#type.unwrap() {
            assert_eq!(invoke.signature.len(), 2);
        } else {
            panic!("Did not deserialize TransactionType::InvokeFunction properly")
        }
    }

    #[test]
    #[ignore = "transaction with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_full_deploy_transaction() {
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_transaction/2_deploy.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(100));
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
            "../../../test-data/raw_gateway_responses/get_transaction/3_not_received.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, None);
        assert!(tx.status.is_not_received());
    }

    #[test]
    #[ignore = "transaction with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_failure() {
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_transaction/4_failure.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        assert!(tx.transaction_failure_reason.is_some());
        let failure_reason = tx.transaction_failure_reason.unwrap();
        assert_eq!(failure_reason.code, "TRANSACTION_FAILED");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_declare_v1_transaction() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction/5_declare_v1.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::Declare(_) => {}
            _ => panic!("Did not deserialize TransactionType::Declare properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_declare_v2_transaction() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction/6_declare_v2.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::Declare(_) => {}
            _ => panic!("Did not deserialize TransactionType::Declare properly"),
        }
    }

    #[test]
    #[ignore = "transaction with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_reverted() {
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_transaction/7_reverted.txt");
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.execution_status.unwrap() {
            TransactionExecutionStatus::Reverted => {}
            _ => panic!("Unexpected execution status"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_invoke_v3_transaction() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction/8_invoke_v3.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::InvokeFunction(tx) => {
                assert_eq!(tx.version, Felt::THREE);
            }
            _ => panic!("Did not deserialize TransactionType::InvokeFunction properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_declare_v3_transaction() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction/9_declare_v3.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::Declare(tx) => {
                assert_eq!(tx.version, Felt::THREE);
            }
            _ => panic!("Did not deserialize TransactionType::Declare properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_deploy_account_v3_transaction() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction/10_deploy_account_v3.txt"
        );
        let tx: TransactionInfo = serde_json::from_str(raw).unwrap();

        match tx.r#type.unwrap() {
            TransactionType::DeployAccount(tx) => {
                assert_eq!(tx.version, Felt::THREE);
            }
            _ => panic!("Did not deserialize TransactionType::DeployAccount properly"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_accepted() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_status/1_accepted.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert!(tx.status.is_accepted_on_l1());
        assert_eq!(
            tx.block_hash,
            Some(
                Felt::from_hex("0x13b390a0b2c48f907cda28c73a12aa31b96d51bc1be004ba5f71174d8d70e4f")
                    .unwrap()
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_not_received() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_status/2_not_received.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert!(tx.status.is_not_received());
        assert!(tx.block_hash.is_none());
    }

    #[test]
    #[ignore = "transaction with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_failure() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_status/3_failure.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert!(tx.status.is_rejected());
        assert!(tx.block_hash.is_none());
        assert!(tx.transaction_failure_reason.is_some());
    }

    #[test]
    #[ignore = "transaction with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_deser_brief_reverted() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_transaction_status/4_reverted.txt"
        );

        let tx: TransactionStatusInfo = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::Reverted);
        assert!(tx.block_hash.is_some());
        assert!(tx.transaction_failure_reason.is_none());
        assert!(tx.transaction_revert_reason.is_some());
    }
}
