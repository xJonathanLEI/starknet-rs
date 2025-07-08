pub(crate) mod u64_hex {
    use serde::{de::Visitor, Deserialize, Serialize};

    struct U64HexVisitor;

    pub fn serialize<S>(v: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{v:#x}"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(U64HexVisitor)
    }

    impl Visitor<'_> for U64HexVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u64::from_str_radix(v.trim_start_matches("0x"), 16)
                .map_err(|err| serde::de::Error::custom(format!("invalid u64 hex string: {err}")))
        }
    }
}

pub(crate) mod u128_hex {
    use serde::{de::Visitor, Deserialize, Serialize};

    struct U128HexVisitor;

    pub fn serialize<S>(v: &u128, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{v:#x}"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(U128HexVisitor)
    }

    impl Visitor<'_> for U128HexVisitor {
        type Value = u128;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u128::from_str_radix(v.trim_start_matches("0x"), 16)
                .map_err(|err| serde::de::Error::custom(format!("invalid u128 hex string: {err}")))
        }
    }
}

pub(crate) mod u64_hex_opt {
    use serde::{de::Visitor, Deserialize, Serialize};

    struct U64HexOptVisitor;

    pub fn serialize<S>(v: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match v {
            Some(v) => serializer.serialize_str(&format!("{v:#x}")),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(U64HexOptVisitor)
    }

    impl Visitor<'_> for U64HexOptVisitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "null or string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(
                u64::from_str_radix(v.trim_start_matches("0x"), 16).map_err(|err| {
                    serde::de::Error::custom(format!("invalid u64 hex string: {err}"))
                })?,
            ))
        }
    }
}
