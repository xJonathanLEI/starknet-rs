use starknet::{
    core::types::{BlockId, BlockTag},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};
use starknet_core::types::requests::{GetBlockWithTxHashesRequestRef, GetBlockWithTxsRequestRef};
use starknet_providers::jsonrpc::{JsonRpcMethod, JsonRpcRequestParams};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));
    
    let batch_mixed_results = provider.batch_requests(vec![(JsonRpcMethod::GetBlockWithTxHashes, JsonRpcRequestParams::GetBlockWithTxHashes(GetBlockWithTxHashesRequestRef {
        block_id: BlockId::Tag(BlockTag::Latest).as_ref(),
    })), (JsonRpcMethod::GetBlockWithTxs, JsonRpcRequestParams::GetBlockWithTxs(GetBlockWithTxsRequestRef {
        block_id: BlockId::Tag(BlockTag::Latest).as_ref(),
    }))]).await;

    match batch_mixed_results {
        Ok(v) => println!("{v:#?}"),
        Err(e) => println!("Error: {e:#?}"),
    }

    let batched_blocks = provider.get_block_with_tx_hashes_batch(vec![BlockId::Tag(BlockTag::Latest), BlockId::Tag(BlockTag::Latest)]).await;

    match batched_blocks {
        Ok(v) => println!("{v:#?}"),
        Err(e) => println!("Error: {e:#?}"),
    }

}
