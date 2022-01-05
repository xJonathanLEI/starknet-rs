use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    #[serde(rename = "StarknetErrorCode.BLOCK_NOT_FOUND")]
    BlockNotFound,
    #[serde(rename = "StarknetErrorCode.TRANSACTION_NOT_FOUND")]
    TransactionNotFound,
    #[serde(rename = "StarkErrorCode.MALFORMED_REQUEST")]
    MalformedRequest,
}
