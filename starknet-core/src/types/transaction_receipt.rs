use super::super::serde::{deserialize_h256_from_hex, deserialize_vec_u256_from_dec};

use ethereum_types::{Address as L1Address, H256, U256};
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    transaction_hash: H256,
    status: TransactionStatus,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    block_hash: H256,
    block_number: u64,
    transaction_index: u64,
    execution_resources: ExecutionResources,
    l2_to_l1_messages: Vec<L2ToL1Message>,
}

#[derive(Debug, Deserialize)]
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
    AcceptedOnL2,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ExecutionResources {
    n_steps: u64,
    n_memory_holes: u64,
    builtin_instance_counter: BuiltinInstanceCounter,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct BuiltinInstanceCounter {
    pedersen_builtin: u64,
    range_check_builtin: u64,
    bitwise_builtin: u64,
    output_builtin: u64,
    ecdsa_builtin: u64,
    ec_op_builtin: u64,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct L2ToL1Message {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    from_address: H256,
    to_address: L1Address,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    payload: Vec<U256>,
}
