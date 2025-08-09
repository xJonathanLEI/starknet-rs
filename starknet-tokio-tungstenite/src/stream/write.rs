use std::time::Duration;

use futures_util::{stream::SplitSink, SinkExt};
use rand::{thread_rng, RngCore};
use starknet_core::types::{
    requests::{
        SubscribeEventsRequest, SubscribeNewHeadsRequest, SubscribeNewTransactionReceiptsRequest,
        SubscribeNewTransactionsRequest, SubscribeTransactionStatusRequest, UnsubscribeRequest,
    },
    ConfirmedBlockId, Felt, L2TransactionFinalityStatus, L2TransactionStatus, SubscriptionId,
};
use starknet_providers::{jsonrpc::JsonRpcRequest, ProviderRequestData, StreamUpdateData};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot::Sender as OneshotSender,
    },
    time::Instant,
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_util::sync::CancellationToken;
use tungstenite::Message;

use crate::subscription::EventSubscriptionOptions;

use super::{
    read::{ReadAcknowledgement, ReadAction},
    CloseResult, SubscriptionResult, UnsubscribeResult,
};

/// An internal type for running the write direction of the WebSocket stream in the background.
pub(crate) struct StreamWriteDriver {
    pub timeout: Duration,
    pub keepalive_interval: Duration,
    pub ping_deadline: Instant,
    pub write_queue: UnboundedReceiver<WriteAction>,
    pub sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    pub read_queue: UnboundedSender<ReadAction>,
    pub disconnection: CancellationToken,
}

#[derive(Debug)]
pub(crate) enum WriteAction {
    Subscribe {
        data: SubscribeWriteData,
        result: UnboundedSender<SubscriptionResult>,
        stream: UnboundedSender<StreamUpdateData>,
    },
    Unsubscribe {
        subscription_id: SubscriptionId,
        result: Option<UnboundedSender<UnsubscribeResult>>,
    },
    Close {
        result: OneshotSender<CloseResult>,
    },
}

#[derive(Debug)]
pub(crate) enum SubscribeWriteData {
    NewHeads {
        block_id: ConfirmedBlockId,
    },
    Events {
        options: EventSubscriptionOptions,
    },
    TransactionStatus {
        transaction_hash: Felt,
    },
    NewTransactionReceipts {
        finality_status: Option<Vec<L2TransactionFinalityStatus>>,
        sender_address: Option<Vec<Felt>>,
    },
    NewTransactions {
        finality_status: Option<Vec<L2TransactionStatus>>,
        sender_address: Option<Vec<Felt>>,
    },
}

enum HandleActionResult {
    /// An action was received and processed successfully.
    Success,
    /// The action queue has been closed and all existing actions have been processed.
    QueueEnded,
}

#[derive(Debug)]
enum SendError {
    Timeout,
    Transport(tungstenite::Error),
}

impl StreamWriteDriver {
    pub fn drive(self) {
        tokio::spawn(self.run());
    }

    async fn run(mut self) {
        let mut close_sent = false;

        loop {
            tokio::select! {
                write_action = self.write_queue.recv() => {
                    if matches!(
                        self.handle_write_action(write_action, &mut close_sent).await,
                        HandleActionResult::QueueEnded
                    ) {
                        // This path is entered only when the stream and all subscriptions have been
                        // dropped, so it's safe to just drop the writer. No messages are lost.
                        break;
                    }
                }
                _ = tokio::time::sleep_until(self.ping_deadline) => {
                    self.ping_deadline += self.keepalive_interval;
                    tokio::select! {
                        _ = self.sink.send(Message::Ping(b"Hello".as_slice().into())) => {},
                        _ = tokio::time::sleep(self.timeout) => {},
                    };
                }
            }
        }
    }

    async fn handle_write_action(
        &mut self,
        action: Option<WriteAction>,
        close_sent: &mut bool,
    ) -> HandleActionResult {
        match action {
            Some(WriteAction::Subscribe {
                data,
                result,
                stream,
            }) => {
                let (ack_tx, ack_rx) = tokio::sync::oneshot::channel::<ReadAcknowledgement>();

                let req_id = thread_rng().next_u32() as u64;
                if self
                    .read_queue
                    .send(ReadAction::Subscribe {
                        request_id: req_id,
                        result: result.clone(),
                        stream,
                        ack: ack_tx,
                    })
                    .is_err()
                {
                    // This failing means the read handler is dropped. There's no point in
                    // retrying anymore.
                    self.write_queue.close();
                    let _ = result.send(SubscriptionResult::TransportError(
                        tungstenite::Error::ConnectionClosed,
                    ));
                    return HandleActionResult::Success;
                }

                // Prevent race condition.
                //
                // The read thread does not block action processing on IO and never panics. The
                // awaiting here should almost always resolve very quickly. It's unnecessary to
                // apply timeout guard here only to add to overhead and code complexity.
                if !matches!(ack_rx.await, Ok(ReadAcknowledgement::Acknowledged)) {
                    // This failing means the read handler is dropped. There's no point in
                    // retrying anymore.
                    self.write_queue.close();
                    let _ = result.send(SubscriptionResult::TransportError(
                        tungstenite::Error::ConnectionClosed,
                    ));
                    return HandleActionResult::Success;
                }

                if let Err(err) = self
                    .send_request(
                        req_id,
                        match data {
                            SubscribeWriteData::NewHeads { block_id } => {
                                ProviderRequestData::SubscribeNewHeads(SubscribeNewHeadsRequest {
                                    block_id: Some(block_id),
                                })
                            }
                            SubscribeWriteData::Events { options } => {
                                ProviderRequestData::SubscribeEvents(SubscribeEventsRequest {
                                    from_address: options.from_address,
                                    keys: options.keys,
                                    block_id: Some(options.block_id),
                                    finality_status: Some(options.finality_status),
                                })
                            }
                            SubscribeWriteData::TransactionStatus { transaction_hash } => {
                                ProviderRequestData::SubscribeTransactionStatus(
                                    SubscribeTransactionStatusRequest { transaction_hash },
                                )
                            }
                            SubscribeWriteData::NewTransactionReceipts {
                                finality_status,
                                sender_address,
                            } => ProviderRequestData::SubscribeNewTransactionReceipts(
                                SubscribeNewTransactionReceiptsRequest {
                                    finality_status,
                                    sender_address,
                                },
                            ),
                            SubscribeWriteData::NewTransactions {
                                finality_status,
                                sender_address,
                            } => ProviderRequestData::SubscribeNewTransactions(
                                SubscribeNewTransactionsRequest {
                                    finality_status,
                                    sender_address,
                                },
                            ),
                        },
                    )
                    .await
                {
                    let _ = result.send(err.into());
                }

                HandleActionResult::Success
            }
            Some(WriteAction::Unsubscribe {
                subscription_id,
                result,
            }) => {
                let (ack_tx, ack_rx) = tokio::sync::oneshot::channel::<ReadAcknowledgement>();

                let req_id = thread_rng().next_u32() as u64;
                if self
                    .read_queue
                    .send(ReadAction::Unsubscribe {
                        request_id: req_id,
                        subscription_id: subscription_id.clone(),
                        result: result.clone(),
                        ack: ack_tx,
                    })
                    .is_err()
                {
                    // This failing means the read handler is dropped. There's no point in
                    // retrying anymore.
                    self.write_queue.close();
                    if let Some(result) = result {
                        let _ = result.send(UnsubscribeResult::TransportError(
                            tungstenite::Error::ConnectionClosed,
                        ));
                    }
                    return HandleActionResult::Success;
                }

                // Prevent race condition.
                //
                // The read thread does not block action processing on IO and never panics. The
                // awaiting here should almost always resolve very quickly. It's unnecessary to
                // apply timeout guard here only to add to overhead and code complexity.
                if !matches!(ack_rx.await, Ok(ReadAcknowledgement::Acknowledged)) {
                    // This failing means the read handler is dropped. There's no point in
                    // retrying anymore.
                    self.write_queue.close();
                    return HandleActionResult::Success;
                }

                if let Err(err) = self
                    .send_request(
                        req_id,
                        ProviderRequestData::Unsubscribe(UnsubscribeRequest { subscription_id }),
                    )
                    .await
                {
                    if let Some(result) = result {
                        let _ = result.send(err.into());
                    }
                }

                HandleActionResult::Success
            }
            Some(WriteAction::Close { result }) => {
                if *close_sent {
                    self.disconnection.cancelled().await;
                    let _ = result.send(CloseResult::Success);
                    return HandleActionResult::Success;
                }

                tokio::select! {
                    send_result = self.sink.send(Message::Close(None)) => {
                        if let Err(err) = send_result {
                            let _ = result.send(CloseResult::TransportError(err));
                            return HandleActionResult::Success;
                        }
                    }
                    _ = tokio::time::sleep(self.timeout) => {
                        let _ = result.send(CloseResult::TimeoutError);
                        return HandleActionResult::Success;
                    }
                };

                *close_sent = true;
                self.disconnection.cancelled().await;
                let _ = result.send(CloseResult::Success);
                self.write_queue.close();

                HandleActionResult::Success
            }
            None => {
                // Write requests are no longer accepted and all existing write requests have
                // been drained.
                //
                // This can happen either when the connection is deemed closed, or when the
                // stream handle and all subscriptions have been dropped.

                // Send a connection close request on a best-effort basis, but it's fine if it
                // doesn't work, as all the handles have been closed anyway.
                if !*close_sent {
                    tokio::select! {
                        _ = self.sink.send(Message::Close(None)) => {},
                        _ = tokio::time::sleep(self.timeout) => {},
                    };
                }

                // There's no need to explicitly notify the stream reader to quit (if hasn't
                // already), as dropping the queue sender already serves that purpose.
                HandleActionResult::QueueEnded
            }
        }
    }

    async fn send_request(
        &mut self,
        id: u64,
        request: ProviderRequestData,
    ) -> Result<(), SendError> {
        let send = self.sink.send(Message::Text(
            serde_json::to_string(&JsonRpcRequest { id, data: request })
                .unwrap()
                .into(),
        ));

        tokio::select! {
            result = send => result.map_err(SendError::Transport),
            _ = tokio::time::sleep(self.timeout) => Err(SendError::Timeout),
        }
    }
}

impl From<SendError> for SubscriptionResult {
    fn from(value: SendError) -> Self {
        match value {
            SendError::Timeout => Self::TimeoutError,
            SendError::Transport(error) => Self::TransportError(error),
        }
    }
}

impl From<SendError> for UnsubscribeResult {
    fn from(value: SendError) -> Self {
        match value {
            SendError::Timeout => Self::TimeoutError,
            SendError::Transport(error) => Self::TransportError(error),
        }
    }
}
