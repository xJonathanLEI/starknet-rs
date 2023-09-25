use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, EventFilter, FieldElement},
    macros::{abigen, felt},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::{LocalWallet, SigningKey},
};

use std::sync::Arc;
use url::Url;

// All the events are always grouped in one enun called `Event`
// in the ABI.
abigen!(Contract, "./examples/contracts_abis/events.json");

#[tokio::main]
async fn main() {
    let rpc_url = Url::parse("http://0.0.0.0:5050").unwrap();
    let provider = Arc::new(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let account = SingleOwnerAccount::new(
        provider.clone(),
        signer,
        address,
        felt!("0x4b4154414e41"), // KATANA
        ExecutionEncoding::Legacy,
    );

    let contract_address = FieldElement::from_hex_be("CONTRACT_ADDRESS_HEX").unwrap();

    let event_contract =
        Contract::new(contract_address, Arc::clone(&provider)).with_account(Arc::new(account));

    // Let emits some events by calling two externals.
    event_contract
        .emit_a(&FieldElement::ONE, &vec![felt!("0xff"), felt!("0xf1")])
        .await
        .expect("Emit a invoke failed");

    event_contract
        .emit_b(&felt!("0x1234"))
        .await
        .expect("Emit b invoke failed");

    // Fetch events with some filters with a chunck size of 100 without continuation
    // token.
    // This will not work on the gateway, you need to use JsonRPC node.
    let event_page = provider
        .get_events(
            EventFilter {
                from_block: Some(BlockId::Number(0)),
                to_block: Some(BlockId::Tag(BlockTag::Latest)),
                address: None,
                keys: None,
            },
            None,
            100,
        )
        .await
        .expect("Fetch events failed");

    for e in event_page.events {
        // abigen! macro generate for you the `TryFrom<EmittedEvent` for the
        // `Event` enum.
        let my_event: Event = match e.try_into() {
            Ok(ev) => ev,
            Err(_s) => {
                // An event from other contracts, ignore.
                continue;
            }
        };

        // This way, the deserialization of the event
        // is automatically done based on the variant
        // from the event keys and data.
        match my_event {
            Event::MyEventA(_a) => {
                // do stuff with a.header and a.value.
            }
            Event::MyEventB(_b) => {
                // do stuff with b.value.
            }
        };
    }
}
