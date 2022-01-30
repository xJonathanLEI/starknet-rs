use starknet_accounts::{Account, SingleOwnerAccount};
use starknet_core::types::UnsignedFieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};

#[tokio::test]
async fn can_get_nonce() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        UnsignedFieldElement::from_hex_str(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = UnsignedFieldElement::from_hex_str(
        "0649cd069661cad8e54c35076f9eca8c3a794379569eb6ba9ec0669055a4d167",
    )
    .unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address);

    assert_ne!(
        account.get_nonce(None).await.unwrap(),
        UnsignedFieldElement::ZERO
    );
}
