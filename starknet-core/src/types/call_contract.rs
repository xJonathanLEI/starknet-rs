use super::{super::serde::unsigned_field_element::UfeHex, FieldElement};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize)]
pub struct InvokeFunction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    pub calldata: Vec<FieldElement>,
    pub signature: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CallContractResult {
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
}
