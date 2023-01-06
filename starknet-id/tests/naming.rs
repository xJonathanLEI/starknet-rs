use starknet_core::types::FieldElement;
use starknet_id::{address_to_domain, domain_to_address, ResolvingError};
use starknet_providers::SequencerGatewayProvider;

#[tokio::test]
async fn domain_to_address_works() {
    let addr_result = domain_to_address!("th0rgal.stark").await;
    match addr_result {
        Ok(_) => {}
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => panic!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => panic!("Invalid contract result"),
            ResolvingError::InvalidDomain => panic!("Invalid domain"),
        },
    }
}

#[tokio::test]
async fn address_to_domain_works() {
    let domain_result = address_to_domain!(FieldElement::from_hex_be(
        "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294"
    )
    .unwrap())
    .await;
    match domain_result {
        Ok(_) => {}
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => panic!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => panic!("Invalid contract result"),
            ResolvingError::InvalidDomain => panic!("Invalid domain"),
        },
    }
}
