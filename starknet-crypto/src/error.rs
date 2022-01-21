/// Errors when performing ECDSA [`sign`](fn.sign) operations
#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("Invalid message hash")]
    InvalidMessageHash,
    #[error("Invalid k")]
    InvalidK,
}

/// Errors when performing ECDSA [`verify`](fn.verify) operations
#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("Invalid message hash")]
    InvalidMessageHash,
    #[error("Invalid r")]
    InvalidR,
    #[error("Invalid s")]
    InvalidS,
}
