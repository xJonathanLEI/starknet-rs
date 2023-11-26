use core::fmt;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use serde::{de::DeserializeOwned, Serialize};

use crate::jsonrpc::{transports::JsonRpcTransport, JsonRpcMethod, JsonRpcResponse};

use super::{HttpTransport, HttpTransportError};

#[derive(Debug)]
pub struct MockTransport {
    // Mock requests lookup
    mocked_requests: HashMap<String, String>,
    // Mock method lookup if request lookup is None
    mocked_methods: HashMap<String, String>,
    // Requests made
    pub requests_log: Arc<Mutex<Vec<(String, String)>>>,
    // HTTP fallback to help build mock requests
    http_transport: Option<HttpTransport>,
}

#[derive(Debug)]
pub struct MissingRequestMock(String);

impl fmt::Display for MissingRequestMock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for MissingRequestMock {}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum MockTransportError {
    Missing(MissingRequestMock),
    Http(HttpTransportError),
    Json(serde_json::Error),
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    id: u64,
    jsonrpc: &'static str,
    method: JsonRpcMethod,
    params: T,
}

impl MockTransport {
    /// Creates a mock transport to use for tests
    /// ```
    ///
    /// ```
    pub fn new(
        http_transport: Option<HttpTransport>,
        requests_log: Arc<Mutex<Vec<(String, String)>>>,
    ) -> Self {
        Self {
            mocked_requests: HashMap::new(),
            mocked_methods: HashMap::new(),
            requests_log,
            http_transport,
        }
    }

    pub fn mock_request(&mut self, request_json: String, response_json: String) {
        self.mocked_requests.insert(request_json, response_json);
    }

    pub fn mock_method(&mut self, method: JsonRpcMethod, response_json: String) {
        let method_str = serde_json::to_string(&method)
            .map_err(MockTransportError::Json)
            .unwrap();
        self.mocked_methods.insert(method_str, response_json);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl JsonRpcTransport for MockTransport {
    type Error = MockTransportError;

    async fn send_request<P: Sync + Send, R>(
        &self,
        method: JsonRpcMethod,
        params: P,
    ) -> Result<JsonRpcResponse<R>, MockTransportError>
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

        let method_str = serde_json::to_string(&method).map_err(MockTransportError::Json)?;

        let request_json =
            serde_json::to_string(&request_body).map_err(MockTransportError::Json)?;

        let response_body;

        if let Some(request_mock) = self.mocked_requests.get(&request_json) {
            response_body = request_mock.clone();
        } else if let Some(method_mock) = self.mocked_methods.get(&method_str) {
            response_body = method_mock.clone();
        } else if let Some(http_transport) = &self.http_transport {
            response_body = http_transport
                .send_request_raw(request_json.clone())
                .await
                .map_err(MockTransportError::Http)?;
            println!("\nUse this code to mock this request\n\n```rs");
            println!("mock_transport.mock_request(\n    r#\"{request_json}\"#.into(),\n    r#\"{response_body}\"#.into()\n);");
            // serde_json::to_string(&resp)?;
            println!("```\n");
        } else {
            return Err(MockTransportError::Missing(MissingRequestMock("".into())));
        }
        self.requests_log
            .lock()
            .unwrap()
            .push((request_json.clone(), response_body.clone()));

        let parsed_response =
            serde_json::from_str(&response_body).map_err(MockTransportError::Json)?;

        Ok(parsed_response)
    }
}
