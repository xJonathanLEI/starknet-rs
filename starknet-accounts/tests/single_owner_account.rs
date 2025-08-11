use starknet_accounts::{
    Account, AccountError, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet_core::{
    types::{contract::SierraClass, Call, ContractExecutionError, Felt, StarknetError},
    utils::get_selector_from_name,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, SequencerGatewayProvider,
};
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;

/// Cairo short string encoding for `SN_SEPOLIA`.
const CHAIN_ID: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_sepolia()
}

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or_else(|_| "https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_9".into());
    JsonRpcClient::new(HttpTransport::new(url::Url::parse(&rpc_url).unwrap()))
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_get_nonce_with_sequencer() {
    can_get_nonce_inner(
        create_sequencer_client(),
        "0x0372943f16a6036bed8ccc986d43ad3859ea05db327b49966198e04645ad2efd",
    )
    .await
}

#[tokio::test]
async fn can_get_nonce_with_jsonrpc() {
    can_get_nonce_inner(
        create_jsonrpc_client(),
        "0x000cf23cc9f3de0b3f8e6922659efe0fd6001c4bbf2162a509230f7e8f22cfe3",
    )
    .await
}

#[tokio::test]
async fn can_estimate_invoke_v3_fee_with_jsonrpc() {
    can_estimate_invoke_v3_fee_inner(
        create_jsonrpc_client(),
        "0x030bf8c9cf629c85160aca40bf2e203cccebf74f2440a346627e7df3f9ab65fd",
    )
    .await
}

#[tokio::test]
async fn can_parse_fee_estimation_error_with_jsonrpc() {
    can_parse_fee_estimation_error_inner(
        create_jsonrpc_client(),
        "0x03f6c60df7b086599c4fa565955d30d60a75ee4f84020adc10d4a8abcc3284e7",
    )
    .await
}

// The `simulate`-related test cases are temporarily removed until it's supported in [Provider]
// TODO: add `simulate` test cases back once transaction simulation in supported

#[tokio::test]
async fn can_execute_eth_transfer_invoke_v3_with_jsonrpc() {
    can_execute_eth_transfer_invoke_v3_inner(
        create_jsonrpc_client(),
        "0x03a08ecef30eaef46780a5167eac194d7cf0407356dccdc7393f851dfc164fd6",
    )
    .await
}

#[tokio::test]
async fn can_execute_eth_transfer_invoke_v3_with_manual_gas_with_jsonrpc() {
    can_execute_eth_transfer_invoke_v3_with_manual_gas_inner(
        create_jsonrpc_client(),
        "0x04a3189bdbc8716f416f7d54d9bf0d0f55ffb454bb89c547118d023a652277dd",
    )
    .await
}

#[tokio::test]
async fn can_estimate_declare_v3_fee_with_jsonrpc() {
    can_estimate_declare_v3_fee_inner(
        create_jsonrpc_client(),
        "0x0678f1879560e7e7e260989ba4911ee170a71c3f25b2467dd2046099aeba92aa",
    )
    .await
}

#[tokio::test]
async fn can_declare_cairo1_contract_v3_with_jsonrpc() {
    can_declare_cairo1_contract_v3_inner(
        create_jsonrpc_client(),
        "0x06aac79bb6c90e1e41c33cd20c67c0281c4a95f01b4e15ad0c3b53fcc6010cf8",
    )
    .await
}

async fn can_get_nonce_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();

    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    assert_ne!(account.get_nonce().await.unwrap(), Felt::ZERO);
}

async fn can_estimate_invoke_v3_fee_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let eth_token_address =
        Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();

    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let fee_estimate = account
        .execute_v3(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![Felt::from_hex("0x1234").unwrap(), Felt::ONE, Felt::ZERO],
        }])
        .estimate_fee()
        .await
        .unwrap();

    assert!(fee_estimate.overall_fee > 0);
}

async fn can_parse_fee_estimation_error_inner<P: Provider + Send + Sync>(
    provider: P,
    address: &str,
) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let eth_token_address =
        Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();

    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    match account
        .execute_v3(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![
                address,
                Felt::from_dec_str("1000000000000000000000").unwrap(),
                Felt::ZERO,
            ],
        }])
        .estimate_fee()
        .await
    {
        Ok(_) => panic!("unexpected successful fee estimation"),
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::TransactionExecutionError(err_data),
        ))) => match err_data.execution_error {
            ContractExecutionError::Nested(_) => {}
            ContractExecutionError::Message(_) => {
                panic!("unexpected error data type")
            }
        },
        _ => panic!("unexpected error type"),
    }
}

async fn can_execute_eth_transfer_invoke_v3_inner<P: Provider + Send + Sync>(
    provider: P,
    address: &str,
) {
    // This test case is not very useful as the sequencer will always respond with
    // `TransactionReceived` even if the transaction will eventually fail, just like how
    // `eth_sendRawTransaction` always responds with success except for insufficient balance. So it
    // can't really test the execution is successful unless we:
    //   - change to use a local testing network similar to Ganacha for Ethereum; or
    //   - poll the transaction hash until it's processed.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let eth_token_address =
        Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();

    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let result = account
        .execute_v3(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![Felt::from_hex("0x1234").unwrap(), Felt::ONE, Felt::ZERO],
        }])
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > Felt::ZERO);
}

async fn can_execute_eth_transfer_invoke_v3_with_manual_gas_inner<P: Provider + Send + Sync>(
    provider: P,
    address: &str,
) {
    // This test tx reverts, as the account does not have sufficient ETH balance. However, the point
    // is to test that a fee estimation is _not_ performed when `gas` is specified. A fee estimation
    // performed on this call would have thrown.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let eth_token_address =
        Felt::from_hex("049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7").unwrap();

    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let result = account
        .execute_v3(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![
                Felt::from_hex("0x1234").unwrap(),
                Felt::from_dec_str("10000000000000000000").unwrap(),
                Felt::ZERO,
            ],
        }])
        .l1_gas(0)
        .l1_gas_price(1000000000000000)
        .l2_gas(1000000)
        .l2_gas_price(10000000000)
        .l1_data_gas(1000)
        .l1_data_gas_price(100000000000000)
        // This tx costs around 10^6 L2 gas. So a tip of 10^10 is around 10^16 FRI (0.01 STRK).
        .tip(1_0000000000)
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > Felt::ZERO);
}

async fn can_estimate_declare_v3_fee_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    #[derive(serde::Deserialize)]
    struct ContractHashes {
        compiled_class_hash: String,
    }

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let contract_artifact = serde_json::from_str::<SierraClass>(include_str!(
        "../test-data/cairo1/artifacts/abi_types_sierra.txt"
    ))
    .unwrap();
    let hashes = serde_json::from_str::<ContractHashes>(include_str!(
        "../test-data/cairo1/artifacts/abi_types.hashes.json"
    ))
    .unwrap();

    // Cairo 1 contract classes are not allowed to be declared multiple times. We spam the network
    // by exploiting the fact that ABI is part of the class hash.
    let mut flattened_class = contract_artifact.flatten().unwrap();
    flattened_class.abi = format!(
        "Declared from starknet-rs test case. Timestamp (ms): {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let result = account
        .declare_v3(
            Arc::new(flattened_class),
            Felt::from_hex(&hashes.compiled_class_hash).unwrap(),
        )
        .estimate_fee()
        .await
        .unwrap();

    assert!(result.overall_fee > 0);
}

async fn can_declare_cairo1_contract_v3_inner<P: Provider + Send + Sync>(
    provider: P,
    address: &str,
) {
    // This test case is not very useful, same as `can_execute_eth_transfer` above.

    #[derive(serde::Deserialize)]
    struct ContractHashes {
        compiled_class_hash: String,
    }

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex("00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    ));
    let address = Felt::from_hex(address).unwrap();
    let account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);

    let contract_artifact = serde_json::from_str::<SierraClass>(include_str!(
        "../test-data/cairo1/artifacts/abi_types_sierra.txt"
    ))
    .unwrap();
    let hashes = serde_json::from_str::<ContractHashes>(include_str!(
        "../test-data/cairo1/artifacts/abi_types.hashes.json"
    ))
    .unwrap();

    // Cairo 1 contract classes are not allowed to be declared multiple times. We spam the network
    // by exploiting the fact that ABI is part of the class hash.
    let mut flattened_class = contract_artifact.flatten().unwrap();
    flattened_class.abi = format!(
        "Declared from starknet-rs test case. Timestamp (ms): {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let result = account
        .declare_v3(
            Arc::new(flattened_class),
            Felt::from_hex(&hashes.compiled_class_hash).unwrap(),
        )
        .l1_gas(0)
        .l1_gas_price(1000000000000000)
        .l2_gas(100000000)
        .l2_gas_price(10000000000)
        .l1_data_gas(1000)
        .l1_data_gas_price(100000000000000)
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > Felt::ZERO);
}
