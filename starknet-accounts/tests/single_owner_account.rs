use starknet_accounts::{Account, Call, SingleOwnerAccount};
use starknet_core::{
    chain_id,
    types::{AddTransactionResultCode, BlockId, FieldElement},
    utils::get_selector_from_name,
};
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use std::sync::Arc;

#[tokio::test]
async fn can_get_nonce() {
    let provider = Arc::new(SequencerGatewayProvider::starknet_alpha_goerli());
    let signer = Arc::new(LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    )));
    let address = FieldElement::from_hex_be(
        "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    assert_ne!(
        account.get_nonce(BlockId::Latest).await.unwrap(),
        FieldElement::ZERO
    );
}

#[tokio::test]
async fn can_estimate_fee() {
    let provider = Arc::new(SequencerGatewayProvider::starknet_alpha_goerli());
    let signer = Arc::new(LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    )));
    let address = FieldElement::from_hex_be(
        "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let fee_estimate = account
        .execute(&[
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

    assert!(fee_estimate.amount > 0);
}

#[tokio::test]
async fn can_execute_tst_mint() {
    // This test case is not very useful as the sequencer will always respond with
    // `TransactionReceived` even if the transaction will eventually fail, just like how
    // `eth_sendRawTransaction` always responds with success except for insufficient balance. So it
    // can't really test the execution is successful unless we:
    //   - change to use a local testing network similar to Ganacha for Ethereum; or
    //   - poll the transaction hash until it's processed.

    let provider = Arc::new(SequencerGatewayProvider::starknet_alpha_goerli());
    let signer = Arc::new(LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    )));
    let address = FieldElement::from_hex_be(
        "01352dd0ac2a462cb53e4f125169b28f13bd6199091a9815c444dcae83056bbc",
    )
    .unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let result = account
        .execute(&[
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
