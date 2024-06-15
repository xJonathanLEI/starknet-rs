use async_trait::async_trait;
use log::trace;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse};

/// A [`JsonRpcTransport`] implementation that uses HTTP connections.
#[derive(Debug)]
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
        R: DeserializeOwned,
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

    async fn send_requests<I, P, R>(
        &self,
        requests: I,
    ) -> Result<Vec<JsonRpcResponse<R>>, Self::Error>
    where
        I: IntoIterator<Item = (JsonRpcMethod, P)> + Send,
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        let batch_requests: Vec<_> = requests
            .into_iter()
            .enumerate()
            .map(|(id, (method, params))| JsonRpcRequest {
                id: id as u64 + 1,
                jsonrpc: "2.0",
                method,
                params,
            })
            .collect();

        let serialized_batch = serde_json::to_string(&batch_requests).map_err(Self::Error::Json)?;

        let mut request = self
            .client
            .post(self.url.clone())
            .body(serialized_batch)
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
}
