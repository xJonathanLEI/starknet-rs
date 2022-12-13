use starknet::{
    contract::ContractFactory,
    core::types::{ContractArtifact, FieldElement},
    providers::SequencerGatewayProvider,
};

#[tokio::main]
async fn main() {
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let contract_factory = ContractFactory::new(&contract_artifact, provider).unwrap();
    contract_factory
        .deploy(vec![FieldElement::from_dec_str("123456").unwrap()])
        .await
        .expect("Unable to deploy contract");
}
