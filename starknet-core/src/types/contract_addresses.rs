use ethereum_types::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContractAddresses {
    #[serde(rename = "Starknet")]
    pub starknet: Address,
    #[serde(rename = "GpsStatementVerifier")]
    pub gps_statement_verifier: Address,
}
