use rand::RngCore;
use starknet_accounts::{
    Account, AccountError, Call, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet_core::{
    chain_id,
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

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_goerli()
}

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or("https://pathfinder.rpc.goerli.starknet.rs/rpc/v0_6".into());
    JsonRpcClient::new(HttpTransport::new(url::Url::parse(&rpc_url).unwrap()))
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_get_nonce_with_sequencer() {
    can_get_nonce_inner(
        create_sequencer_client(),
        "0x4edd59099fb8f462021abe43a6660c1f0a4b3ffcdaf5483a0846c5bce0ca563",
    )
    .await
}

#[tokio::test]
async fn can_get_nonce_with_jsonrpc() {
    can_get_nonce_inner(
        create_jsonrpc_client(),
        "0x69194dcf3379d2b1747487b6aa0d22d50993c1e2955a74342fcee39ae38c89d",
    )
    .await
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_estimate_fee_with_sequencer() {
    can_estimate_fee_inner(
        create_sequencer_client(),
        "0x6509cb370e06f4c5acc42c7269ac4cf0bfc9afc6c83eca9c040de8b3e24f92e",
    )
    .await
}

#[tokio::test]
async fn can_estimate_fee_with_jsonrpc() {
    can_estimate_fee_inner(
        create_jsonrpc_client(),
        "0x44c3c30803ea9c4e063ae052e6b7ef537284fca6b93849dae9a093e42aa1574",
    )
    .await
}

#[tokio::test]
async fn can_parse_fee_estimation_error_with_jsonrpc() {
    can_parse_fee_estimation_error_inner(
        create_jsonrpc_client(),
        "0x44c3c30803ea9c4e063ae052e6b7ef537284fca6b93849dae9a093e42aa1574",
    )
    .await
}

// The `simulate`-related test cases are temporarily removed until it's supported in [Provider]
// TODO: add `simulate` test cases back once transaction simulation in supported

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_execute_tst_mint_with_sequencer() {
    can_execute_tst_mint_inner(
        create_sequencer_client(),
        "0x1377e5cc40f099c23ef670f0d4979304b8bf975404fe44f4bd78f76eb5014e0",
    )
    .await
}

#[tokio::test]
async fn can_execute_tst_mint_with_jsonrpc() {
    can_execute_tst_mint_inner(
        create_jsonrpc_client(),
        "0x32e340cf84c5e80102031e555ca8b2688855895000d7ad2f2c1fd29e3503ef7",
    )
    .await
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_declare_cairo1_contract_with_sequencer() {
    can_declare_cairo1_contract_inner(
        create_sequencer_client(),
        "0x5d56c86af91e6732f71ceb0fd12d29a86928799f8767a8447d73c9c9a8c1bb4",
    )
    .await
}

#[tokio::test]
async fn can_declare_cairo1_contract_with_jsonrpc() {
    can_declare_cairo1_contract_inner(
        create_jsonrpc_client(),
        "0x2a27190134a9b2f3af972782233764ad22defde1e6ea69608a0820a537e8e1f",
    )
    .await
}

#[tokio::test]
#[ignore = "endpoint deprecated since Starknet v0.12.3"]
async fn can_declare_cairo0_contract_with_sequencer() {
    can_declare_cairo0_contract_inner(
        create_sequencer_client(),
        "0x45dba6ce6a4dc3d2f31aa6da5f51007f1e43e84a1e62c4481bac5454dea4e6d",
    )
    .await
}

#[tokio::test]
async fn can_declare_cairo0_contract_with_jsonrpc() {
    can_declare_cairo0_contract_inner(
        create_jsonrpc_client(),
        "0x2a6bb48ab184f8e5fa5b3050523b2891519308e3a7200f933ab4a5598bed9da",
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

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
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
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

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

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
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

async fn can_execute_tst_mint_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
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
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let mut rng = rand::thread_rng();
    let random_amount = rng.next_u64().into();

    let result = account
        .execute(vec![
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![address, random_amount, FieldElement::ZERO],
            },
            Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    random_amount * FieldElement::TWO,
                    FieldElement::ZERO,
                ],
            },
        ])
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > FieldElement::ZERO);
}

async fn can_declare_cairo1_contract_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
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
    let address = FieldElement::from_hex_be(address).unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
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
        .send()
        .await
        .unwrap();

    dbg!(&result);

    assert!(result.transaction_hash > FieldElement::ZERO);
}

async fn can_declare_cairo0_contract_inner<P: Provider + Send + Sync>(provider: P, address: &str) {
    // This test case is not very useful, same as `can_execute_tst_mint` above.

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(address).unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );
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
        // There seems to be a fee estimation issue with `pathfinder`
        //   https://github.com/eqlabs/pathfinder/issues/1372
        .fee_estimate_multiplier(2.0)
        .send()
        .await
        .unwrap();

    assert!(result.transaction_hash > FieldElement::ZERO);
}
