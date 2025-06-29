use std::time::Duration;

use starknet::core::types::ConfirmedBlockId;
use starknet_tokio_tungstenite::{NewHeadsUpdate, TungsteniteStream};

#[tokio::main]
async fn main() {
    let stream = TungsteniteStream::connect("ws://localhost:9545/rpc/v0_8", Duration::from_secs(5))
        .await
        .expect("WebSocket connection failed");

    {
        let mut new_heads_subscription = stream
            .subscribe_new_heads(ConfirmedBlockId::Latest)
            .await
            .unwrap();

        for _ in 0..5 {
            match new_heads_subscription.recv().await {
                Ok(NewHeadsUpdate::NewHeader(head)) => {
                    println!(
                        "Received new chain head #{}: {:#064x}",
                        head.block_number, head.block_hash
                    );
                }
                Ok(NewHeadsUpdate::Reorg(reorg)) => {
                    println!(
                        "Encountered reorg of segment: #{} -> #{}",
                        reorg.starting_block_number, reorg.ending_block_number
                    );
                }
                Err(err) => {
                    eprintln!("Failed to receieve update: {err}");
                }
            }
        }

        // `new_heads_subscription` goes out of scope. A `starknet_unsubscribe` request is
        // automatically sent on drop.
    }

    // `stream` goes out of scope. WebSocket connection is automatically closed.
}
