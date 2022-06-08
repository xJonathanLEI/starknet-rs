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

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError<T> {
    #[error(transparent)]
    TransportError(T),
    #[error(transparent)]
    RpcError(JsonRpcError),
}

#[derive(Debug, thiserror::Error, Deserialize)]
#[error("JSON-RPC error: code={code}, message=\"{message}\"")]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse<T> {
    Success { id: u64, result: T },
    Error { id: u64, error: JsonRpcError },
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
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
    pub async fn block_number(&self) -> Result<u64, JsonRpcClientError<T::Error>> {
        match self
            .transport
            .send_request(JsonRpcMethod::BlockNumber, ())
            .await
            .map_err(JsonRpcClientError::TransportError)?
        {
            JsonRpcResponse::Success { result, .. } => Ok(result),
            JsonRpcResponse::Error { error, .. } => Err(JsonRpcClientError::RpcError(error)),
        }
    }
}
