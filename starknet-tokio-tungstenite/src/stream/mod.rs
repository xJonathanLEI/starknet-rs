use std::time::Duration;

use futures_util::StreamExt;
use serde::Deserialize;
use starknet_core::types::{ConfirmedBlockId, Felt, StarknetError, SubscriptionId};
use starknet_providers::{
    jsonrpc::{JsonRpcError, JsonRpcResponse, JsonRpcStreamUpdate},
    StreamUpdateData,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::connect_async;
use tokio_util::sync::CancellationToken;
use tungstenite::{client::IntoClientRequest, Error as TungsteniteError};

mod read;
use read::{ReadAction, StreamReadDriver};

mod write;
pub(crate) use write::WriteAction;
use write::{StreamWriteDriver, SubscribeWriteData};

use crate::{
    error::{CloseError, ConnectError, SubscribeError},
    subscription::{
        EventSubscriptionOptions, EventsSubscription, NewHeadsSubscription,
        PendingTransactionDetailsSubscription, PendingTransactionHashesSubscription, Subscription,
        TransactionStatusSubscription,
    },
};

/// WebSocket stream client powered by `tokio-tungstenite`.
///
/// Internally, this type only holds a handle to send write requests to the underlying stream, not
/// the stream itself. It's therefore safe to be dropped without affecting live subscriptions.
///
/// When this instance _and_ all subscription handles are dropped, the underlying WebSocket stream
/// is closed automatically. Alternatively, the stream can also be closed by calling `.close()`.
#[derive(Debug)]
pub struct TungsteniteStream {
    write_queue: UnboundedSender<WriteAction>,
}

/// All possible update types to be streamed from subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamUpdateType {
    /// New chain head.
    NewHeads,
    /// New event.
    Events,
    /// Initial or changes to a transaction's status.
    TransactionStatus,
    /// New pending transaction.
    PendingTransactions,
    /// Chain reorganization.
    Reorg,
}

/// Internal type for communicating subscribe action results.
#[derive(Debug)]
pub(crate) enum SubscriptionResult {
    Success { id: SubscriptionId },
    JsonRpcError(JsonRpcError),
    TimeoutError,
    TransportError(TungsteniteError),
}

/// Internal type for communicating unsubscribe action results.
#[derive(Debug)]
pub(crate) enum UnsubscribeResult {
    Success { success: bool },
    JsonRpcError(JsonRpcError),
    TimeoutError,
    TransportError(TungsteniteError),
}

/// Internal type for communicating close action results.
#[derive(Debug)]
pub(crate) enum CloseResult {
    Success,
    TimeoutError,
    TransportError(TungsteniteError),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StreamUpdateOrResponse {
    StreamUpdate(JsonRpcStreamUpdate),
    Response(JsonRpcResponse<SubscriptionIdOrBool>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SubscriptionIdOrBool {
    SubscriptionId(SubscriptionId),
    Bool(bool),
}

impl TungsteniteStream {
    /// Establishes a connection to a WebSocket server specified by the request.
    ///
    /// This method attempts to connect to the WebSocket server and sets up the read and write
    /// drivers for handling the stream communication. If the connection isn't established within
    /// the specified timeout, a timeout error is returned.
    ///
    /// The timeout is also used for subsequent actions on the stream such as creating subscriptions.
    ///
    /// # Parameters
    /// * `request` - The WebSocket connection request.
    /// * `timeout` - Maximum duration to wait for connection establishment.
    pub async fn connect<R>(request: R, timeout: Duration) -> Result<Self, ConnectError>
    where
        R: IntoClientRequest,
    {
        let connect = connect_async(request.into_client_request()?);
        let (stream, _) = tokio::select! {
            result = connect => result?,
            _ = tokio::time::sleep(timeout) => {
                return Err(ConnectError::Timeout);
            }
        };

        // Using unbounded channel allows for sync queuing
        let (write_queue_tx, write_queue_rx) =
            tokio::sync::mpsc::unbounded_channel::<WriteAction>();
        let (registration_tx, registration_rx) =
            tokio::sync::mpsc::unbounded_channel::<ReadAction>();

        let (write, read) = stream.split();
        let disconnection_token = CancellationToken::new();

        StreamWriteDriver {
            timeout,
            write_queue: write_queue_rx,
            read_queue: registration_tx,
            sink: write,
            disconnection: disconnection_token.clone(),
        }
        .drive();

        StreamReadDriver {
            registry: Default::default(),
            pending_subscriptions: Default::default(),
            pending_unsubscriptions: Default::default(),
            stream: read,
            read_queue: registration_rx,
            disconnection: disconnection_token,
        }
        .drive();

        Ok(Self {
            write_queue: write_queue_tx,
        })
    }

    /// Subscribes for new chain heads.
    ///
    /// Sends a `starknet_subscribeNewHeads` request to the server.
    pub async fn subscribe_new_heads(
        &self,
        block_id: ConfirmedBlockId,
    ) -> Result<NewHeadsSubscription, SubscribeError> {
        Ok(NewHeadsSubscription {
            inner: self
                .subscribe(SubscribeWriteData::NewHeads { block_id })
                .await?,
        })
    }

    /// Subscribes for new events.
    ///
    /// Sends a `starknet_subscribeEvents` request to the server.
    pub async fn subscribe_events(
        &self,
        options: EventSubscriptionOptions,
    ) -> Result<EventsSubscription, SubscribeError> {
        Ok(EventsSubscription {
            inner: self
                .subscribe(SubscribeWriteData::Events { options })
                .await?,
        })
    }

    /// Subscribes for status changes for a transaction.
    ///
    /// Sends a `starknet_subscribeTransactionStatus` request to the server.
    pub async fn subscribe_transaction_status(
        &self,
        transaction_hash: Felt,
    ) -> Result<TransactionStatusSubscription, SubscribeError> {
        Ok(TransactionStatusSubscription {
            inner: self
                .subscribe(SubscribeWriteData::TransactionStatus { transaction_hash })
                .await?,
        })
    }

    /// Subscribes for new pending transaction hashes.
    ///
    /// Sends a `starknet_subscribePendingTransactions` request to the server.
    pub async fn subscribe_pending_transaction_hashes(
        &self,
        sender_address: Option<Vec<Felt>>,
    ) -> Result<PendingTransactionHashesSubscription, SubscribeError> {
        Ok(PendingTransactionHashesSubscription {
            inner: self
                .subscribe(SubscribeWriteData::PendindTransactions {
                    transaction_details: false,
                    sender_address,
                })
                .await?,
        })
    }

    /// Subscribes for new pending transaction with details.
    ///
    /// Sends a `starknet_subscribePendingTransactions` request to the server.
    pub async fn subscribe_pending_transaction_details(
        &self,
        sender_address: Option<Vec<Felt>>,
    ) -> Result<PendingTransactionDetailsSubscription, SubscribeError> {
        Ok(PendingTransactionDetailsSubscription {
            inner: self
                .subscribe(SubscribeWriteData::PendindTransactions {
                    transaction_details: true,
                    sender_address,
                })
                .await?,
        })
    }

    /// Requests the underlying WebSocket stream to be closed.
    ///
    /// All inflight `.recv()` calls on open subscriptions will yield an `Err` with
    /// [`SubscriptionReceiveError::StreamClosed`](crate::SubscriptionReceiveError::StreamClosed).
    pub async fn close(self) -> Result<(), CloseError> {
        let (result_tx, result_rx) = tokio::sync::oneshot::channel::<CloseResult>();

        match self
            .write_queue
            .send(WriteAction::Close { result: result_tx })
        {
            Ok(_) => {
                // Unwrapping is safe as results are always sent.
                match result_rx.await.unwrap() {
                    // Treat these errors as success
                    CloseResult::Success
                    | CloseResult::TransportError(TungsteniteError::ConnectionClosed)
                    | CloseResult::TransportError(TungsteniteError::AlreadyClosed) => Ok(()),
                    CloseResult::TimeoutError => Err(CloseError::Timeout),
                    CloseResult::TransportError(err) => Err(CloseError::Transport(err)),
                }
            }
            Err(_) => {
                // This means the connection was already closed, likely forced as nothing other than
                // calling this function sends a disconnection request.
                Ok(())
            }
        }
    }

    async fn subscribe(&self, data: SubscribeWriteData) -> Result<Subscription, SubscribeError> {
        let (result_tx, mut result_rx) =
            tokio::sync::mpsc::unbounded_channel::<SubscriptionResult>();
        let (stream_tx, stream_rx) = tokio::sync::mpsc::unbounded_channel::<StreamUpdateData>();

        if self
            .write_queue
            .send(WriteAction::Subscribe {
                data,
                result: result_tx,
                stream: stream_tx,
            })
            .is_err()
        {
            return Err(SubscribeError::Transport(tungstenite::Error::AlreadyClosed));
        }

        // Unwrapping is safe as results are always sent.
        let sub_result = result_rx.recv().await.unwrap();
        match sub_result {
            SubscriptionResult::Success { id } => Ok(Subscription {
                subscription_id: id,
                stream: stream_rx,
                write_queue: self.write_queue.clone(),
                unsubscribed: false,
            }),
            SubscriptionResult::JsonRpcError(err) => match StarknetError::try_from(&err) {
                Ok(StarknetError::TooManyBlocksBack) => Err(SubscribeError::TooManyBlocksBack),
                Ok(StarknetError::BlockNotFound) => Err(SubscribeError::BlockNotFound),
                Ok(StarknetError::TooManyAddressesInFilter) => {
                    Err(SubscribeError::TooManyAddressesInFilter)
                }
                Ok(StarknetError::TooManyKeysInFilter) => Err(SubscribeError::TooManyKeysInFilter),
                _ => Err(SubscribeError::UnexpectedError(err)),
            },
            SubscriptionResult::TimeoutError => Err(SubscribeError::Timeout),
            SubscriptionResult::TransportError(err) => Err(SubscribeError::Transport(err)),
        }
    }
}

impl std::fmt::Display for StreamUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewHeads => write!(f, "NewHeads"),
            Self::Events => write!(f, "Events"),
            Self::TransactionStatus => write!(f, "TransactionStatus"),
            Self::PendingTransactions => write!(f, "PendingTransactions"),
            Self::Reorg => write!(f, "Reorg"),
        }
    }
}
