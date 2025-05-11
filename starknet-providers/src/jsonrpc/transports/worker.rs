use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::{
    jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse},
    ProviderRequestData,
};

/// A [`JsonRpcTransport`] implementation for the Cloudflare Workers environment.
#[derive(Debug, Clone)]
pub struct WorkersTransport {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
    url: Url,
}

/// Errors using [`WorkersTransport`].
#[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorkersTransportError {
    /// JSON serialization/deserialization errors.
    Json(serde_json::Error),
    /// Workers SDK error.
    Workers(worker::Error),
    /// Unexpected response ID.
    #[error("unexpected response ID: {0}")]
    UnexpectedResponseId(u64),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

impl WorkersTransport {
    /// Constructs [`WorkersTransport`] from a JSON-RPC server URL.
    pub fn new(url: impl Into<Url>) -> Self {
        Self { url: url.into() }
    }
}

// Stub implementation to make allow compiling outside Workers environment. This makes development
// easier.
#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl JsonRpcTransport for WorkersTransport {
    type Error = WorkersTransportError;

    async fn send_request<P, R>(
        &self,
        _method: JsonRpcMethod,
        _params: P,
    ) -> Result<JsonRpcResponse<R>, Self::Error>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        panic!("Cloudflare Workers transport is only supported in WASM")
    }

    async fn send_requests<R>(
        &self,
        _requests: R,
    ) -> Result<Vec<JsonRpcResponse<serde_json::Value>>, Self::Error>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync,
    {
        panic!("Cloudflare Workers transport is only supported in WASM")
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl JsonRpcTransport for WorkersTransport {
    type Error = WorkersTransportError;

    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<JsonRpcResponse<R>, Self::Error>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        use worker::*;

        let request_body = JsonRpcRequest {
            id: 1,
            jsonrpc: "2.0",
            method,
            params,
        };

        let request_body = serde_json::to_string(&request_body)?;

        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json")?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(request_body.into()));

        let req = Request::new_with_init(self.url.as_ref(), &init)?;
        let mut response = Fetch::Request(req).send().await?;
        let response_body = response.text().await?;

        let parsed_response = serde_json::from_str(&response_body)?;

        Ok(parsed_response)
    }

    async fn send_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<JsonRpcResponse<serde_json::Value>>, Self::Error>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync,
    {
        use worker::*;

        let request_bodies = requests
            .as_ref()
            .iter()
            .enumerate()
            .map(|(ind, request)| JsonRpcRequest {
                id: ind as u64,
                jsonrpc: "2.0",
                method: request.jsonrpc_method(),
                params: request,
            })
            .collect::<Vec<_>>();

        let request_count = request_bodies.len();

        let request_body = serde_json::to_string(&request_bodies)?;

        let mut headers = Headers::new();
        headers.append("Content-Type", "application/json")?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(request_body.into()));

        let req = Request::new_with_init(self.url.as_ref(), &init)?;
        let mut response = Fetch::Request(req).send().await?;
        let response_body = response.text().await?;

        let parsed_response: Vec<JsonRpcResponse<serde_json::Value>> =
            serde_json::from_str(&response_body).map_err(Self::Error::Json)?;

        let mut responses: Vec<Option<JsonRpcResponse<serde_json::Value>>> = vec![];
        responses.resize(request_bodies.len(), None);

        // Re-order the responses as servers do not maintain order.
        for response_item in parsed_response {
            let id = match &response_item {
                JsonRpcResponse::Success { id, .. } | JsonRpcResponse::Error { id, .. } => {
                    *id as usize
                }
            };

            if id >= request_count {
                return Err(Self::Error::UnexpectedResponseId(id as u64));
            }

            responses[id] = Some(response_item);
        }

        let responses = responses.into_iter().flatten().collect::<Vec<_>>();
        Ok(responses)
    }
}

impl From<serde_json::Error> for WorkersTransportError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<worker::Error> for WorkersTransportError {
    fn from(value: worker::Error) -> Self {
        Self::Workers(value)
    }
}
