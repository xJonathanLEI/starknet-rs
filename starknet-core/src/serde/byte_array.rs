pub mod base64 {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<Vec<u8>>,
    {
        serializer.serialize_str(&STANDARD.encode(value.as_ref()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match STANDARD.decode(value) {
            Ok(value) => Ok(value),
            Err(err) => Err(serde::de::Error::custom(format!(
                "invalid base64 string: {err}"
            ))),
        }
    }
}
