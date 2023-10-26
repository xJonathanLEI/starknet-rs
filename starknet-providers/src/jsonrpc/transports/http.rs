use async_trait::async_trait;
use log::trace;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse};

#[derive(Debug)]
pub struct HttpTransport {
    client: Client,
    url: Url,
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
        }
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

        let response = self
            .client
            .post(self.url.clone())
            .body(request_body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(Self::Error::Reqwest)?;

        let response_body = response.text().await.map_err(Self::Error::Reqwest)?;
        trace!("Response from JSON-RPC: {}", response_body);

        let mut parsed_response =
            serde_json::from_str(&response_body).map_err(Self::Error::Json)?;

        if let JsonRpcResponse::Error { ref mut error, .. } = parsed_response {
            if error.code == 40 {
                trace!("Extracting `data` from ContractError (40)");
                let json_raw: Value =
                    serde_json::from_str(&response_body).map_err(Self::Error::Json)?;
                // "error" key is safe to unwrap here as we parsed the response correctly with
                // the field "error".
                if let Some(data) = json_raw.get("error").unwrap().get("data") {
                    if let Some(revert_error) = data.get("revert_error") {
                        error.message += &format!(
                            "\nrevert_error: {}",
                            revert_error.as_str().unwrap_or("Not available")
                        );
                    }
                }
            }
        }

        Ok(parsed_response)
    }
}
