//! `starknet-tokio-tungstenite` provides a Starknet JSON-RPC WebSocket client implementation for
//! the `tokio` runtime with `tokio-tungstenite`.

#![deny(missing_docs)]

mod error;
pub use error::*;

mod stream;
pub use stream::{StreamUpdateType, TungsteniteStream};

mod subscription;
pub use subscription::*;
