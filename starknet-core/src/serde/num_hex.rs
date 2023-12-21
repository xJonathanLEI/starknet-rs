pub mod u64 {
    use alloc::{fmt::Formatter, format};
    use core::mem;

    use serde::{de::Visitor, Deserializer, Serializer};

    struct NumHexVisitor;

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{value:#x}"))
        } else {
            serializer.serialize_bytes(&value.to_be_bytes())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(NumHexVisitor)
        } else {
            deserializer.deserialize_bytes(NumHexVisitor)
        }
    }

    impl<'de> Visitor<'de> for NumHexVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
            write!(formatter, "string, or an array of u8")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u64::from_str_radix(v.trim_start_matches("0x"), 16)
                .map_err(|err| serde::de::Error::custom(format!("invalid u64 hex string: {err}")))
        }

        fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
            <[u8; mem::size_of::<u64>()]>::try_from(v)
                .map(u64::from_be_bytes)
                .map_err(serde::de::Error::custom)
        }
    }
}
