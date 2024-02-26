pub mod u64 {
    use alloc::{fmt::Formatter, format};

    use serde::{de::Visitor, Deserializer, Serializer};

    struct NumHexVisitor;

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumHexVisitor)
    }

    impl<'de> Visitor<'de> for NumHexVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
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
