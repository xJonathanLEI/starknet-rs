use super::{super::serde::unsigned_field_element::UfeHex, UnsignedFieldElement};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize)]
pub struct InvokeFunction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: UnsignedFieldElement,
    pub calldata: Vec<UnsignedFieldElement>,
    pub signature: Vec<UnsignedFieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CallContractResult {
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<UnsignedFieldElement>,
}
