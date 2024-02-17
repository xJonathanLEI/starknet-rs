use starknet_core::types::Transaction; 
use starknet_providers::{Provider, ProviderError, SequencerGatewayProvider};
use std::str::FromStr;

struct StarkNetConnector {
    sequencer_gateway: SequencerGatewayProvider,
}

impl StarkNetConnector {
    fn new() -> Self {
        Self {
            sequencer_gateway: SequencerGatewayProvider::starknet_alpha_mainnet(),
        }
    }

    async fn get_transaction_details(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<Transaction, ProviderError> {
        self.sequencer_gateway
            .get_transaction_by_hash(transaction_hash)
            .await
    }
}

#[tokio::main]
async fn main() {
    let starknet_connector = StarkNetConnector::new();

    let transaction_hash_str = "0x06bb4f4c85e7e8c05244003ae4d9a370f8faad54bc0bec76456f6cf0a1f26ae6";
    match FieldElement::from_str(transaction_hash_str) {
        Ok(transaction_hash) => {
            match starknet_connector
                .get_transaction_details(transaction_hash)
                .await
            {
                Ok(transaction_details) => println!("{:?}", transaction_details),
                Err(error) => eprintln!("Failed to get transaction details: {:?}", error),
            }
        }
        Err(parse_error) => eprintln!("Failed to parse transaction hash: {:?}", parse_error),
    }
}
