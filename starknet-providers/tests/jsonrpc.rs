use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.cartridge.gg/").unwrap(),
    ))
}

#[tokio::test]
async fn jsonrpc_block_number() {
    let rpc_client = create_jsonrpc_client();

    let block_number = rpc_client.block_number().await.unwrap();
    assert!(block_number > 0);
}
