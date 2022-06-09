use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};

use crate::jsonrpc::models::*;

mod transports;
pub use transports::{HttpTransport, JsonRpcTransport};

/// Temporary module for holding JSON-RPC data models until the provider switch:
///
/// https://github.com/xJonathanLEI/starknet-rs/issues/77#issuecomment-1150184364
pub mod models;

#[derive(Debug)]
pub struct JsonRpcClient<T> {
    transport: T,
}

#[derive(Debug, Serialize)]
pub enum JsonRpcMethod {
    #[serde(rename = "starknet_blockNumber")]
    BlockNumber,
    #[serde(rename = "starknet_call")]
    Call,
}

#[derive(Debug, thiserror::Error)]
pub enum JsonRpcClientError<T> {
    #[error(transparent)]
    JsonError(serde_json::Error),
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

#[serde_as]
#[derive(Deserialize)]
struct FeltArray(#[serde_as(as = "Vec<UfeHex>")] pub Vec<FieldElement>);

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
        self.send_request(JsonRpcMethod::BlockNumber, ()).await
    }

    /// Call a starknet function without creating a StarkNet transaction
    pub async fn call(
        &self,
        request: &FunctionCall,
        block_hash: &BlockHashOrTag,
    ) -> Result<Vec<FieldElement>, JsonRpcClientError<T::Error>> {
        Ok(self
            .send_request::<_, FeltArray>(
                JsonRpcMethod::Call,
                [
                    serde_json::to_value(request)?,
                    serde_json::to_value(block_hash)?,
                ],
            )
            .await?
            .0)
    }

    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<R, JsonRpcClientError<T::Error>>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        match self
            .transport
            .send_request(method, params)
            .await
            .map_err(JsonRpcClientError::TransportError)?
        {
            JsonRpcResponse::Success { result, .. } => Ok(result),
            JsonRpcResponse::Error { error, .. } => Err(JsonRpcClientError::RpcError(error)),
        }
    }
}

impl<T> From<serde_json::Error> for JsonRpcClientError<T> {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
