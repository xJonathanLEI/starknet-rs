use std::sync::Arc;

use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{
        chain_id,
        types::{contract::SierraClass, BlockId, BlockTag, FieldElement},
    },
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

#[tokio::main]
async fn main() {
    // Sierra class artifact. Output of the `starknet-compile` command
    let contract_artifact: SierraClass =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();

    // Class hash of the compiled CASM class from the `starknet-sierra-compile` command
    let compiled_class_hash =
        FieldElement::from_hex_be("COMPILED_CASM_CLASS_HASH_IN_HEX_HERE").unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();

    let mut account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // We need to flatten the ABI into a string first
    let flattened_class = contract_artifact.flatten().unwrap();

    let result = account
        .declare(Arc::new(flattened_class), compiled_class_hash)
        .send()
        .await
        .unwrap();

    dbg!(result);
}
