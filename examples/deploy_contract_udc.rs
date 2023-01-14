use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::{
        chain_id,
        types::{ContractArtifact, FieldElement},
        utils::get_selector_from_name,
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // Defining an account contract in the Testnet
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let account_address =
        FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let account_testnet =
        SingleOwnerAccount::new(&provider, signer, account_address, chain_id::TESTNET);

    // Defining ContractArtifact
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();

    // Deploy assuming that the the class hash has been previously declared
    let udc_address = FieldElement::from_hex_be(
        "0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf",
    )
    .unwrap();

    let salt = FieldElement::from_dec_str("0").unwrap();
    let unique = FieldElement::from_dec_str("0").unwrap();
    let calldata_len = FieldElement::from_dec_str("1").unwrap();
    let calldata = FieldElement::from_dec_str("123456").unwrap();

    let result_deploy = account_testnet
        .execute(&[Call {
            to: udc_address,
            selector: get_selector_from_name("deployContract").unwrap(),
            calldata: vec![
                contract_artifact.class_hash().unwrap(),
                salt,
                unique,
                calldata_len,
                calldata,
            ],
        }])
        .send()
        .await
        .unwrap();

    dbg!(result_deploy);
}
