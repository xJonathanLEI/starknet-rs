use serde::Deserialize;
use starknet_core::types::L1Address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractAddresses {
    pub starknet: L1Address,
    pub gps_statement_verifier: L1Address,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_contract_addresses_deser() {
        // curl -X GET https://alpha4.starknet.io/feeder_gateway/get_contract_addresses
        let raw = r#"{"Starknet": "0xde29d060D45901Fb19ED6C6e959EB22d8626708e", "GpsStatementVerifier": "0xAB43bA48c9edF4C2C4bB01237348D1D7B28ef168"}"#;
        let ca: ContractAddresses = serde_json::from_str(raw).unwrap();
        assert_eq!(
            ca.starknet,
            L1Address::from_str("0xde29d060D45901Fb19ED6C6e959EB22d8626708e").unwrap()
        );
        assert_eq!(
            ca.gps_statement_verifier,
            L1Address::from_str("0xAB43bA48c9edF4C2C4bB01237348D1D7B28ef168").unwrap()
        );
    }
}
