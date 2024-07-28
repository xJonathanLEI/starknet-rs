use alloc::{fmt::Formatter, format};

use crypto_bigint::U256;
use serde::{
    de::{Error as DeError, Visitor},
    Deserializer, Serializer,
};
use serde_with::{DeserializeAs, SerializeAs};

use starknet_types_core::felt::Felt;

const PRIME: U256 =
    U256::from_be_hex("0800000000000011000000000000000000000000000000000000000000000001");

/// Serialize/deserialize [`Felt`] as hex strings. For use with `serde_with`.
#[derive(Debug)]
pub struct UfeHex;

/// Serialize/deserialize [`Option<Felt>`] as hex strings. For use with `serde_with`.
#[derive(Debug)]
pub struct UfeHexOption;

/// Serialize/deserialize [`Option<Felt>`] as hex strings in a pending block hash context. For use
/// with `serde_with`.
#[derive(Debug)]
pub struct UfePendingBlockHash;

struct UfeHexVisitor;
struct UfeHexOptionVisitor;
struct UfePendingBlockHashVisitor;

impl SerializeAs<Felt> for UfeHex {
    fn serialize_as<S>(value: &Felt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{value:#x}"))
        } else {
            serializer.serialize_bytes(&value.to_bytes_be())
        }
    }
}

impl<'de> DeserializeAs<'de, Felt> for UfeHex {
    fn deserialize_as<D>(deserializer: D) -> Result<Felt, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(UfeHexVisitor)
        } else {
            deserializer.deserialize_bytes(UfeHexVisitor)
        }
    }
}

impl<'de> Visitor<'de> for UfeHexVisitor {
    type Value = Felt;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> alloc::fmt::Result {
        write!(formatter, "a hex string, or an array of u8")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Felt::from_hex(v).map_err(|err| DeError::custom(format!("invalid hex string: {err}")))
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        let buf = <[u8; 32]>::try_from(v).map_err(serde::de::Error::custom)?;

        if U256::from_be_slice(&buf) < PRIME {
            Ok(Felt::from_bytes_be(&buf))
        } else {
            Err(serde::de::Error::custom("field element value out of range"))
        }
    }
}

impl SerializeAs<Option<Felt>> for UfeHexOption {
    fn serialize_as<S>(value: &Option<Felt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match value {
                Some(value) => serializer.serialize_str(&format!("{value:#064x}")),
                None => serializer.serialize_none(),
            }
        } else {
            match value {
                Some(value) => serializer.serialize_bytes(&value.to_bytes_be()),
                None => serializer.serialize_bytes(&[]),
            }
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt>> for UfeHexOption {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(UfeHexOptionVisitor)
        } else {
            deserializer.deserialize_bytes(UfeHexOptionVisitor)
        }
    }
}

impl<'de> Visitor<'de> for UfeHexOptionVisitor {
    type Value = Option<Felt>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> alloc::fmt::Result {
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

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        if v.is_empty() {
            return Ok(None);
        }

        let buf = <[u8; 32]>::try_from(v).map_err(serde::de::Error::custom)?;

        if U256::from_be_slice(&buf) < PRIME {
            Ok(Some(Felt::from_bytes_be(&buf)))
        } else {
            Err(serde::de::Error::custom("field element value out of range"))
        }
    }
}

impl SerializeAs<Option<Felt>> for UfePendingBlockHash {
    fn serialize_as<S>(value: &Option<Felt>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match value {
                Some(value) => serializer.serialize_str(&format!("{value:#064x}")),
                // We don't know if it's `null` or `"pending"`
                None => serializer.serialize_none(),
            }
        } else {
            match value {
                Some(value) => serializer.serialize_bytes(&value.to_bytes_be()),
                None => serializer.serialize_bytes(&[]),
            }
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt>> for UfePendingBlockHash {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(UfePendingBlockHashVisitor)
        } else {
            deserializer.deserialize_bytes(UfePendingBlockHashVisitor)
        }
    }
}

impl<'de> Visitor<'de> for UfePendingBlockHashVisitor {
    type Value = Option<Felt>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> alloc::fmt::Result {
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

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        if v.is_empty() {
            return Ok(None);
        }

        let buf = <[u8; 32]>::try_from(v).map_err(serde::de::Error::custom)?;

        if U256::from_be_slice(&buf) < PRIME {
            Ok(Some(Felt::from_bytes_be(&buf)))
        } else {
            Err(serde::de::Error::custom("field element value out of range"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_literal::hex;
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct TestStruct(#[serde_as(as = "UfeHex")] pub Felt);

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct TestOptionStruct(#[serde_as(as = "UfeHexOption")] pub Option<Felt>);

    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct TestBlockHashStruct(#[serde_as(as = "UfePendingBlockHash")] pub Option<Felt>);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn bin_ser() {
        let r = bincode::serialize(&TestStruct(Felt::ONE)).unwrap();
        assert_eq!(
            r,
            hex!(
                "2000000000000000 0000000000000000000000000000000000000000000000000000000000000001"
            )
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn bin_deser() {
        let r = bincode::deserialize::<TestStruct>(&hex!(
            "2000000000000000 0000000000000000000000000000000000000000000000000000000000000001"
        ))
        .unwrap();
        assert_eq!(r.0, Felt::ONE);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn bin_deser_out_of_range() {
        if bincode::deserialize::<TestStruct>(&hex!(
            "2000000000000000 0800000000000011000000000000000000000000000000000000000000000001"
        ))
        .is_ok()
        {
            panic!("deserialization should fail")
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn option_deser_empty_string() {
        let r = serde_json::from_str::<TestOptionStruct>("\"\"").unwrap();
        assert_eq!(r.0, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn option_bin_ser_none() {
        let r = bincode::serialize(&TestOptionStruct(None)).unwrap();
        assert_eq!(r, hex!("0000000000000000"));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn option_bin_deser_none() {
        let r = bincode::deserialize::<TestOptionStruct>(&hex!("0000000000000000")).unwrap();
        assert_eq!(r.0, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn pending_block_hash_deser_pending() {
        let r = serde_json::from_str::<TestBlockHashStruct>("\"pending\"").unwrap();
        assert_eq!(r.0, None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn pending_block_hash_bin_ser_none() {
        let r = bincode::serialize(&TestBlockHashStruct(None)).unwrap();
        assert_eq!(r, hex!("0000000000000000"));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn pending_block_hash_bin_deser_none() {
        let r = bincode::deserialize::<TestBlockHashStruct>(&hex!("0000000000000000")).unwrap();
        assert_eq!(r.0, None);
    }
}
