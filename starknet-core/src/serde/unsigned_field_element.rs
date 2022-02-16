use serde::{de::Error as DeError, Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

use crate::types::UnsignedFieldElement;

pub struct UfeHex;

pub struct UfeHexOption;

pub struct UfePendingBlockHash;

impl SerializeAs<UnsignedFieldElement> for UfeHex {
    fn serialize_as<S>(value: &UnsignedFieldElement, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:#064x}", value))
    }
}

impl<'de> DeserializeAs<'de, UnsignedFieldElement> for UfeHex {
    fn deserialize_as<D>(deserializer: D) -> Result<UnsignedFieldElement, D::Error>
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

impl SerializeAs<Option<UnsignedFieldElement>> for UfeHexOption {
    fn serialize_as<S>(
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
}

impl<'de> DeserializeAs<'de, Option<UnsignedFieldElement>> for UfeHexOption {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<UnsignedFieldElement>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "" => Ok(None),
            _ => match UnsignedFieldElement::from_hex_str(&value) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(DeError::custom(format!("invalid hex string: {}", err))),
            },
        }
    }
}

impl SerializeAs<Option<UnsignedFieldElement>> for UfePendingBlockHash {
    fn serialize_as<S>(
        value: &Option<UnsignedFieldElement>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&format!("{:#064x}", value)),
            // We don't know if it's `null` or `"pending"`
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<UnsignedFieldElement>> for UfePendingBlockHash {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<UnsignedFieldElement>, D::Error>
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_with::serde_as;

    #[serde_as]
    #[derive(Deserialize)]
    struct TestStruct(#[serde_as(as = "UfeHexOption")] pub Option<UnsignedFieldElement>);

    #[test]
    fn empty_string_deser() {
        let r = serde_json::from_str::<TestStruct>("\"\"").unwrap();
        assert_eq!(r.0, None);
    }
}
