pub mod base64 {
    use alloc::{fmt::Formatter, format, vec::Vec};

    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde::{de::Visitor, Deserializer, Serializer};

    struct Base64Visitor;

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
        deserializer.deserialize_any(Base64Visitor)
    }

    impl<'de> Visitor<'de> for Base64Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut Formatter) -> alloc::fmt::Result {
            write!(formatter, "string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            STANDARD
                .decode(v)
                .map_err(|err| serde::de::Error::custom(format!("invalid base64 string: {err}")))
        }
    }
}
