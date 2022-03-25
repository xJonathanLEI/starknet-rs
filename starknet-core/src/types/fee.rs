use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FeeEstimate {
    pub amount: u64,
    pub unit: FeeUnit,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum FeeUnit {
    #[serde(rename = "wei")]
    Wei,
}
