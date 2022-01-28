use super::{super::serde::unsigned_field_element::hex, UnsignedFieldElement};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InvokeFunction {
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub entry_point_selector: UnsignedFieldElement,
    pub calldata: Vec<UnsignedFieldElement>,
    pub signature: Vec<UnsignedFieldElement>,
}

#[derive(Debug, Deserialize)]
pub struct CallContractResult {
    pub result: Vec<UnsignedFieldElement>,
}
