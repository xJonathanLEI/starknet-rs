use starknet::providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, Url,
};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-sepolia.public.blastapi.io/rpc/v0_8").unwrap(),
    ));

    let block_number = provider.block_number().await.unwrap();
    println!("{block_number}");
}
