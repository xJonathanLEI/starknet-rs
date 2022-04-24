use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, FieldElement},
        utils::get_selector_from_name,
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let tst_token_address = FieldElement::from_hex_be(
        "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);
    let nonce = account.get_nonce(BlockId::Latest).await.unwrap();

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

    dbg!(result);
}
