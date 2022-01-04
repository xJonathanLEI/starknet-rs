use super::{super::serde::deserialize_h256_from_hex, Transaction, TransactionReceipt};

use ethereum_types::H256;
use serde::Deserialize;

pub enum BlockId {
    Hash(H256),
    Number(u64),
}

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub block_hash: H256,
    pub block_number: u64,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub parent_block_hash: H256,
    pub timestamp: u64,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub state_root: H256,
    pub transactions: Vec<Transaction>,
    pub transaction_receipts: Vec<TransactionReceipt>,
}
