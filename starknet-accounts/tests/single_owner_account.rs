use starknet_accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount};
use starknet_core::{
    chain_id,
    types::{
        contract::{legacy::LegacyContractClass, SierraClass},
        AddTransactionResultCode, FieldElement,
    },
    utils::get_selector_from_name,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, SequencerGatewayProvider,
};
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_goerli()
}

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        url::Url::parse("https://starknet-goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")
            .unwrap(),
    ))
}

#[tokio::test]
async fn can_get_nonce_with_sequencer() {
    can_get_nonce_inner(create_sequencer_client()).await
}

#[tokio::test]
async fn can_get_nonce_with_jsonrpc() {
    can_get_nonce_inner(create_jsonrpc_client()).await
}

#[tokio::test]
async fn can_estimate_fee_with_sequencer() {
    can_estimate_fee_inner(create_sequencer_client()).await
}

#[tokio::test]
async fn can_estimate_fee_with_jsonrpc() {
    can_estimate_fee_inner(create_jsonrpc_client()).await
}

#[tokio::test]
async fn can_simulate_execution_with_sequencer() {
    can_simulate_execution_inner(create_sequencer_client()).await
}

// TODO: add `can_simulate_execution` case when it's supported in pathfinder

#[tokio::test]
async fn can_execute_tst_mint_with_sequencer() {
    can_execute_tst_mint_inner(create_sequencer_client()).await
}

#[tokio::test]
async fn can_execute_tst_mint_with_jsonrpc() {
    can_execute_tst_mint_inner(create_jsonrpc_client()).await
}

#[tokio::test]
async fn can_declare_cairo1_contract_with_sequencer() {
    can_declare_cairo1_contract_inner(create_sequencer_client()).await
}

#[tokio::test]
async fn can_declare_cairo1_contract_with_jsonrpc() {
    can_declare_cairo1_contract_inner(create_jsonrpc_client()).await
}

#[tokio::test]
async fn can_declare_cairo0_contract_with_sequencer() {
    can_declare_cairo0_contract_inner(create_sequencer_client()).await
}

#[tokio::test]
async fn can_declare_cairo0_contract_with_jsonrpc() {
    can_declare_cairo0_contract_inner(create_jsonrpc_client()).await
}

async fn can_get_nonce_inner<P: Provider + Send + Sync>(provider: P) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    assert_ne!(account.get_nonce().await.unwrap(), FieldElement::ZERO);
}

async fn can_estimate_fee_inner<P: Provider + Send + Sync>(provider: P) {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let fee_estimate = account
        .execute(vec![
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("2000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
        ])
        .estimate_fee()
        .await
        .unwrap();

    assert!(fee_estimate.overall_fee > 0);
}

async fn can_simulate_execution_inner<P: Provider + Send + Sync>(provider: P) {
    // Simulates the tx in `can_execute_tst_mint()` without actually sending

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let result = account
        .execute(vec![
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("2000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
        ])
        .simulate()
        .await
        .unwrap();

    assert!(!result
        .trace
        .function_invocation
        .unwrap()
        .internal_calls
        .is_empty());
}

async fn can_execute_tst_mint_inner<P: Provider + Send + Sync>(provider: P) {
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
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let result = account
        .execute(vec![
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("2000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            },
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}

async fn can_declare_cairo1_contract_inner<P: Provider + Send + Sync>(provider: P) {
    // This test case is not very useful, same as `can_execute_tst_mint` above.

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
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

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
        "Declared from starknet-rs test case. Timestamp: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let result = account
        .declare(
            Arc::new(flattened_class),
            FieldElement::from_hex_be(&hashes.compiled_class_hash).unwrap(),
        )
        .send()
        .await
        .unwrap();

    dbg!(&result);

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}

async fn can_declare_cairo0_contract_inner<P: Provider + Send + Sync>(provider: P) {
    // This test case is not very useful, same as `can_execute_tst_mint` above.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let contract_artifact: LegacyContractClass =
        serde_json::from_str(include_str!("../test-data/cairo0/artifacts/oz_account.txt")).unwrap();

    let result = account
        .declare_legacy(Arc::new(contract_artifact))
        .send()
        .await
        .unwrap();

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}
