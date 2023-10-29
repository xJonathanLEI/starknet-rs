use starknet_providers::{
    jsonrpc::{HttpTransport, HttpTransportError, JsonRpcClient, JsonRpcClientError},
    Provider, ProviderError,
};
use url::Url;

#[tokio::main]
async fn main() {
    let rpc_client =
        JsonRpcClient::new(HttpTransport::new(Url::parse("https://fake.url/").unwrap()));

    let error = match rpc_client.block_number().await.unwrap_err() {
        ProviderError::Other(inner) => inner,
        _ => panic!("unexpected error variant"),
    };

    // The implementation-specific error type is erased to make the `ProviderError` type easier to
    // work with. Here, we showcase how to recover the inner type.
    let impl_specific_error: &JsonRpcClientError<HttpTransportError> =
        match error.as_any().downcast_ref() {
            Some(inner) => inner,
            None => panic!("unexpected downcast failure"),
        };

    dbg!(impl_specific_error);
}
