use super::{super::serde::unsigned_field_element::UfeHex, FieldElement};

use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct CallContractResult {
    #[serde_as(as = "Vec<UfeHex>")]
    pub result: Vec<FieldElement>,
}
