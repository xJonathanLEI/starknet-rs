use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};
use url::Url;

#[tokio::main]
async fn main() {
    let rpc_client = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")
            .unwrap(),
    ));

    let block_number = rpc_client.block_number().await.unwrap();
    println!("{block_number}");
}
