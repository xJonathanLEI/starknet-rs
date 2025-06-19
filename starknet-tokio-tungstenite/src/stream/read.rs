use std::collections::HashMap;

use futures_util::{stream::SplitStream, StreamExt};
use starknet_core::types::SubscriptionId;
use starknet_providers::{jsonrpc::JsonRpcResponse, StreamUpdateData};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{UnboundedReceiver, UnboundedSender},
        oneshot::Sender as OneshotSender,
    },
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_util::sync::CancellationToken;
use tungstenite::Message;

use super::{StreamUpdateOrResponse, SubscriptionIdOrBool, SubscriptionResult, UnsubscribeResult};

/// An internal type for running the read direction of the WebSocket stream in the background.
pub(crate) struct StreamReadDriver {
    pub registry: HashMap<SubscriptionId, UnboundedSender<StreamUpdateData>>,
    pub pending_subscriptions: HashMap<u64, PendingSubscription>,
    pub pending_unsubscriptions: HashMap<u64, PendingUnsubscription>,
    pub stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub read_queue: UnboundedReceiver<ReadAction>,
    pub disconnection: CancellationToken,
}

#[derive(Debug)]
pub(crate) enum ReadAction {
    Subscribe {
        request_id: u64,
        result: UnboundedSender<SubscriptionResult>,
        stream: UnboundedSender<StreamUpdateData>,
        ack: OneshotSender<ReadAcknowledgement>,
    },
    Unsubscribe {
        request_id: u64,
        subscription_id: SubscriptionId,
        result: Option<UnboundedSender<UnsubscribeResult>>,
        ack: OneshotSender<ReadAcknowledgement>,
    },
}

#[derive(Debug)]
pub(crate) enum ReadAcknowledgement {
    Acknowledged,
    StreamClosed,
}

pub(crate) struct PendingSubscription {
    result: UnboundedSender<SubscriptionResult>,
    stream: UnboundedSender<StreamUpdateData>,
}

pub(crate) struct PendingUnsubscription {
    subscription_id: SubscriptionId,
    result: Option<UnboundedSender<UnsubscribeResult>>,
}

enum HandleActionResult {
    /// An action was received and processed successfully.
    Success,
    /// The action queue has been closed and all existing actions have been processed.
    QueueEnded,
}

enum HandleMessageResult {
    /// A message was received and processed successfully.
    Success,
    /// Malformed JSON message received.
    MalformedMessage,
    /// The stream is closed and won't yield any more messages.
    StreamEnded,
    /// Unable to retrieved message due to an unexpectedly closed stream.
    StreamAborted,
}

impl StreamReadDriver {
    pub fn drive(self) {
        tokio::spawn(self.run());
    }

    async fn run(mut self) {
        loop {
            tokio::select! {
                action = self.read_queue.recv() => {
                    if matches!(self.handle_action(action), HandleActionResult::QueueEnded)
                    {
                        // This path is entered only when the stream and all subscriptions have been
                        // dropped, so it's safe to just drop the reader. No updates are lost.
                        break;
                    }
                }
                message = self.stream.next() => {
                    match self.handle_message(message) {
                        HandleMessageResult::Success | HandleMessageResult::MalformedMessage => {}
                        HandleMessageResult::StreamEnded | HandleMessageResult::StreamAborted => {
                            break
                        }
                    }
                }
            }
        }

        // Drain remaining read actions to avoid indefinitely blocking write
        self.read_queue.close();
        while let Some(action) = self.read_queue.recv().await {
            match action {
                ReadAction::Subscribe { ack, .. } | ReadAction::Unsubscribe { ack, .. } => {
                    let _ = ack.send(ReadAcknowledgement::StreamClosed);
                }
            }
        }

        self.disconnection.cancel();
    }

    fn handle_action(&mut self, action: Option<ReadAction>) -> HandleActionResult {
        match action {
            Some(ReadAction::Subscribe {
                request_id,
                result,
                stream,
                ack,
            }) => {
                self.pending_subscriptions
                    .insert(request_id, PendingSubscription { result, stream });

                let _ = ack.send(ReadAcknowledgement::Acknowledged);

                HandleActionResult::Success
            }
            Some(ReadAction::Unsubscribe {
                request_id,
                subscription_id,
                result,
                ack,
            }) => {
                self.pending_unsubscriptions.insert(
                    request_id,
                    PendingUnsubscription {
                        subscription_id,
                        result,
                    },
                );

                let _ = ack.send(ReadAcknowledgement::Acknowledged);

                HandleActionResult::Success
            }
            None => HandleActionResult::QueueEnded,
        }
    }

    fn handle_message(
        &mut self,
        message: Option<Result<Message, tungstenite::Error>>,
    ) -> HandleMessageResult {
        let message = if let Some(Ok(message)) = message {
            message
        } else {
            return HandleMessageResult::StreamAborted;
        };

        match message {
            Message::Text(text) => {
                let parsed_message =
                    match serde_json::from_str::<StreamUpdateOrResponse>(text.as_str()) {
                        Ok(message) => message,
                        Err(_) => return HandleMessageResult::MalformedMessage,
                    };

                match parsed_message {
                    StreamUpdateOrResponse::StreamUpdate(stream_update) => {
                        match self.registry.get_mut(stream_update.data.subscription_id()) {
                            Some(sub_stream) => {
                                if sub_stream.send(stream_update.data).is_err() {
                                    // Subscriptions getting dropped should automatically trigger
                                    // unsubscribing. However, there could be a race condition where
                                    // an update arrives before that. This is normal but probably
                                    // still worth flagging.
                                    log::warn!("WARNING: unable to dump updates");
                                }
                            }
                            None => {
                                // Unsolicited subscription update. This is probably not worth
                                // panicking over.
                                log::warn!("WARNING: unsolicited subscription update");
                            }
                        }
                    }
                    StreamUpdateOrResponse::Response(JsonRpcResponse::Success { id, result }) => {
                        match result {
                            SubscriptionIdOrBool::SubscriptionId(subscription_id) => {
                                // Response for subscribe requests
                                match self.pending_subscriptions.remove(&id) {
                                    Some(pending) => {
                                        match pending.result.send(SubscriptionResult::Success {
                                            id: subscription_id.clone(),
                                        }) {
                                            Ok(_) => {
                                                self.registry
                                                    .insert(subscription_id, pending.stream);
                                            }
                                            Err(_) => {
                                                // This failing here means the caller gave up on
                                                // waiting. We now have a dangling subscription.
                                                //
                                                // Ideally, here we request to unsubscribe to avoid
                                                // useless incoming messages.
                                                //
                                                // TODO: cancel subscription here
                                            }
                                        }
                                    }
                                    None => {
                                        // Unsolicited subscription result. This is probably not
                                        // worth panicking over.
                                        log::warn!("WARNING: unsolicited subscription result");
                                    }
                                }
                            }
                            SubscriptionIdOrBool::Bool(success) => {
                                // Response for unsubscribe requests
                                match self.pending_unsubscriptions.remove(&id) {
                                    Some(pending) => {
                                        // Remove the subscription from internal registry on a best-
                                        // effort basis. It's fine if it doesn't exist.
                                        self.registry.remove(&pending.subscription_id);

                                        if let Some(callback) = pending.result {
                                            // Callback is also best-effort.
                                            let _ = callback
                                                .send(UnsubscribeResult::Success { success });
                                        }
                                    }
                                    None => {
                                        // Unsolicited unsubscribe result. This is probably not
                                        // worth panicking over.
                                        log::warn!("WARNING: unsolicited unsubscribe result");
                                    }
                                }
                            }
                        }
                    }
                    StreamUpdateOrResponse::Response(JsonRpcResponse::Error { id, error }) => {
                        if let Some(pending_sub) = self.pending_subscriptions.remove(&id) {
                            // This failing means the caller gave up on waiting. Ignoring failure is
                            // fine as the subscription didn't succeed anyway.
                            let _ = pending_sub
                                .result
                                .send(SubscriptionResult::JsonRpcError(error));
                        } else if let Some(pending_unsub) = self.pending_unsubscriptions.remove(&id)
                        {
                            // This failing means the caller gave up on waiting. Ignoring failure is
                            // fine as this usually indicates that the subscription doesn't exist.
                            if let Some(result) = pending_unsub.result {
                                let _ = result.send(UnsubscribeResult::JsonRpcError(error));
                            }
                        } else {
                            log::warn!("WARNING: unsolicited error");
                        }
                    }
                }

                HandleMessageResult::Success
            }
            Message::Ping(_) => {
                // Nothing to do here as `tungstenite` handles `Ping` by internally queuing a `Pong`
                // response and it will get flushed automatically on the next read.
                log::trace!("Received Ping message from WebSocket server");
                HandleMessageResult::Success
            }
            Message::Pong(_) => {
                // This is most likely just the server responding to our `Ping`. Nothing to do here.
                log::trace!("Received Pong message from WebSocket server");
                HandleMessageResult::Success
            }
            Message::Close(_) => HandleMessageResult::StreamEnded,
            _ => {
                // Ignore
                // TODO: print trace logs

                HandleMessageResult::Success
            }
        }
    }
}
