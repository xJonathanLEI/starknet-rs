use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct FeeEstimate {
    pub amount: u64,
    pub unit: FeeUnit,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum FeeUnit {
    #[serde(rename = "wei")]
    Wei,
}
