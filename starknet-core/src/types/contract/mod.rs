/// Module containing types related to artifacts of contracts compiled with a Cairo 0.x compiler.
pub mod legacy;

#[derive(Debug, thiserror::Error)]
pub enum ComputeClassHashError {
    #[error("invalid builtin name")]
    InvalidBuiltinName,
    #[error("json serialization error: {0}")]
    Json(serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CompressProgramError {
    #[error("json serialization error: {0}")]
    Json(serde_json::Error),
    #[error("compression io error: {0}")]
    Io(std::io::Error),
}
