use super::{
    super::serde::unsigned_field_element::{UfeHex, UfeHexOption},
    ConfirmedTransactionReceipt, FieldElement, TransactionType,
};

use serde::Deserialize;
use serde_with::serde_as;

#[derive(Debug, Clone, Copy)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Pending,
    Latest,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum BlockStatus {
    /// Block that is yet to be closed
    Pending,
    /// Block failed in the L2 pipeline
    Aborted,
    /// A reverted block (rejected on L1)
    Reverted,
    /// Block that was created on L2, in contrast to Pending, which is not yet closed
    AcceptedOnL2,
    /// Accepted on L1
    AcceptedOnL1,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Block {
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
    pub block_hash: Option<FieldElement>,
    pub block_number: Option<u64>,
    #[serde_as(as = "UfeHex")]
    pub parent_block_hash: FieldElement,
    pub timestamp: u64,
    // Field marked optional as old blocks don't include it yet. Drop optional once resolved.
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
    pub sequencer_address: Option<FieldElement>,
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
    pub state_root: Option<FieldElement>,
    pub status: BlockStatus,
    #[serde_as(as = "UfeHex")]
    pub gas_price: FieldElement,
    pub transactions: Vec<TransactionType>,
    pub transaction_receipts: Vec<ConfirmedTransactionReceipt>,
    // Field marked optional as old blocks don't include it yet. Drop optional once resolved.
    pub starknet_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_transactions() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/1_with_transactions.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 39232);
        assert_eq!(block.status, BlockStatus::AcceptedOnL1);
        assert_eq!(
            block.state_root.unwrap(),
            FieldElement::from_hex_be(
                "06cb132715b8687f1c1d79a7282975986fb0a9c166d64b384cfad965a602fe02"
            )
            .unwrap()
        );
        assert_eq!(block.transactions.len(), 3);
        assert_eq!(block.transaction_receipts.len(), 3);

        if let TransactionType::Deploy(tx) = &block.transactions[0] {
            assert_eq!(tx.constructor_calldata.len(), 2);
        } else {
            panic!("Did not deserialize Transaction::Deploy properly");
        }
        if let TransactionType::InvokeFunction(tx) = &block.transactions[1] {
            assert_eq!(tx.calldata.len(), 7);
        } else {
            panic!("Did not deserialize Transaction::InvokeFunction properly");
        }
        let receipt = &block.transaction_receipts[0];
        assert_eq!(receipt.execution_resources.as_ref().unwrap().n_steps, 68);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_messages() {
        // has an L2 to L1 message
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/2_with_messages.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 122387);
        assert_eq!(block.transaction_receipts.len(), 49);
        let receipt = &block.transaction_receipts[22];
        assert_eq!(receipt.l2_to_l1_messages.len(), 1);
        assert_eq!(receipt.l2_to_l1_messages[0].payload.len(), 2);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_messages_without_nonce() {
        // has an L2 to L1 message
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_block/9_with_messages_without_nonce.txt"
        );

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 1564);
        assert_eq!(block.transaction_receipts.len(), 4);
        let receipt = &block.transaction_receipts[1];
        assert_eq!(receipt.l2_to_l1_messages.len(), 1);
        assert_eq!(receipt.l2_to_l1_messages[0].payload.len(), 2);

        let receipt = &block.transaction_receipts[2];
        assert!(receipt.l1_to_l2_consumed_message.is_some());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_events() {
        // has events introduced with Starknet v0.7.0
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/3_with_events.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert_eq!(block.block_number.unwrap(), 47543);
        assert_eq!(block.transaction_receipts.len(), 4);
        let receipt = &block.transaction_receipts[3];
        assert_eq!(receipt.events.len(), 1);
        assert_eq!(receipt.events[0].data.len(), 2);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_pending() {
        // pending blocks don't have `block_hash`, `block_number`, or `state_root`
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/4_pending.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert!(block.block_hash.is_none());
        assert!(block.block_number.is_none());
        assert!(block.state_root.is_none());
        assert_eq!(block.status, BlockStatus::Pending);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_new_attributes_0_8_2() {
        // This block contains new fields introduced in Starknet v0.8.2
        let new_block: Block = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_block/6_with_sequencer_address.txt"
        ))
        .unwrap();
        assert!(new_block.sequencer_address.is_some());

        let old_block: Block = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_block/2_with_messages.txt"
        ))
        .unwrap();
        assert!(old_block.sequencer_address.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_new_attributes_0_9_1() {
        // This block contains new fields introduced in Starknet v0.9.1
        let new_block: Block = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_block/8_with_starknet_version.txt"
        ))
        .unwrap();
        assert!(new_block.starknet_version.is_some());

        let old_block: Block = serde_json::from_str(include_str!(
            "../../test-data/raw_gateway_responses/get_block/2_with_messages.txt"
        ))
        .unwrap();
        assert!(old_block.starknet_version.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_declare_tx() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/7_with_declare_tx.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        let tx = match &block.transactions[26] {
            TransactionType::Declare(tx) => tx,
            _ => panic!("Unexpected tx type"),
        };

        assert_eq!(tx.sender_address, FieldElement::ONE);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_l1_handler() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/10_with_l1_handler.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        let tx = match &block.transactions[23] {
            TransactionType::L1Handler(tx) => tx,
            _ => panic!("Unexpected tx type"),
        };

        assert_eq!(
            tx.contract_address,
            FieldElement::from_hex_be(
                "0x4a472fe795cc40e9dc838fe4f1608cb91bf027854d016675ec81e172a2e3599"
            )
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_without_execution_resources() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_block/11_without_execution_resources.txt"
        );

        let block: Block = serde_json::from_str(raw).unwrap();

        let receipt = &block.transaction_receipts[17];

        assert!(receipt.execution_resources.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_l1_handler_without_nonce() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_block/12_l1_handler_without_nonce.txt"
        );

        let block: Block = serde_json::from_str(raw).unwrap();

        let tx = match &block.transactions[22] {
            TransactionType::L1Handler(tx) => tx,
            _ => panic!("Unexpected tx type"),
        };

        assert!(tx.nonce.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_without_entry_point() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_block/13_without_entry_point.txt"
        );

        let block: Block = serde_json::from_str(raw).unwrap();

        let tx = match &block.transactions[16] {
            TransactionType::InvokeFunction(tx) => tx,
            _ => panic!("Unexpected tx type"),
        };

        assert!(tx.entry_point_selector.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_deploy_account() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_block/14_deploy_account.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        let tx = match &block.transactions[43] {
            TransactionType::DeployAccount(tx) => tx,
            _ => panic!("Unexpected tx type"),
        };

        assert_eq!(tx.signature.len(), 2);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_block_deser_with_declare_v2() {
        let raw = include_str!("../../test-data/raw_gateway_responses/get_block/15_declare_v2.txt");

        let block: Block = serde_json::from_str(raw).unwrap();

        assert!(block
            .transactions
            .into_iter()
            .any(|tx| matches!(tx, TransactionType::Declare(_))));
    }
}
