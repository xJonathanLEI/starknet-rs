use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use starknet_core::types::{BlockId, BlockTag, MaybePendingBlockWithTxHashes};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient, JsonRpcMethod, MockTransport},
    Provider,
};
use url::Url;

fn mock_transport_with_http() -> (Arc<Mutex<Vec<(String, String)>>>, MockTransport) {
    let rpc_url =
        std::env::var("STARKNET_RPC").unwrap_or("https://rpc-goerli-1.starknet.rs/rpc/v0.4".into());
    let http_transport = HttpTransport::new(Url::parse(&rpc_url).unwrap());
    let req_log = Arc::new(Mutex::new(vec![]));
    (
        req_log.clone(),
        MockTransport::new(Some(http_transport), req_log),
    )
}

#[tokio::test]
async fn mock_transport_fallback() {
    let (_, mock_transport) = mock_transport_with_http();

    let rpc_client = JsonRpcClient::new(mock_transport);

    let block = rpc_client
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    assert!(block.block_number > 0);
}

#[tokio::test]
async fn mock_transport() {
    let (_, mut mock_transport) = mock_transport_with_http();
    // Block number 100000
    mock_transport.mock_request(
        r#"{"id":1,"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":["latest"]}"#.into(),
        r#"{"jsonrpc":"2.0","result":{"block_hash":"0x127edd99c58b5e7405c3fa24920abbf4c3fcfcd532a1c9f496afb917363c386","block_number":100000,"new_root":"0x562df6c11a47b6711242d00318fec36c9f0f2613f7b711cd732857675b4f7f5","parent_hash":"0x294f21cc482c8329b7e1f745cff69071685aec7955de7f5f9dae2be3cc27446","sequencer_address":"0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8","status":"ACCEPTED_ON_L2","timestamp":1701037710,"transactions":["0x1"]},"id":1}"#.into()
    );

    let rpc_client = JsonRpcClient::new(mock_transport);

    let block = rpc_client
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    assert!(block.block_number == 100000);
}

#[tokio::test]
async fn mock_transport_method() {
    let (_, mut mock_transport) = mock_transport_with_http();
    // Block number 100000
    mock_transport.mock_method(
        JsonRpcMethod::GetBlockWithTxHashes,
        r#"{"jsonrpc":"2.0","result":{"block_hash":"0x127edd99c58b5e7405c3fa24920abbf4c3fcfcd532a1c9f496afb917363c386","block_number":100000,"new_root":"0x562df6c11a47b6711242d00318fec36c9f0f2613f7b711cd732857675b4f7f5","parent_hash":"0x294f21cc482c8329b7e1f745cff69071685aec7955de7f5f9dae2be3cc27446","sequencer_address":"0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8","status":"ACCEPTED_ON_L2","timestamp":1701037710,"transactions":["0x1"]},"id":1}"#.into()
      );

    let rpc_client = JsonRpcClient::new(mock_transport);

    let block = rpc_client
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    assert!(block.block_number == 100000);
}

#[tokio::test]
async fn mock_transport_log() {
    let (logs, mut mock_transport) = mock_transport_with_http();

    mock_transport.mock_request(
        r#"{"id":1,"jsonrpc":"2.0","method":"starknet_getBlockWithTxHashes","params":["latest"]}"#.into(), 
        r#"{"jsonrpc":"2.0","result":{"block_hash":"0x42fd8152ab51f0d5937ca83225035865c0dcdaea85ab84d38243ec5df23edac","block_number":100000,"new_root":"0x372c133dace5d2842e3791741b6c05af840f249b52febb18f483d1eb38aaf8a","parent_hash":"0x7f6df65f94584de3ff9807c67822197692cc8895aa1de5340af0072ac2ccfb5","sequencer_address":"0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8","status":"ACCEPTED_ON_L2","timestamp":1701033987,"transactions":["0x1"]},"id":1}"#.into()
    );

    let rpc_client = JsonRpcClient::new(mock_transport);

    let block = rpc_client
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let block = match block {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    let logs = logs.lock().unwrap();

    assert!(block.block_number > 0);

    assert!(logs.len() == 1);
    // Check request contains getBlockWithTxHashes
    assert!(logs[0].0.contains("starknet_getBlockWithTxHashes") == true);
    // Check response result has block_hash
    assert!(logs[0].1.contains("block_hash") == true);
}
