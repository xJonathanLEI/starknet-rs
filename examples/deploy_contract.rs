use std::sync::Arc;

use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    contract::ContractFactory,
    core::{
        chain_id,
        types::{contract::legacy::LegacyContractClass, BlockId, BlockTag, FieldElement},
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
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );

    // `SingleOwnerAccount` defaults to checking nonce and estimating fees against the latest
    // block. Optionally change the target block to pending with the following line:
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // Wrapping in `Arc` is meaningless here. It's just showcasing it could be done as
    // `Arc<Account>` implements `Account` too.
    let account = Arc::new(account);

    let contract_factory = ContractFactory::new(class_hash, account);
    contract_factory
        .deploy(vec![felt!("123456")], felt!("1122"), false)
        .send()
        .await
        .expect("Unable to deploy contract");
}
