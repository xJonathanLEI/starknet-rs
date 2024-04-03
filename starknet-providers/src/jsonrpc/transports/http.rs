use async_trait::async_trait;
use log::trace;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse};

#[derive(Debug)]
pub struct HttpTransport {
    client: Client,
    url: Url,
    headers: Vec<(String, String)>,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum HttpTransportError {
    Reqwest(reqwest::Error),
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
    pub fn new(url: impl Into<Url>) -> Self {
        Self::new_with_client(url, Client::new())
    }

    pub fn new_with_client(url: impl Into<Url>, client: Client) -> Self {
        Self {
            client,
            url: url.into(),
            headers: vec![],
        }
    }

    /// Consumes the current [HttpTransport] instance and returns a new one with the header
    /// appended. Same as calling [add_header].
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
        for (name, value) in self.headers.iter() {
            request = request.header(name, value);
        }

        let response = request.send().await.map_err(Self::Error::Reqwest)?;

        let response_body = response.text().await.map_err(Self::Error::Reqwest)?;
        trace!("Response from JSON-RPC: {}", response_body);

        let parsed_response = serde_json::from_str(&response_body).map_err(Self::Error::Json)?;

        Ok(parsed_response)
    }
}
