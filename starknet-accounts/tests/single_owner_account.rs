use starknet_accounts::{Account, Call, SingleOwnerAccount};
use starknet_core::{
    types::{AddTransactionResultCode, BlockId, FieldElement, ContractDefinition, ContractCode, ContractArtifact},
    utils::get_selector_from_name,
};
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};

#[tokio::test]
async fn can_get_nonce() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "059b844bae1727516c6d5c40d2540f6f0a0eebc7eed2adf760515b45dbc20593",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address);

    assert_ne!(
        account.get_nonce(BlockId::Latest).await.unwrap(),
        FieldElement::ZERO
    );
}

#[tokio::test]
async fn can_execute_tst_mint() {
    // This test case is not very useful as the sequencer will always respond with
    // `TransactionReceived` even if the transaction will eventually fail, just like how
    // `eth_sendRawTransaction` always responds with success except for insufficient balance. So it
    // can't really test the execution is successful unless we:
    //   - change to use a local testing network similar to Ganacha for Ethereum; or
    //   - poll the transaction hash until it's processed.

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "059b844bae1727516c6d5c40d2540f6f0a0eebc7eed2adf760515b45dbc20593",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address);
    let nonce = account.get_nonce(BlockId::Pending).await.unwrap();

    let result = account
        .execute(
            &[
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
            ],
            nonce,
        )
        .await
        .unwrap();

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}

#[tokio::test]
async fn can_deploy_account() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "059b844bae1727516c6d5c40d2540f6f0a0eebc7eed2adf760515b45dbc20593",
    )
    .unwrap();
    let account = SingleOwnerAccount::new(provider, signer, address);

    // Create Contract Address Salt
    let contract_address_salt = FieldElement::from_hex_be("0x34edda938a6194021e35f29f0edfd873935f18519f2ee9f417055a8d550a0db").unwrap();

    // Create Constructor Calldata
    let constructor_calldata = vec![
        FieldElement::from_hex_be("0x63d875b04e9b599470550a0236f2d430ec6f83efe00d0af8069490aad15610").unwrap(),
        FieldElement::from_hex_be("0x0").unwrap()
    ];

    // Grab the contract bytecode
    // let raw =
    //     include_str!("../test-data/contract.txt");
    // let cc: ContractCode = serde_json::from_str(raw).unwrap();

    // // Map bytecode field elements to u8s
    // let program_bytecode = cc.bytecode.iter().flat_map(|fe| fe.to_bytes_be()).collect::<Vec<u8>>();

    // Get Contract Entrypoints
    let raw =
        include_str!("../test-data/contract2.txt");
    let ca: ContractArtifact = serde_json::from_str::<ContractArtifact>(raw).unwrap();
    let program_bytecode = ca.program.data.iter().flat_map(|fe| fe.to_bytes_be()).collect::<Vec<u8>>();

    let contract_definition: ContractDefinition = ContractDefinition {
        program: program_bytecode,
        entry_points_by_type: ca.entry_points_by_type,
        abi: Some(ca.abi)
    };

    let result = account
        .deploy_account(
            constructor_calldata,
            contract_definition,
            contract_address_salt,
        )
        .await
        .unwrap();

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}
