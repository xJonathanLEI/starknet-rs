use starknet_core::{
    types::FieldElement,
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::jsonrpc::{
    models::{BlockHashOrTag, BlockNumOrTag, BlockTag, EventFilter, FunctionCall, SyncStatusType},
    HttpTransport, JsonRpcClient, JsonRpcClientError,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.cartridge.gg/").unwrap(),
    ))
}

#[tokio::test]
async fn jsonrpc_get_block_by_hash() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_hash(&BlockHashOrTag::Tag(BlockTag::Latest))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_by_hash_with_txns() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_hash_with_txns(&BlockHashOrTag::Tag(BlockTag::Latest))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_by_hash_with_receipts() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_hash_with_receipts(&BlockHashOrTag::Tag(BlockTag::Latest))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_by_number() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_number(&BlockNumOrTag::Number(234469))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_by_number_with_txns() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_number_with_txns(&BlockNumOrTag::Number(234469))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_block_by_number_with_receipts() {
    let rpc_client = create_jsonrpc_client();

    let block = rpc_client
        .get_block_by_number_with_receipts(&BlockNumOrTag::Number(234469))
        .await
        .unwrap();
    assert!(block.metadata.block_number > 0);
}

#[tokio::test]
async fn jsonrpc_get_storage_at() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance via storage taking advantage of implementation detail
    let eth_balance = rpc_client
        .get_storage_at(
            FieldElement::from_hex_be(
                "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            get_storage_var_address(
                "ERC20_balances",
                &[FieldElement::from_hex_be(
                    "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
                )
                .unwrap()],
            )
            .unwrap(),
            &BlockHashOrTag::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_hash(
            FieldElement::from_hex_be(
                "05b08d06a7f6422881d6461175f325844d179ca9018dbab5e92dc34e5c176ff1",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert!(tx.entry_point_selector.is_some());
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_hash_non_existent_tx() {
    let rpc_client = create_jsonrpc_client();

    let err = rpc_client
        .get_transaction_by_hash(FieldElement::from_hex_be("1234").unwrap())
        .await
        .unwrap_err();

    match err {
        JsonRpcClientError::RpcError(err) => {
            // INVALID_TXN_HASH
            assert_eq!(err.code, 25);
        }
        _ => panic!("Unexpected error"),
    }
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_block_hash_and_index() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_block_hash_and_index(
            &BlockHashOrTag::Hash(
                FieldElement::from_hex_be(
                    "04d893935543cc0a39d1ce1597695e0fc02f9512781e0b23f41bbb01b0c6b5f1",
                )
                .unwrap(),
            ),
            0,
        )
        .await
        .unwrap();

    assert!(tx.entry_point_selector.is_some());
}

#[tokio::test]
async fn jsonrpc_get_transaction_by_block_number_and_index() {
    let rpc_client = create_jsonrpc_client();

    let tx = rpc_client
        .get_transaction_by_block_number_and_index(&BlockNumOrTag::Number(234500), 0)
        .await
        .unwrap();

    assert!(tx.entry_point_selector.is_some());
}

#[tokio::test]
async fn jsonrpc_get_transaction_receipt() {
    let rpc_client = create_jsonrpc_client();

    let receipt = rpc_client
        .get_transaction_receipt(
            FieldElement::from_hex_be(
                "05b08d06a7f6422881d6461175f325844d179ca9018dbab5e92dc34e5c176ff1",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert!(receipt.actual_fee > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_get_block_transaction_count_by_hash() {
    let rpc_client = create_jsonrpc_client();

    let tx_count = rpc_client
        .get_block_transaction_count_by_hash(&BlockHashOrTag::Hash(
            FieldElement::from_hex_be(
                "0ef4773e814cf100e0535fe5ddffcb8d1d966fc81a9cdf9ca94b2672e130334",
            )
            .unwrap(),
        ))
        .await
        .unwrap();

    assert_eq!(tx_count, 45);
}

#[tokio::test]
async fn jsonrpc_get_block_transaction_count_by_number() {
    let rpc_client = create_jsonrpc_client();

    let tx_count = rpc_client
        .get_block_transaction_count_by_number(&BlockNumOrTag::Number(234519))
        .await
        .unwrap();

    assert_eq!(tx_count, 45);
}

#[tokio::test]
async fn jsonrpc_block_number() {
    let rpc_client = create_jsonrpc_client();

    let block_number = rpc_client.block_number().await.unwrap();
    assert!(block_number > 0);
}

#[tokio::test]
async fn jsonrpc_chain_id() {
    let rpc_client = create_jsonrpc_client();

    let chain_id = rpc_client.chain_id().await.unwrap();
    assert!(chain_id > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_syncing() {
    let rpc_client = create_jsonrpc_client();

    let syncing = rpc_client.syncing().await.unwrap();
    if let SyncStatusType::Syncing(sync_status) = syncing {
        assert!(sync_status.highest_block_num > 0);
    }
}

#[tokio::test]
async fn jsonrpc_get_events() {
    let rpc_client = create_jsonrpc_client();

    let events = rpc_client
        .get_events(
            EventFilter {
                from_block: Some(234500),
                to_block: None,
                address: None,
                keys: None,
            },
            20,
            10,
        )
        .await
        .unwrap();

    assert_eq!(events.events.len(), 20);
}

#[tokio::test]
async fn jsonrpc_call() {
    let rpc_client = create_jsonrpc_client();

    // Checks L2 ETH balance
    let eth_balance = rpc_client
        .call(
            &FunctionCall {
                contract_address: FieldElement::from_hex_be(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
                calldata: vec![FieldElement::from_hex_be(
                    "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
                )
                .unwrap()],
            },
            &BlockHashOrTag::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert!(eth_balance[0] > FieldElement::ZERO);
}

#[tokio::test]
async fn jsonrpc_add_invoke_transaction() {
    let rpc_client = create_jsonrpc_client();

    // This is an invalid made-up transaction but the sequencer will happily accept it anyways
    let add_tx_result = rpc_client
        .add_invoke_transaction(
            &FunctionCall {
                contract_address: FieldElement::from_hex_be(
                    "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                entry_point_selector: get_selector_from_name("__execute__").unwrap(),
                calldata: vec![FieldElement::from_hex_be("1234").unwrap()],
            },
            vec![],
            FieldElement::ONE,
            FieldElement::ZERO,
        )
        .await
        .unwrap();

    assert!(add_tx_result.transaction_hash > FieldElement::ZERO);
}
