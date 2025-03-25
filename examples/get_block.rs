use starknet::{
    core::types::{BlockId, BlockTag},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_8").unwrap(),
    ));

    let latest_block = provider
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await;
    println!("{latest_block:#?}");
}
