use starknet_providers::jsonrpc::JsonRpcError;
use tungstenite::Error as TungsteniteError;

use crate::StreamUpdateType;

/// Error type for WebSocket connection failures.
#[derive(Debug)]
pub enum ConnectError {
    /// Connection attempt timed out.
    Timeout,
    /// Error in the underlying WebSocket transport.
    Transport(TungsteniteError),
}

/// Error type for subscription failures.
#[derive(Debug)]
pub enum SubscribeError {
    /// Subscription fails due to requesting too many blocks in the past.
    TooManyBlocksBack,
    /// Requested block for subscription was not found.
    BlockNotFound,
    /// Filter contains too many addresses.
    TooManyAddressesInFilter,
    /// Filter contains too many keys.
    TooManyKeysInFilter,
    /// Unexpected error from the JSON-RPC API.
    UnexpectedError(JsonRpcError),
    /// Subscription request timed out.
    Timeout,
    /// Error in the underlying WebSocket transport.
    Transport(TungsteniteError),
}

/// Error type for failures when receiving subscription updates.
#[derive(Debug, Clone)]
pub enum SubscriptionReceiveError {
    /// The WebSocket stream was closed.
    StreamClosed,
    /// Received an update of unexpected type.
    UnexpectedType {
        /// The expected update types.
        expecting: &'static [StreamUpdateType],
        /// The actual update type received.
        actual: StreamUpdateType,
    },
}

/// Error type for unsubscription failures.
#[derive(Debug)]
pub enum UnsubscribeError {
    /// Server unexpectedly returned a failure result.
    UnexpectedResult,
    /// The subscription ID used for unsubscribing is invalid.
    InvalidSubscriptionId,
    /// Unexpected error from the JSON-RPC API.
    UnexpectedError(JsonRpcError),
    /// Unsubscribe request timed out.
    Timeout,
    /// Error in the underlying WebSocket transport.
    Transport(TungsteniteError),
}

/// Error type for WebSocket connection closing failures.
#[derive(Debug)]
pub enum CloseError {
    /// Connection close attempt timed out.
    Timeout,
    /// Error in the underlying WebSocket transport while closing.
    Transport(TungsteniteError),
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "operation timeout"),
            Self::Transport(err) => write!(f, "tungstenite transport error: {err}"),
        }
    }
}

impl std::error::Error for ConnectError {}

impl From<TungsteniteError> for ConnectError {
    fn from(value: TungsteniteError) -> Self {
        Self::Transport(value)
    }
}

impl std::fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyBlocksBack => {
                write!(f, "attempting to subscribe for too many blocks back")
            }
            Self::BlockNotFound => write!(f, "block not found"),
            Self::TooManyAddressesInFilter => write!(f, "too many addresses in filter"),
            Self::TooManyKeysInFilter => write!(f, "too many keys in filter"),
            Self::UnexpectedError(json_rpc_error) => {
                write!(f, "unexpected JSON-RPC error: {json_rpc_error:?}")
            }
            Self::Timeout => write!(f, "operation timeout"),
            Self::Transport(err) => write!(f, "tungstenite transport error: {err}"),
        }
    }
}

impl std::error::Error for SubscribeError {}

impl std::fmt::Display for SubscriptionReceiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StreamClosed => {
                write!(f, "WebSocket stream closed")
            }
            Self::UnexpectedType { expecting, actual } => {
                write!(
                    f,
                    "unexpected subscription update type: {actual}; expecting "
                )?;

                for (ind, item) in expecting.iter().enumerate() {
                    if ind == expecting.len() - 1 {
                        write!(f, "{item}")?
                    } else {
                        write!(f, "{item}, ")?
                    }
                }

                Ok(())
            }
        }
    }
}

impl std::error::Error for SubscriptionReceiveError {}

impl std::fmt::Display for UnsubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedResult => {
                write!(f, "WebSocket server unexpectedly returned `false`")
            }
            Self::InvalidSubscriptionId => {
                write!(f, "invalid subscription ID")
            }
            Self::UnexpectedError(err) => {
                write!(f, "unexpected JSON-RPC error: {err}")
            }
            Self::Timeout => write!(f, "operation timeout"),
            Self::Transport(err) => write!(f, "tungstenite transport error: {err}"),
        }
    }
}

impl std::error::Error for UnsubscribeError {}

impl std::fmt::Display for CloseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "operation timeout"),
            Self::Transport(err) => write!(f, "tungstenite transport error: {err}"),
        }
    }
}

impl std::error::Error for CloseError {}
