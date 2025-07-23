use async_trait::async_trait;
use log::trace;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse},
    ProviderRequestData,
};

/// A [`JsonRpcTransport`] implementation that uses HTTP connections.
#[derive(Debug, Clone)]
pub struct HttpTransport {
    client: Client,
    url: Url,
    headers: Vec<(String, String)>,
}

/// Errors using [`HttpTransport`].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum HttpTransportError {
    /// HTTP-related errors.
    Reqwest(reqwest::Error),
    /// JSON serialization/deserialization errors.
    Json(serde_json::Error),
    /// Unexpected response ID.
    #[error("unexpected response ID: {0}")]
    UnexpectedResponseId(u64),
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

impl HttpTransport {
    /// Constructs [`HttpTransport`] from a JSON-RPC server URL, using default HTTP client settings.
    ///
    /// To use custom HTTP settings (e.g. proxy, timeout), use
    /// [`new_with_client`](fn.new_with_client) instead.
    pub fn new(url: impl Into<Url>) -> Self {
        Self::new_with_client(url, Client::new())
    }

    /// Constructs [`HttpTransport`] from a JSON-RPC server URL and a custom `reqwest` client.
    pub fn new_with_client(url: impl Into<Url>, client: Client) -> Self {
        Self {
            client,
            url: url.into(),
            headers: vec![],
        }
    }

    /// Consumes the current [`HttpTransport`] instance and returns a new one with the header
    /// appended. Same as calling [`add_header`](fn.add_header).
    pub fn with_header(self, name: String, value: String) -> Self {
        let mut headers = self.headers;
        headers.push((name, value));

        Self {
            client: self.client,
            url: self.url,
            headers,
        }
    }

    /// Adds a custom HTTP header to be sent for requests.
    pub fn add_header(&mut self, name: String, value: String) {
        self.headers.push((name, value))
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl JsonRpcTransport for HttpTransport {
    type Error = HttpTransportError;

    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<JsonRpcResponse<R>, Self::Error>
    where
        P: Serialize + Send,
        R: DeserializeOwned + Send,
    {
        let request_body = JsonRpcRequest {
            id: 1,
            jsonrpc: "2.0",
            method,
            params,
        };

        let request_body = serde_json::to_string(&request_body).map_err(Self::Error::Json)?;
        trace!("Sending request via JSON-RPC: {}", request_body);

        let mut request = self
            .client
            .post(self.url.clone())
            .body(request_body)
            .header("Content-Type", "application/json");
        for (name, value) in &self.headers {
            request = request.header(name, value);
        }

        let response = request.send().await.map_err(Self::Error::Reqwest)?;

        let response_body = response.text().await.map_err(Self::Error::Reqwest)?;
        trace!("Response from JSON-RPC: {}", response_body);

        let parsed_response = serde_json::from_str(&response_body).map_err(Self::Error::Json)?;

        Ok(parsed_response)
    }

    async fn send_requests<R>(
        &self,
        requests: R,
    ) -> Result<Vec<JsonRpcResponse<serde_json::Value>>, Self::Error>
    where
        R: AsRef<[ProviderRequestData]> + Send + Sync,
    {
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

        let request_body = serde_json::to_string(&request_bodies).map_err(Self::Error::Json)?;
        trace!("Sending request via JSON-RPC: {}", request_body);

        let mut request = self
            .client
            .post(self.url.clone())
            .body(request_body)
            .header("Content-Type", "application/json");
        for (name, value) in &self.headers {
            request = request.header(name, value);
        }

        let response = request.send().await.map_err(Self::Error::Reqwest)?;

        let response_body = response.text().await.map_err(Self::Error::Reqwest)?;
        trace!("Response from JSON-RPC: {}", response_body);

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
                return Err(HttpTransportError::UnexpectedResponseId(id as u64));
            }

            responses[id] = Some(response_item);
        }

        let responses = responses.into_iter().flatten().collect::<Vec<_>>();
        Ok(responses)
    }
}
