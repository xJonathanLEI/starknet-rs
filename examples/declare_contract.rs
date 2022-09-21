use std::sync::Arc;

use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        chain_id,
        types::{ContractArtifact, FieldElement},
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();

    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let result = account
        .declare(
            Arc::new(contract_artifact.compress().unwrap()),
            contract_artifact.class_hash().unwrap(),
        )
        .send()
        .await
        .unwrap();

    dbg!(result);
}
