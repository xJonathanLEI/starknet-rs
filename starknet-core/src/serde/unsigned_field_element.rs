use alloc::{fmt::Formatter, format};

use serde::{
    de::{Error as DeError, Visitor},
    Deserializer, Serializer,
};
use serde_with::{DeserializeAs, SerializeAs};

use starknet_types_core::felt::Felt;

pub struct UfeHex;

pub struct UfeHexOption;

pub struct UfePendingBlockHash;

struct UfeHexVisitor;
struct UfeHexOptionVisitor;
struct UfePendingBlockHashVisitor;

impl SerializeAs<Felt> for UfeHex {
    fn serialize_as<S>(value: &Felt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }
}

impl<'de> DeserializeAs<'de, Felt> for UfeHex {
    fn deserialize_as<D>(deserializer: D) -> Result<Felt, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UfeHexVisitor)
    }
}

impl<'de> Visitor<'de> for UfeHexVisitor {
    type Value = Felt;

    fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Felt::from_hex(v).map_err(|err| DeError::custom(format!("invalid hex string: {err}")))
    }
}

impl SerializeAs<Option<Felt>> for UfeHexOption {
    fn serialize_as<S>(value: &Option<Felt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&format!("{value:#064x}")),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt>> for UfeHexOption {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UfeHexOptionVisitor)
    }
}

impl<'de> Visitor<'de> for UfeHexOptionVisitor {
    type Value = Option<Felt>;

    fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        match v {
            "" => Ok(None),
            _ => match Felt::from_hex(v) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(DeError::custom(format!("invalid hex string: {err}"))),
            },
        }
    }
}

impl SerializeAs<Option<Felt>> for UfePendingBlockHash {
    fn serialize_as<S>(value: &Option<Felt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&format!("{value:#064x}")),
            // We don't know if it's `null` or `"pending"`
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt>> for UfePendingBlockHash {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UfePendingBlockHashVisitor)
    }
}

impl<'de> Visitor<'de> for UfePendingBlockHashVisitor {
    type Value = Option<Felt>;

    fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        if v.is_empty() || v == "pending" || v == "None" {
            Ok(None)
        } else {
            match Felt::from_hex(v) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(DeError::custom(format!("invalid hex string: {err}"))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Deserialize)]
    struct TestStruct(#[serde_as(as = "UfeHexOption")] pub Option<Felt>);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn empty_string_deser() {
        let r = serde_json::from_str::<TestStruct>("\"\"").unwrap();
        assert_eq!(r.0, None);
    }
}
