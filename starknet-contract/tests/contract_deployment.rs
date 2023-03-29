use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_accounts::SingleOwnerAccount;
use starknet_contract::ContractFactory;
use starknet_core::{
    chain_id,
    types::{contract::legacy::LegacyContractClass, FieldElement},
};
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};

#[tokio::test]
async fn can_deploy_contract_to_alpha_goerli() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )
        .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
    )
    .unwrap();
    let account = SingleOwnerAccount::new(provider, signer, address, chain_id::TESTNET);

    let artifact = serde_json::from_str::<LegacyContractClass>(include_str!(
        "../test-data/cairo0/artifacts/oz_account.txt"
    ))
    .unwrap();

    let factory = ContractFactory::new(artifact.class_hash().unwrap(), account);

    let mut salt_buffer = [0u8; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut salt_buffer[1..]);

    let result = factory
        .deploy(
            vec![FieldElement::ONE],
            FieldElement::from_bytes_be(&salt_buffer).unwrap(),
            true,
        )
        .send()
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {err}"),
    }
}
