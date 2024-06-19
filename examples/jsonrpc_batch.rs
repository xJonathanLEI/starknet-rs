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
    // Create a new JSON RPC client using HTTP transport with the specified URL
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_7").unwrap(),
    ));

    // batch_requests allows to define a vector of requests for batch processing, ensuring each request specifies its corresponding JsonRpcMethod and JsonRpcRequestParams.
    // This approach allows for a generic way to handle batch requests.
    let batch_mixed_results = provider
        .batch_requests(vec![
            // Request 1: Retrieve block data including transaction hashes.
            (
                JsonRpcMethod::GetBlockWithTxHashes,
                JsonRpcRequestParams::GetBlockWithTxHashes(GetBlockWithTxHashesRequestRef {
                    block_id: BlockId::Tag(BlockTag::Latest).as_ref(),
                }),
            ),
            // Request 2: Retrieve block data including full transaction details.
            (
                JsonRpcMethod::GetBlockWithTxs,
                JsonRpcRequestParams::GetBlockWithTxs(GetBlockWithTxsRequestRef {
                    block_id: BlockId::Tag(BlockTag::Latest).as_ref(),
                }),
            ),
        ])
        .await;

    match batch_mixed_results {
        Ok(v) => println!("{v:#?}"),
        Err(e) => println!("Error: {e:#?}"),
    }

    // The following example demonstrates the process of sending a batch request to retrieve multiple blocks, each including transaction hashes.
    // get_block_with_tx_hashes_batch utilizes a vector of BlockId parameters to construct the batch request.
    let batched_blocks = provider
        .get_block_with_tx_hashes_batch(vec![
            BlockId::Tag(BlockTag::Latest),
            BlockId::Tag(BlockTag::Latest),
        ])
        .await;

    match batched_blocks {
        Ok(v) => println!("{v:#?}"),
        Err(e) => println!("Error: {e:#?}"),
    }
}
