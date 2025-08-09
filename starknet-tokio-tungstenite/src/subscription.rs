use starknet_core::types::{
    BlockHeader, ConfirmedBlockId, EmittedEventWithFinality, Felt, L2TransactionFinalityStatus,
    NewTransactionStatus, ReorgData, StarknetError, SubscriptionId,
    TransactionReceiptWithBlockInfo, TransactionWithL2Status,
};
use starknet_providers::StreamUpdateData;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    error::{SubscriptionReceiveError, UnsubscribeError},
    stream::{UnsubscribeResult, WriteAction},
    StreamUpdateType,
};

/// A subscription for retrieving updates from `starknet_subscribeNewHeads` stream.
///
/// # Dropping behavior
///
/// When this subscription is dropped, it automatically sends an unsubscribe
/// request to the server if the subscription hasn't been manually unsubscribed.
/// This ensures proper cleanup of server-side resources without requiring
/// explicit calls to `unsubscribe()`.
#[derive(Debug)]
pub struct NewHeadsSubscription {
    pub(crate) inner: Subscription,
}

/// A subscription for retrieving updates from `starknet_subscribeEvents` stream.
///
/// # Dropping behavior
///
/// When this subscription is dropped, it automatically sends an unsubscribe
/// request to the server if the subscription hasn't been manually unsubscribed.
/// This ensures proper cleanup of server-side resources without requiring
/// explicit calls to `unsubscribe()`.
#[derive(Debug)]
pub struct EventsSubscription {
    pub(crate) inner: Subscription,
}

/// A subscription for retrieving updates from `starknet_subscribeTransactionStatus` stream.
///
/// # Dropping behavior
///
/// When this subscription is dropped, it automatically sends an unsubscribe
/// request to the server if the subscription hasn't been manually unsubscribed.
/// This ensures proper cleanup of server-side resources without requiring
/// explicit calls to `unsubscribe()`.
#[derive(Debug)]
pub struct TransactionStatusSubscription {
    pub(crate) inner: Subscription,
}

/// A subscription for retrieving updates from `starknet_subscribeNewTransactionReceipts` stream.
///
/// # Dropping behavior
///
/// When this subscription is dropped, it automatically sends an unsubscribe
/// request to the server if the subscription hasn't been manually unsubscribed.
/// This ensures proper cleanup of server-side resources without requiring
/// explicit calls to `unsubscribe()`.
#[derive(Debug)]
pub struct NewTransactionReceiptsSubscription {
    pub(crate) inner: Subscription,
}

/// A subscription for retrieving updates from `starknet_subscribeNewTransactions` stream.
///
/// # Dropping behavior
///
/// When this subscription is dropped, it automatically sends an unsubscribe
/// request to the server if the subscription hasn't been manually unsubscribed.
/// This ensures proper cleanup of server-side resources without requiring
/// explicit calls to `unsubscribe()`.
#[derive(Debug)]
pub struct NewTransactionsSubscription {
    pub(crate) inner: Subscription,
}

/// Update from a new heads subscription.
///
/// Represents either a new block header or a chain reorganization.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum NewHeadsUpdate {
    /// A new block header has been added to the chain.
    NewHeader(BlockHeader),
    /// A chain reorganization has occurred.
    Reorg(ReorgData),
}

/// Update from an events subscription.
///
/// Represents either a new emitted event or a chain reorganization.
#[derive(Debug)]
pub enum EventsUpdate {
    /// A new event has been emitted.
    Event(EmittedEventWithFinality),
    /// A chain reorganization has occurred.
    Reorg(ReorgData),
}

/// Update from a transaction status subscription.
///
/// Represents either a transaction status change or a chain reorganization.
#[derive(Debug)]
pub enum TransactionStatusUpdate {
    /// Updated status of a transaction.
    Status(NewTransactionStatus),
    /// A chain reorganization has occurred.
    Reorg(ReorgData),
}

/// Options for subscribing to Starknet events.
#[derive(Debug, Clone)]
pub struct EventSubscriptionOptions {
    /// Filter events by contract address.
    pub from_address: Option<Felt>,
    /// Filter events by keys (array of event key arrays).
    pub keys: Option<Vec<Vec<Felt>>>,
    /// The block from which to start receiving events.
    pub block_id: ConfirmedBlockId,
    /// The finality status of the most recent events to include.
    pub finality_status: L2TransactionFinalityStatus,
}

#[derive(Debug)]
pub(crate) struct Subscription {
    pub subscription_id: SubscriptionId,
    pub stream: UnboundedReceiver<StreamUpdateData>,
    pub write_queue: UnboundedSender<WriteAction>,
    pub unsubscribed: bool,
}

impl EventSubscriptionOptions {
    /// Creates a new `EventSubscriptionOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the contract address to filter events by.
    pub fn with_from_address(mut self, from_address: Felt) -> Self {
        self.from_address = Some(from_address);
        self
    }

    /// Sets the event keys to filter events by.
    pub fn with_keys(mut self, keys: Vec<Vec<Felt>>) -> Self {
        self.keys = Some(keys);
        self
    }

    /// Sets the block ID from which to start receiving events.
    pub fn with_block_id(mut self, block_id: ConfirmedBlockId) -> Self {
        self.block_id = block_id;
        self
    }
}

impl NewHeadsSubscription {
    /// Receives the next update from the subscription.
    ///
    /// Returns a new block header or chain reorganization notification.
    pub async fn recv(&mut self) -> Result<NewHeadsUpdate, SubscriptionReceiveError> {
        match self.inner.stream.recv().await {
            Some(StreamUpdateData::SubscriptionNewHeads(update)) => {
                Ok(NewHeadsUpdate::NewHeader(update.result))
            }
            Some(StreamUpdateData::SubscriptionReorg(update)) => {
                Ok(NewHeadsUpdate::Reorg(update.result))
            }
            Some(StreamUpdateData::SubscriptionEvents(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewHeads, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::Events,
                })
            }
            Some(StreamUpdateData::SubscriptionTransactionStatus(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewHeads, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::TransactionStatus,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransactionReceipts(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewHeads, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransactionReceipts,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransaction(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewHeads, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransaction,
                })
            }
            None => Err(SubscriptionReceiveError::StreamClosed),
        }
    }

    /// Unsubscribes from this subscription.
    ///
    /// Sends an unsubscribe request to the server to stop receiving updates.
    pub async fn unsubscribe(self) -> Result<(), UnsubscribeError> {
        self.inner.unsubscribe().await
    }
}

impl EventsSubscription {
    /// Receives the next update from the subscription.
    ///
    /// Returns a new event or chain reorganization notification.
    pub async fn recv(&mut self) -> Result<EventsUpdate, SubscriptionReceiveError> {
        match self.inner.stream.recv().await {
            Some(StreamUpdateData::SubscriptionEvents(update)) => {
                Ok(EventsUpdate::Event(update.result))
            }
            Some(StreamUpdateData::SubscriptionReorg(update)) => {
                Ok(EventsUpdate::Reorg(update.result))
            }
            Some(StreamUpdateData::SubscriptionNewHeads(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::Events, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewHeads,
                })
            }
            Some(StreamUpdateData::SubscriptionTransactionStatus(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::Events, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::TransactionStatus,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransactionReceipts(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::Events, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransactionReceipts,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransaction(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::Events, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransaction,
                })
            }
            None => Err(SubscriptionReceiveError::StreamClosed),
        }
    }

    /// Unsubscribes from this subscription.
    ///
    /// Sends an unsubscribe request to the server to stop receiving updates.
    pub async fn unsubscribe(self) -> Result<(), UnsubscribeError> {
        self.inner.unsubscribe().await
    }
}

impl TransactionStatusSubscription {
    /// Receives the next update from the subscription.
    ///
    /// Returns a transaction status update or chain reorganization notification.
    pub async fn recv(&mut self) -> Result<TransactionStatusUpdate, SubscriptionReceiveError> {
        match self.inner.stream.recv().await {
            Some(StreamUpdateData::SubscriptionTransactionStatus(update)) => {
                Ok(TransactionStatusUpdate::Status(update.result))
            }
            Some(StreamUpdateData::SubscriptionReorg(update)) => {
                Ok(TransactionStatusUpdate::Reorg(update.result))
            }
            Some(StreamUpdateData::SubscriptionNewHeads(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::TransactionStatus, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewHeads,
                })
            }
            Some(StreamUpdateData::SubscriptionEvents(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::TransactionStatus, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::Events,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransactionReceipts(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::TransactionStatus, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransactionReceipts,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransaction(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::TransactionStatus, StreamUpdateType::Reorg],
                    actual: StreamUpdateType::NewTransaction,
                })
            }
            None => Err(SubscriptionReceiveError::StreamClosed),
        }
    }

    /// Unsubscribes from this subscription.
    ///
    /// Sends an unsubscribe request to the server to stop receiving updates.
    pub async fn unsubscribe(self) -> Result<(), UnsubscribeError> {
        self.inner.unsubscribe().await
    }
}

impl NewTransactionReceiptsSubscription {
    /// Receives the next new transaction receipt from the subscription.
    ///
    /// Returns the transaction receipt.
    pub async fn recv(
        &mut self,
    ) -> Result<TransactionReceiptWithBlockInfo, SubscriptionReceiveError> {
        match self.inner.stream.recv().await {
            Some(StreamUpdateData::SubscriptionNewTransactionReceipts(update)) => Ok(update.result),
            Some(StreamUpdateData::SubscriptionNewHeads(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransactionReceipts],
                    actual: StreamUpdateType::NewHeads,
                })
            }
            Some(StreamUpdateData::SubscriptionEvents(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransactionReceipts],
                    actual: StreamUpdateType::Events,
                })
            }
            Some(StreamUpdateData::SubscriptionTransactionStatus(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransactionReceipts],
                    actual: StreamUpdateType::TransactionStatus,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransaction(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransactionReceipts],
                    actual: StreamUpdateType::NewTransaction,
                })
            }
            Some(StreamUpdateData::SubscriptionReorg(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransactionReceipts],
                    actual: StreamUpdateType::Reorg,
                })
            }
            None => Err(SubscriptionReceiveError::StreamClosed),
        }
    }

    /// Unsubscribes from this subscription.
    ///
    /// Sends an unsubscribe request to the server to stop receiving updates.
    pub async fn unsubscribe(self) -> Result<(), UnsubscribeError> {
        self.inner.unsubscribe().await
    }
}

impl NewTransactionsSubscription {
    /// Receives the next new transaction from the subscription.
    ///
    /// Returns the transaction alongside its layer-2 status.
    pub async fn recv(&mut self) -> Result<TransactionWithL2Status, SubscriptionReceiveError> {
        match self.inner.stream.recv().await {
            Some(StreamUpdateData::SubscriptionNewTransaction(update)) => Ok(update.result),
            Some(StreamUpdateData::SubscriptionNewHeads(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransaction],
                    actual: StreamUpdateType::NewHeads,
                })
            }
            Some(StreamUpdateData::SubscriptionEvents(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransaction],
                    actual: StreamUpdateType::Events,
                })
            }
            Some(StreamUpdateData::SubscriptionTransactionStatus(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransaction],
                    actual: StreamUpdateType::TransactionStatus,
                })
            }
            Some(StreamUpdateData::SubscriptionNewTransactionReceipts(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransaction],
                    actual: StreamUpdateType::NewTransactionReceipts,
                })
            }
            Some(StreamUpdateData::SubscriptionReorg(_)) => {
                Err(SubscriptionReceiveError::UnexpectedType {
                    expecting: &[StreamUpdateType::NewTransaction],
                    actual: StreamUpdateType::Reorg,
                })
            }
            None => Err(SubscriptionReceiveError::StreamClosed),
        }
    }

    /// Unsubscribes from this subscription.
    ///
    /// Sends an unsubscribe request to the server to stop receiving updates.
    pub async fn unsubscribe(self) -> Result<(), UnsubscribeError> {
        self.inner.unsubscribe().await
    }
}

impl Subscription {
    /// Sends an unsubscribe request to the server and processes the response.
    ///
    /// This is an internal method used by public subscription types.
    async fn unsubscribe(mut self) -> Result<(), UnsubscribeError> {
        let (result_tx, mut result_rx) =
            tokio::sync::mpsc::unbounded_channel::<UnsubscribeResult>();

        if self
            .write_queue
            .send(WriteAction::Unsubscribe {
                subscription_id: self.subscription_id.clone(),
                result: Some(result_tx),
            })
            .is_err()
        {
            // Connection already closed. No need to waste effort on drop.
            self.unsubscribed = true;

            return Err(UnsubscribeError::Transport(
                tungstenite::Error::AlreadyClosed,
            ));
        }

        let unsub_result = result_rx.recv().await.ok_or(UnsubscribeError::Transport(
            tungstenite::Error::AlreadyClosed,
        ))?;
        match unsub_result {
            UnsubscribeResult::Success { success } => {
                self.unsubscribed = success;

                if success {
                    Ok(())
                } else {
                    Err(UnsubscribeError::UnexpectedResult)
                }
            }
            UnsubscribeResult::JsonRpcError(err) => match StarknetError::try_from(&err) {
                Ok(StarknetError::InvalidSubscriptionId) => {
                    Err(UnsubscribeError::InvalidSubscriptionId)
                }
                _ => Err(UnsubscribeError::UnexpectedError(err)),
            },
            UnsubscribeResult::TimeoutError => Err(UnsubscribeError::Timeout),
            UnsubscribeResult::TransportError(err) => Err(UnsubscribeError::Transport(err)),
        }
    }
}

impl Default for EventSubscriptionOptions {
    fn default() -> Self {
        Self {
            from_address: None,
            keys: None,
            block_id: ConfirmedBlockId::Latest,
            finality_status: L2TransactionFinalityStatus::AcceptedOnL2,
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if !self.unsubscribed {
            // This failing means the connection is already broken anyway.
            let _ = self.write_queue.send(WriteAction::Unsubscribe {
                subscription_id: self.subscription_id.clone(),
                result: None,
            });
        }
    }
}
