pub mod fee_amount {
    use serde::{de::Error as DeError, Deserialize, Deserializer};
    use serde_json::Number;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Number::deserialize(deserializer)?;
        value
            .as_u64()
            .ok_or_else(|| DeError::custom(format!("number of out range: {}", value)))
    }
}
