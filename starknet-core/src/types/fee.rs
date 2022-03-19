use super::super::serde::u64::fee_amount::deserialize;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FeeEstimate {
    // Don't know why but it won't just work without this. Possibly related to:
    //   https://github.com/serde-rs/serde/issues/1183
    #[serde(deserialize_with = "deserialize")]
    pub amount: u64,
    pub unit: FeeUnit,
}

#[derive(Debug, Deserialize)]
pub enum FeeUnit {
    #[serde(rename = "wei")]
    Wei,
}
