use super::{super::serde::deserialize_h256, Transaction, TransactionReceipt};

use ethereum_types::H256;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(deserialize_with = "deserialize_h256")]
    block_hash: H256,
    block_number: u64,
    #[serde(deserialize_with = "deserialize_h256")]
    parent_block_hash: H256,
    timestamp: u64,
    #[serde(deserialize_with = "deserialize_h256")]
    state_root: H256,
    transactions: Vec<Transaction>,
    transaction_receipts: Vec<TransactionReceipt>,
}
