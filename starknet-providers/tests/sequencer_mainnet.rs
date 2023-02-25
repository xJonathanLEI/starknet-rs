use std::str::FromStr;

use starknet_core::types::{
    AccountTransaction, BlockId, CallL1Handler, FieldElement, InvokeFunctionTransactionRequest,
    L1Address,
};
use starknet_providers::{Provider, SequencerGatewayProvider};

use starknet_id::naming::{ResolvingError, MAINNET_CONTRACT};

fn create_sequencer_client() -> SequencerGatewayProvider {
    SequencerGatewayProvider::starknet_alpha_mainnet()
}

fn felt_hex(hex: &str) -> FieldElement {
    FieldElement::from_hex_be(hex).unwrap()
}

fn felt_dec(dec: &str) -> FieldElement {
    FieldElement::from_dec_str(dec).unwrap()
}

#[tokio::test]
async fn test_domain_to_address() {
    let client = create_sequencer_client();
    
    let addr_result = client.domain_to_address(
        "th0rgal.stark",
        MAINNET_CONTRACT,
    ).await;
    match addr_result {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => panic!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => panic!("Invalid contract result"),
            ResolvingError::InvalidDomain => panic!("Invalid domain"),
        },
    }
}
#[tokio::test]
async fn test_address_to_domain() {
    let client = create_sequencer_client();
    
    let domain_result = client.address_to_domain(
        FieldElement::from_hex_be(
        "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294"
        ).unwrap(),
        MAINNET_CONTRACT,
    ).await;
    match domain_result {
        Ok(domain) => println!("domain: {}", domain),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => panic!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => panic!("Invalid contract result"),
            ResolvingError::InvalidDomain => panic!("Invalid domain"),
        },
    }
}