use starknet::{
    contract::ContractFactory, core::types::ContractArtifact, providers::SequencerGatewayProvider,
};

#[tokio::main]
async fn main() {
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("/path/to/contract/artifact.json").unwrap())
            .unwrap();
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let contract_factory = ContractFactory::new(contract_artifact, provider).unwrap();
    let declare_result = contract_factory
        .declare(None)
        .await
        .expect("Unable to declare contract");

    println!(
        "Contract class hash: {:#064x}",
        declare_result.class_hash.expect("Missing class hash")
    );
}
