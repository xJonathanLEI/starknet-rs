use starknet_providers::{jsonrpc::JsonRpcRequest, ProviderRequestData};

fn main() {
    // Let's pretend this is the raw request body coming from HTTP
    let raw_request = r#"{
    "id": 1,
    "jsonrpc": "2.0",
    "method": "starknet_getBlockTransactionCount",
    "params": [{ "block_number": 200 }]
}"#;

    // Your server framework should handle this for you. Here we deserialize manually to see what's
    // going on.
    let parsed_request =
        serde_json::from_str::<JsonRpcRequest>(raw_request).expect("unable to parse request");

    println!("Request received: {:#?}", parsed_request);

    match parsed_request.data {
        ProviderRequestData::GetBlockTransactionCount(req) => {
            println!(
                "starknet_getBlockTransactionCount request received for block: {:?}",
                req.block_id
            );
        }
        _ => panic!("Request handler for this method has not been implemented"),
    }
}
