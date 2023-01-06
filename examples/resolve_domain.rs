use starknet::providers::SequencerGatewayProvider;
use starknet_core::types::FieldElement;
use starknet_id::{address_to_domain, domain_to_address, ResolvingError, GOERLI_CONTRACT};

#[tokio::main]
async fn main() {
    println!("On mainnet:");
    let addr = domain_to_address!("th0rgal.stark").await;
    match addr {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
        },
    }

    let domain_result = address_to_domain!(FieldElement::from_hex_be(
        "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294"
    )
    .unwrap())
    .await;
    match domain_result {
        Ok(domain) => println!("domain: {}", domain),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
        },
    }

    println!("On goerli:");
    let addr = domain_to_address(
        "th0rgal.stark",
        SequencerGatewayProvider::starknet_alpha_goerli(),
        GOERLI_CONTRACT,
    )
    .await;
    match addr {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
        },
    }

    let domain_result = address_to_domain(
        FieldElement::from_hex_be(
            "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294",
        )
        .unwrap(),
        SequencerGatewayProvider::starknet_alpha_goerli(),
        GOERLI_CONTRACT,
    )
    .await;
    match domain_result {
        Ok(domain) => println!("domain: {}", domain),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
        },
    }
}
