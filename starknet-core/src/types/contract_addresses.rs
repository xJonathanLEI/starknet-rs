use ethereum_types::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContractAddresses {
    pub starknet: Address,
    pub gps_statement_verifier: Address,
}
