use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;

#[tokio::main]
async fn main() {
    let rpc_client = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.cartridge.gg/").unwrap(),
    ));

    let block_number = rpc_client.block_number().await.unwrap();
    println!("{}", block_number);
}
