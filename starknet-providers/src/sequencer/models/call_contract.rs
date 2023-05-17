use serde::Deserialize;
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CallContractResult {
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
}
