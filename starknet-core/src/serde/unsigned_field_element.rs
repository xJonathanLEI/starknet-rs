pub mod hex {
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};

    use crate::types::UnsignedFieldElement;

    pub fn serialize<S>(value: &UnsignedFieldElement, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:#064x}", value))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<UnsignedFieldElement, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match UnsignedFieldElement::from_hex_str(&value) {
            Ok(value) => Ok(value),
            Err(err) => Err(DeError::custom(format!("invalid hex string: {}", err))),
        }
    }
}

pub mod hex_option {
    use serde::{Deserializer, Serializer};

    use crate::types::UnsignedFieldElement;

    pub fn serialize<S>(
        value: &Option<UnsignedFieldElement>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&format!("{:#064x}", value)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<UnsignedFieldElement>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(super::hex::deserialize(deserializer)?))
    }
}

pub mod hex_slice {
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};

    use crate::types::UnsignedFieldElement;

    pub fn serialize<S>(value: &[UnsignedFieldElement], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(value.iter().map(|item| format!("{:#064x}", item)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<UnsignedFieldElement>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values: Vec<String> = Vec::deserialize(deserializer)?;

        values
            .iter()
            .map(|value| UnsignedFieldElement::from_hex_str(value))
            .collect::<Result<Vec<UnsignedFieldElement>, _>>()
            .map_err(|err| DeError::custom(format!("invalid hex string: {}", err)))
    }
}

pub mod pending_block_hash {
    use serde::{de::Error as DeError, Deserialize, Deserializer};

    use crate::types::UnsignedFieldElement;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<UnsignedFieldElement>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value == "pending" {
            Ok(None)
        } else {
            match UnsignedFieldElement::from_hex_str(&value) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(DeError::custom(format!("invalid hex string: {}", err))),
            }
        }
    }
}
