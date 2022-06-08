use serde::{Deserialize, Serialize};

mod transports;
pub use transports::{HttpTransport, JsonRpcTransport};

#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

#[derive(Debug, Serialize)]
pub enum JsonRpcMethod {
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    #[allow(unused)]
    id: u64,
    #[allow(unused)]
    jsonrpc: String,
    result: T,
}

impl<T> JsonRpcClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> JsonRpcClient<T>
where
    T: JsonRpcTransport,
{
    /// Get the most recent accepted block number
    pub async fn block_number(&self) -> Result<u64, T::Error> {
        self.transport
            .send_request(JsonRpcMethod::BlockNumber, ())
            .await
    }
}
