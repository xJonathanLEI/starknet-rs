use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::jsonrpc::{
    transports::JsonRpcTransport, JsonRpcMethod, JsonRpcRequest, JsonRpcResponse,
};

#[derive(Debug)]
pub struct HttpTransport {
    client: Client,
    url: Url,
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
    type Error = reqwest::Error;

    async fn send_request<P, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<JsonRpcResponse<R>, Self::Error>
    where
        P: Serialize + Send,
        R: DeserializeOwned,
    {
        let request = self.client.post(self.url.clone()).json(&JsonRpcRequest {
            id: 1,
            jsonrpc: "2.0",
            method,
            params,
        });
        let response = request.send().await?;
        response.json().await
    }
}
