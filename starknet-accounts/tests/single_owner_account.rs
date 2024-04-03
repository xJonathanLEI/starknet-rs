use starknet_accounts::{
    Account, AccountError, Call, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet_core::{
    types::{
        contract::{
            legacy::{LegacyContractClass, RawLegacyAbiEntry, RawLegacyFunction},
            SierraClass,
        },
        BlockId, BlockTag, FieldElement, StarknetError,
    },
    utils::get_selector_from_name,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, SequencerGatewayProvider,
};
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;

/// Cairo short string encoding for `SN_SEPOLIA`.
const CHAIN_ID: FieldElement = FieldElement::from_mont([
    1555806712078248243,
    18446744073708869172,
    18446744073709551615,
    507980251676163170,
]);

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_sepolia()
}

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_7".into());
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
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_estimate_fee_with_sequencer() {
    can_estimate_fee_inner(
        create_sequencer_client(),
        "0x054c9746cfae36f616222e450ae5c8eadf93e5705d4e5d077b9bce5a06f87ee2",
    )
    .await
}

#[tokio::test]
async fn can_estimate_fee_with_jsonrpc() {
    can_estimate_fee_inner(
        create_jsonrpc_client(),
        "0x06d3f2113fca3c4eb6da508c5da3616bf219d84cd47692df3f1f78183a9f5f59",
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
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_execute_eth_transfer_with_sequencer() {
    can_execute_eth_transfer_inner(
        create_sequencer_client(),
        "0x05ea1832b1e399cdcf8ae8184ff881f121d2ecc98aaebe6070ec17518bc2f668",
    )
    .await
}

#[tokio::test]
async fn can_execute_eth_transfer_with_jsonrpc() {
    can_execute_eth_transfer_inner(
        create_jsonrpc_client(),
        "0x056a817d8cbc2834f7b00aa3a0bf6a16ae0d060445d65f31b4a2bf0140b14afd",
    )
    .await
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_declare_cairo1_contract_with_sequencer() {
    can_declare_cairo1_contract_inner(
        create_sequencer_client(),
        "0x04225fdb21c93800832e047d29e5a929bf65f95ab7c1ba101d66d0419661b7df",
    )
    .await
}

#[tokio::test]
async fn can_declare_cairo1_contract_with_jsonrpc() {
    can_declare_cairo1_contract_inner(
        create_jsonrpc_client(),
        "0x00af46a3d75c1abc204cbe7e08f220680958bd8aca2c3cfc2ef34c686148ecf7",
    )
    .await
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_declare_cairo0_contract_with_sequencer() {
    can_declare_cairo0_contract_inner(
        create_sequencer_client(),
        "0x038fe7f6cb2895f2016f0dc4799ede9493e2e01fe02814e8d038499eb3935864",
    )
    .await
}

#[tokio::test]
async fn can_declare_cairo0_contract_with_jsonrpc() {
    can_declare_cairo0_contract_inner(
        create_jsonrpc_client(),
        "0x02cc631ca0c544639f6e4403b8f3611696a3d831e8157ea1c946e35429c7ac31",
    )
    .await
}

async fn can_get_nonce_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    assert_ne!(account.get_nonce().await.unwrap(), FieldElement::ZERO);
}

async fn can_estimate_fee_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let eth_token_address = FieldElement::from_hex_be(
        "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    )
    .unwrap();

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let fee_estimate = account
        .execute(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![
                FieldElement::from_hex_be("0x1234").unwrap(),
                FieldElement::ONE,
                FieldElement::ZERO,
            ],
        }])
        .estimate_fee()
        .await
        .unwrap();

    assert!(fee_estimate.overall_fee > FieldElement::ZERO);
}

async fn can_parse_fee_estimation_error_inner<P: Provider + Send + Sync>(
    provider: P,
    address: &str,
) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let eth_token_address = FieldElement::from_hex_be(
        "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    )
    .unwrap();

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    match account
        .execute(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![
                address,
                FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                FieldElement::ZERO,
            ],
        }])
        .estimate_fee()
        .await
    {
        Ok(_) => panic!("unexpected successful fee estimation"),
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::TransactionExecutionError(err_data),
        ))) => {
            assert!(!err_data.execution_error.is_empty());
        }
        _ => panic!("unexpected error type"),
    }
}

async fn can_execute_eth_transfer_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    // This test case is not very useful as the sequencer will always respond with
    // `TransactionReceived` even if the transaction will eventually fail, just like how
    // `eth_sendRawTransaction` always responds with success except for insufficient balance. So it
    // can't really test the execution is successful unless we:
    //   - change to use a local testing network similar to Ganacha for Ethereum; or
    //   - poll the transaction hash until it's processed.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let eth_token_address = FieldElement::from_hex_be(
        "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    )
    .unwrap();

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let result = account
        .execute(vec![Call {
            to: eth_token_address,
            selector: get_selector_from_name("transfer").unwrap(),
            calldata: vec![
                FieldElement::from_hex_be("0x1234").unwrap(),
                FieldElement::ONE,
                FieldElement::ZERO,
            ],
        }])
        .max_fee(FieldElement::from_dec_str("1000000000000000000").unwrap())
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > FieldElement::ZERO);
}

async fn can_declare_cairo1_contract_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    // This test case is not very useful, same as `can_execute_eth_transfer` above.

    #[derive(serde::Deserialize)]
    struct ContractHashes {
        compiled_class_hash: String,
    }

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

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
        .declare(
            Arc::new(flattened_class),
            FieldElement::from_hex_be(&hashes.compiled_class_hash).unwrap(),
        )
        .max_fee(FieldElement::from_dec_str("1000000000000000000").unwrap())
        .send()
        .await
        .unwrap();

    dbg!(&result);

    assert!(result.transaction_hash > FieldElement::ZERO);
}

async fn can_declare_cairo0_contract_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    // This test case is not very useful, same as `can_execute_eth_transfer` above.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let mut account =
        SingleOwnerAccount::new(provider, signer, address, CHAIN_ID, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let mut contract_artifact: LegacyContractClass =
        serde_json::from_str(include_str!("../test-data/cairo0/artifacts/oz_account.txt")).unwrap();

    // Since Starknet v0.12.0 identical transactions are no longer allowed. We make transactions
    // unique by appending a fake ABI entry.
    contract_artifact
        .abi
        .push(RawLegacyAbiEntry::Function(RawLegacyFunction {
            inputs: vec![],
            name: format!(
                "Declared from starknet-rs test case. Timestamp (ms): {}",
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            ),
            outputs: vec![],
            state_mutability: None,
        }));

    let result = account
        .declare_legacy(Arc::new(contract_artifact))
        .max_fee(FieldElement::from_dec_str("1000000000000000000").unwrap())
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > FieldElement::ZERO);
}
