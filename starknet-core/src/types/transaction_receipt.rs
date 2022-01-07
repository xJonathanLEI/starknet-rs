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
    pub status: TransactionStatusType,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub block_hash: H256,
    pub block_number: u64,
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

#[derive(Debug, Deserialize)]
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
