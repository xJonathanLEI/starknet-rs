# starknet-tokio-tungstenite

`starknet-tokio-tungstenite` provides a Starknet JSON-RPC WebSocket client implementation for the `tokio` runtime with `tokio-tungstenite`.

Note that `starknet-tokio-tungstenite` is _under active development with foreseeable major breaking changes_, so the **API is unstable** (it still follows SemVer though). Therefore, as of this writing, the crate is not integrated into the rest of the library crates as a dependency to avoid unnecessarily breaking those. To use `starknet-tokio-tungstenite`, you must import it directly:

```console
cargo add starknet-tokio-tungstenite
```

> [!NOTE]
>
> At the moment, the crate offers the WebSocket client **for subscriptions only**. The underlying transport can technically be used for non-subscription requests as well but that has yet to be implemented.
>
> A future version might completely restructure the types to offer a unified interface.

## Usage

### WebSocket subscription

To establish a WebSocket connection, use the `TungsteniteStream::connect` method, which takes a URL and a timeout duration:

```rust
use starknet_tokio_tungstenite::TungsteniteStream;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Connect to a Starknet node with WebSocket support
    let stream = TungsteniteStream::connect("ws://localhost:9545/rpc/v0_8", Duration::from_secs(5))
        .await
        .unwrap();

    // Use the stream for subscriptions
}
```

The `TungsteniteStream` supports all Starknet JSON-RPC subscription types. See the [WebSocket example](../examples/websocket.rs) for a fully functional subscription flow.
