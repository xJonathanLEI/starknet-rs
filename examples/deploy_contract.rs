use std::sync::Arc;

use starknet::{
    accounts::SingleOwnerAccount,
    contract::ContractFactory,
    core::{
        chain_id,
        types::{contract::legacy::LegacyContractClass, FieldElement},
    },
    macros::felt,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // NOTE: you will need to declare this class first
    let contract_artifact: LegacyContractClass =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let class_hash = contract_artifact.class_hash().unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(class_hash, account);
    contract_factory
        .deploy(&vec![felt!("123456")], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");
}
