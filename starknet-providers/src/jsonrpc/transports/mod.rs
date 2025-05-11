use async_trait::async_trait;
use auto_impl::auto_impl;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

use crate::{
    jsonrpc::{JsonRpcMethod, JsonRpcResponse},
    ProviderRequestData,
};

mod http;
pub use http::{HttpTransport, HttpTransportError};

#[cfg(feature = "worker")]
mod worker;
#[cfg(feature = "worker")]
pub use worker::{WorkersTransport, WorkersTransportError};

/// Any type that is capable of producing JSON-RPC responses when given JSON-RPC requests. An
/// implementation does not necessarily use the network, but typically does.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[auto_impl(&, Box, Arc)]
pub trait JsonRpcTransport {
    /// Possible errors processing requests.
    type Error: Error + Send + Sync;

    /// Sends a JSON-RPC request to retrieve a response.
    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<JsonRpcResponse<R>, Self::Error>
    where
        P: Serialize + Send + Sync,
        R: DeserializeOwned;

    /// Sends multiple JSON-RPC requests in parallel.
    async fn send_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<JsonRpcResponse<serde_json::Value>>, Self::Error>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync;
}
