use std::time::Duration;

use starknet_core::types::{ConfirmedBlockId, Felt};
use starknet_tokio_tungstenite::{
    EventSubscriptionOptions, EventsUpdate, NewHeadsUpdate, TransactionStatusUpdate,
    TungsteniteStream,
};

async fn create_stream() -> TungsteniteStream {
    TungsteniteStream::connect(
        "wss://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_9",
        Duration::from_secs(5),
    )
    .await
    .expect("WebSocket connection failed")
}

#[tokio::test]
async fn websocket_new_heads_subscription() {
    let stream = create_stream().await;

    let mut subscription = stream
        .subscribe_new_heads(ConfirmedBlockId::Latest)
        .await
        .unwrap();

    // Stream should immediately return the current chain head
    let NewHeadsUpdate::NewHeader(current_latest_block) =
        tokio::time::timeout(Duration::from_secs(5), subscription.recv())
            .await
            .unwrap()
            .unwrap()
    else {
        panic!("Unexpected update type");
    };

    // Next block should come within 20 seconds (testnet block interval is 5 seconds)
    let NewHeadsUpdate::NewHeader(next_block) =
        tokio::time::timeout(Duration::from_secs(20), subscription.recv())
            .await
            .unwrap()
            .unwrap()
    else {
        panic!("Unexpected update type");
    };

    // Test can fail if a reorg occurs
    assert_eq!(
        current_latest_block.block_number + 1,
        next_block.block_number
    );
}

#[tokio::test]
async fn websocket_events_subscription() {
    let stream = create_stream().await;

    // Subscribe to STRK events
    let mut subscription = stream
        .subscribe_events(EventSubscriptionOptions::default().with_from_address(
            Felt::from_hex_unchecked(
                "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
            ),
        ))
        .await
        .unwrap();

    // There should be at least one event in 20 seconds since it's the fee token
    let EventsUpdate::Event(_) = tokio::time::timeout(Duration::from_secs(20), subscription.recv())
        .await
        .unwrap()
        .unwrap()
    else {
        panic!("Unexpected update type");
    };
}

#[tokio::test]
async fn websocket_transaction_status_subscription() {
    let stream = create_stream().await;

    let mut subscription = stream
        .subscribe_transaction_status(Felt::from_hex_unchecked(
            "0x03f786ecc4955a2602c91a291328518ef866cb7f3d50e4b16fd42282952623aa",
        ))
        .await
        .unwrap();

    // The transaction is already confirmed
    let TransactionStatusUpdate::Status(status) =
        tokio::time::timeout(Duration::from_secs(5), subscription.recv())
            .await
            .unwrap()
            .unwrap()
    else {
        panic!("Unexpected update type");
    };

    assert!(status.status.is_accepted_on_l1());
}

#[tokio::test]
async fn websocket_new_transaction_receipts_subscription() {
    let stream = create_stream().await;

    let mut subscription = stream
        .subscribe_new_transaction_receipts(None, None)
        .await
        .unwrap();

    // There should be at least one transaction in 20 seconds
    let receipt = tokio::time::timeout(Duration::from_secs(20), subscription.recv())
        .await
        .unwrap()
        .unwrap();

    assert_ne!(receipt.receipt.transaction_hash(), &Felt::ZERO);
}

#[tokio::test]
async fn websocket_new_transactions_subscription() {
    let stream = create_stream().await;

    let mut subscription = stream.subscribe_new_transactions(None, None).await.unwrap();

    // There should be at least one transaction in 20 seconds
    let tx = tokio::time::timeout(Duration::from_secs(20), subscription.recv())
        .await
        .unwrap()
        .unwrap();

    assert_ne!(tx.txn.transaction_hash(), &Felt::ZERO);
}
