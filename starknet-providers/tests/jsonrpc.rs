use starknet_core::{
    types::FieldElement,
    utils::{get_selector_from_name, get_storage_var_address},
};
use starknet_providers::jsonrpc::{
    models::{BlockHashOrTag, BlockNumOrTag, BlockTag, FunctionCall, SyncStatusType},
    HttpTransport, JsonRpcClient,
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
