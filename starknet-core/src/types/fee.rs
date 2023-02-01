use serde::Deserialize;

use super::TransactionTrace;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct FeeEstimate {
    pub overall_fee: u64,
    pub unit: FeeUnit,
    pub gas_price: u64,
    pub gas_usage: u64,
}

/// Represents the information regarding a Starknet transaction's simulation.
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionSimulationInfo {
    pub trace: TransactionTrace,
    pub fee_estimation: FeeEstimate,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum FeeUnit {
    #[serde(rename = "wei")]
    Wei,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_fee_estimate_deser() {
        serde_json::from_str::<FeeEstimate>(include_str!(
            "../../test-data/raw_gateway_responses/estimate_fee/1_success.txt"
        ))
        .unwrap();
    }
}
