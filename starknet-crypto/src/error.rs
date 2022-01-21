#[derive(Debug, thiserror::Error)]
pub enum SignError {
    #[error("Invalid message hash")]
    InvalidMessageHash,
    #[error("Invalid k")]
    InvalidK,
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("Invalid message hash")]
    InvalidMessageHash,
    #[error("Invalid r")]
    InvalidR,
    #[error("Invalid s")]
    InvalidS,
}
