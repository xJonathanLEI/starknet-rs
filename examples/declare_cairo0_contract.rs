use std::sync::Arc;

use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        chain_id,
        types::{contract::legacy::LegacyContractClass, BlockId, FieldElement},
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    let contract_artifact: LegacyContractClass =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();

    let mut account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Pending);

    let result = account
        .declare_legacy(Arc::new(contract_artifact))
        .send()
        .await
        .unwrap();

    dbg!(result);
}
