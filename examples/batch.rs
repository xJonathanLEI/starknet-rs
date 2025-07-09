use starknet::providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderRequestData, ProviderResponseData, Url,
};
use starknet_core::types::{
    requests::{BlockNumberRequest, GetBlockTransactionCountRequest},
    BlockId,
};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_9").unwrap(),
    ));

    let responses = provider
        .batch_requests([
            ProviderRequestData::BlockNumber(BlockNumberRequest),
            ProviderRequestData::GetBlockTransactionCount(GetBlockTransactionCountRequest {
                block_id: BlockId::Number(100),
            }),
        ])
        .await
        .unwrap();

    match (&responses[0], &responses[1]) {
        (
            ProviderResponseData::BlockNumber(block_number),
            ProviderResponseData::GetBlockTransactionCount(count),
        ) => {
            println!("The latest block is #{block_number}");
            println!("Block #100 has {count} transactions");
        }
        _ => panic!("unexpected response type"),
    }
}
