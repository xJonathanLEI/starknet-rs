use super::super::serde::{deserialize_h256_from_hex, deserialize_vec_u256_from_dec};

use ethereum_types::{Address as L1Address, H256, U256};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {
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
#[serde(tag = "tx_status")]
pub enum TransactionStatus {
    #[serde(rename = "NOT_RECEIVED")]
    NotReceived,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2(TransactionBlockHash),
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1(TransactionBlockHash),
}

#[derive(Debug, Deserialize)]
pub enum TransactionStatusType {
    #[serde(rename = "NOT_RECEIVED")]
    NotReceived,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
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
