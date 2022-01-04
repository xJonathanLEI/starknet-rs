use super::{super::serde::deserialize_h256_from_hex, Transaction, TransactionReceipt};

use ethereum_types::H256;
use serde::Deserialize;

pub enum BlockId {
    Hash(H256),
    Number(u64),
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    block_hash: H256,
    block_number: u64,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    parent_block_hash: H256,
    timestamp: u64,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    state_root: H256,
    transactions: Vec<Transaction>,
    transaction_receipts: Vec<TransactionReceipt>,
}
