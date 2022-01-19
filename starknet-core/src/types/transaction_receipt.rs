use super::super::serde::{
    deserialize_h256_from_hex, deserialize_pending_block_hash, deserialize_vec_u256_from_dec,
};

use ethereum_types::{Address as L1Address, H256, U256};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Receipt {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub transaction_hash: H256,
    pub status: TransactionStatusType,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_pending_block_hash")]
    pub block_hash: Option<H256>,
    pub block_number: Option<u64>,
    pub transaction_index: Option<u64>,
    pub execution_resources: Option<ExecutionResources>,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmedReceipt {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub execution_resources: ExecutionResources,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "tx_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    NotReceived,
    Received,
    Pending,
    Rejected,
    AcceptedOnL2(TransactionBlockHash),
    AcceptedOnL1(TransactionBlockHash),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatusType {
    NotReceived,
    Received,
    Pending,
    Rejected,
    AcceptedOnL2,
    AcceptedOnL1,
}

#[derive(Debug, Deserialize)]
pub struct TransactionBlockHash {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub block_hash: H256,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionResources {
    pub n_steps: u64,
    pub n_memory_holes: u64,
    pub builtin_instance_counter: BuiltinInstanceCounter,
}

#[derive(Debug, Deserialize)]
pub struct BuiltinInstanceCounter {
    pub pedersen_builtin: u64,
    pub range_check_builtin: u64,
    pub bitwise_builtin: u64,
    pub output_builtin: u64,
    pub ecdsa_builtin: u64,
    pub ec_op_builtin: u64,
}

#[derive(Debug, Deserialize)]
pub struct L2ToL1Message {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub from_address: H256,
    pub to_address: L1Address,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    pub payload: Vec<U256>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_receipt_deser_accepted() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_receipt/1_accepted.txt"
        );

        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.status, TransactionStatusType::AcceptedOnL1);
        assert_eq!(receipt.block_number, Some(39207));
        assert_eq!(receipt.execution_resources.unwrap().n_steps, 489);
    }

    #[test]
    fn test_receipt_deser_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_receipt/2_not_received.txt"
        );
        let receipt: Receipt = serde_json::from_str(raw).unwrap();

        assert_eq!(receipt.status, TransactionStatusType::NotReceived);
        assert_eq!(
            receipt.transaction_hash,
            H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
        );
        assert_eq!(receipt.block_hash, None);
    }

    #[test]
    fn test_transaction_status_deser_accepted_on_l2() {
        // note that the hashes coming from the API can be shorter
        // by a byte or two than the H256 into which we serialize into,
        // that's why there's extra 0 in the H256::from_str values

        // curl -X GET https://alpha4.starknet.io/feeder_gateway/get_transaction_status\?transactionHash\=0x5d76420c7e7002c20d54c93fc8dbd056638f1a35a654748fc0647fda1a3f088
        let raw = r#"{
            "tx_status": "ACCEPTED_ON_L2",
            "block_hash": "0x7b44bda3371fa91541e719493b1638b71c7ccf2304dc67bbadb028dbfa16dec"
        }"#;

        let tx: TransactionStatus = serde_json::from_str(raw).unwrap();
        if let TransactionStatus::AcceptedOnL2(b) = tx {
            assert_eq!(
                b.block_hash,
                H256::from_str(
                    "0x07b44bda3371fa91541e719493b1638b71c7ccf2304dc67bbadb028dbfa16dec",
                )
                .unwrap()
            );
        } else {
            panic!("Did not deserialize TransactionStatus::AcceptedOnL2 properly");
        }
    }

    #[test]
    fn test_transaction_status_deser_accepted_on_l1() {
        // curl -X GET https://alpha4.starknet.io/feeder_gateway/get_transaction_status\?transactionHash\=0x10f2462bd8d90ad7242f16c5432f5ca6a53d2846592c6170242e032a5f836a
        let raw = r#"{
            "tx_status": "ACCEPTED_ON_L1",
            "block_hash": "0x5da543f8121c912cd2a80ae386f1aa6d4df626695742cf870c85690bb1ab60"
        }"#;

        let tx: TransactionStatus = serde_json::from_str(raw).unwrap();
        if let TransactionStatus::AcceptedOnL1(b) = tx {
            assert_eq!(
                b.block_hash,
                H256::from_str(
                    "0x005da543f8121c912cd2a80ae386f1aa6d4df626695742cf870c85690bb1ab60"
                )
                .unwrap()
            )
        } else {
            panic!("Did not deserialize TransactionStatus::AcceptedOnL1 properly");
        }
    }
}
