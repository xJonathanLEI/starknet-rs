use starknet_accounts::{Account, Call, SingleOwnerAccount};
use starknet_core::{
    types::{AddTransactionResultCode, BlockId, FieldElement},
    utils::get_selector_from_name,
};
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};

#[tokio::test]
#[ignore = "temporarily skipping test until Starkware improves network stability"]
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
#[ignore = "temporarily skipping test until Starkware improves network stability"]
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
            &[Call {
                to: tst_token_address,
                selector: get_selector_from_name("mint").unwrap(),
                calldata: vec![
                    address,
                    FieldElement::from_dec_str("1000000000000000000000").unwrap(),
                    FieldElement::ZERO,
                ],
            }],
            nonce,
        )
        .await
        .unwrap();

    assert_eq!(result.code, AddTransactionResultCode::TransactionReceived);
}
