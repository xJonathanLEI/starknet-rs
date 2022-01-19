use starknet_contract::{ContractArtifact, ContractFactory};
use starknet_core::types::U256;

#[tokio::test]
async fn can_deploy_contract_to_alpha_goerli() {
    let artifact = serde_json::from_str::<ContractArtifact>(include_str!(
        "../test-data/artifacts/oz_account.txt"
    ))
    .unwrap();
    let provider = starknet_providers::SequencerGatewayProvider::starknet_alpha_goerli();

    let factory = ContractFactory::new(artifact, provider).unwrap();

    let result = factory
        .deploy(vec![U256::from_dec_str("1").unwrap()], None)
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {}", err),
    }
}
