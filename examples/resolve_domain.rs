use starknet::providers::{Provider, SequencerGatewayProvider};
use starknet_core::types::FieldElement;
use starknet_id::naming::{ResolvingError, GOERLI_CONTRACT, MAINNET_CONTRACT};

#[tokio::main]
async fn main() {
    let client = SequencerGatewayProvider::starknet_alpha_mainnet();
    println!("On mainnet:");
    let addr = client
        .domain_to_address("th0rgal.stark", MAINNET_CONTRACT)
        .await;
    match addr {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }

    let domain_result = client
        .address_to_domain(
            FieldElement::from_hex_be(
                "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294",
            )
            .unwrap(),
            MAINNET_CONTRACT,
        )
        .await;
    match domain_result {
        Ok(domain) => println!("domain: {}", domain),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }
    let client_goerli = SequencerGatewayProvider::starknet_alpha_goerli();

    println!("On goerli:");
    let addr = client_goerli
        .domain_to_address("th0rgal.stark", GOERLI_CONTRACT)
        .await;
    match addr {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }

    let domain_result = client
        .address_to_domain(
            FieldElement::from_hex_be(
                "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294",
            )
            .unwrap(),
            GOERLI_CONTRACT,
        )
        .await;
    match domain_result {
        Ok(domain_result) => println!("domain: {}", domain_result),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }
}
