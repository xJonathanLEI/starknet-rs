use super::{
    super::serde::{deserialize_h256_from_hex, deserialize_option_h256_from_hex},
    ConfirmedTransactionReceipt, Transaction,
};

use ethereum_types::H256;
use serde::Deserialize;

pub enum BlockId {
    Hash(H256),
    Number(u64),
    Pending,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_h256_from_hex")]
    pub block_hash: Option<H256>,
    pub block_number: Option<u64>,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub parent_block_hash: H256,
    pub timestamp: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_h256_from_hex")]
    pub state_root: Option<H256>,
    pub transactions: Vec<Transaction>,
    pub transaction_receipts: Vec<ConfirmedTransactionReceipt>,
}

#[cfg(test)]
mod tests {
    use super::super::transaction::EntryPointType;
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_block_deser_with_deploy_tx() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/1_with_deploy_tx.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 39232);
        assert_eq!(
            block.state_root.unwrap(),
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
        assert_eq!(receipt.execution_resources.n_steps, 68);
    }

    #[test]
    fn test_block_deser_with_messages() {
        // has an L2 to L1 message
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/2_with_messages.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 39227);
        assert_eq!(block.transaction_receipts.len(), 4);
        let receipt = &block.transaction_receipts[0];
        assert_eq!(receipt.l2_to_l1_messages.len(), 1);
        assert_eq!(receipt.l2_to_l1_messages[0].payload.len(), 5);
    }

    #[test]
    fn test_block_deser_with_events() {
        // has an L2 to L1 message
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/3_with_events.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 47543);
        assert_eq!(block.transaction_receipts.len(), 4);
        let receipt = &block.transaction_receipts[3];
        assert_eq!(receipt.events.len(), 1);
        assert_eq!(receipt.events[0].data.len(), 2);
    }

    #[test]
    fn test_block_deser_pending() {
        // has an L2 to L1 message
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/4_pending.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert!(block.block_hash.is_none());
        assert!(block.block_number.is_none());
        assert!(block.state_root.is_none());
    }
}
