use serde::Deserialize;
use starknet_core::types::FieldElement;
use starknet_providers::jsonrpc::models::{BlockId, BlockTag};

#[test]
fn test_serialize_deserialize_block_id() {
    let block_ids = [
        BlockId::Hash(FieldElement::ONE),
        BlockId::Number(100),
        BlockId::Tag(BlockTag::Latest),
        BlockId::Tag(BlockTag::Pending),
    ];

    for block_id in block_ids {
        let serialized = serde_json::to_string(&block_id).unwrap();
        println!("{block_id:?}: {serialized}");
        let deserialized: BlockId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, block_id);
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Test {
    x: u32,
    y: bool,
}

#[test]
fn test_deserialize_invalid_block_id() {
    let invalid_block_ids = [
        "\"invalid\"",
        "{\"block_number\"}",
        "{\"invalid\"}",
        "{\"invalid\": 1}",
        "{\"block_number\": 1, \"invalid\": 2}",
        "[1]",
    ];
    for invalid_block_id in invalid_block_ids {
        let result: Result<BlockId, serde_json::Error> = serde_json::from_str(invalid_block_id);
        assert!(result.is_err());
    }
}
