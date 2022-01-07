use super::{super::serde::deserialize_h256_from_hex, ConfirmedTransactionReceipt, Transaction};

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
    pub transaction_receipts: Vec<ConfirmedTransactionReceipt>,
}

#[cfg(test)]
mod tests {
    use super::super::transaction::EntryPointType;
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_block_deser() {
        // has a deploy TX
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/1_latest.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number, 39232);
        assert_eq!(
            block.state_root,
            H256::from_str("06cb132715b8687f1c1d79a7282975986fb0a9c166d64b384cfad965a602fe02")
                .unwrap()
        );
        assert_eq!(block.transactions.len(), 3);
        assert_eq!(block.transaction_receipts.len(), 3);
        if let Transaction::Deploy(tx) = &block.transactions[0] {
            assert_eq!(tx.constructor_calldata.len(), 2)
        } else {
            panic!("Did not deserialize Transaction::Deploy properly");
        }
        if let Transaction::InvokeFunction(tx) = &block.transactions[1] {
            assert_eq!(tx.entry_point_type, EntryPointType::External);
            assert_eq!(tx.calldata.len(), 7);
        } else {
            panic!("Did not deserialize Transaction::InvokeFunction properly");
        }
        let receipt = &block.transaction_receipts[0];
        assert_eq!(receipt.block_number, 39232);
        assert_eq!(receipt.execution_resources.n_steps, 68);

        // has an L2 to L1 message
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/2_with_messages.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number, 39227);
        assert_eq!(block.transaction_receipts.len(), 4);
        let receipt = &block.transaction_receipts[0];
        assert_eq!(receipt.l2_to_l1_messages.len(), 1);
        assert_eq!(receipt.l2_to_l1_messages[0].payload.len(), 5);
    }
}
