use starknet_contract::ContractFactory;
use starknet_core::types::{ContractArtifact, UnsignedFieldElement};
use std::str::FromStr;

#[tokio::test]
#[ignore = "temporarily skipping test until Starkware improves network stability"]
async fn can_deploy_contract_to_alpha_goerli() {
    let artifact = serde_json::from_str::<ContractArtifact>(include_str!(
        "../test-data/artifacts/oz_account.txt"
    ))
    .unwrap();
    let provider = starknet_providers::SequencerGatewayProvider::starknet_alpha_goerli();

    let factory = ContractFactory::new(artifact, provider).unwrap();

    let result = factory
        .deploy(vec![UnsignedFieldElement::from_str("1").unwrap()], None)
        .await;

    match result {
        Ok(_) => {}
        Err(err) => panic!("Contract deployment failed: {}", err),
    }
}
