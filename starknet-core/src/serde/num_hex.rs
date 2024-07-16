pub mod u64 {
    use alloc::{fmt::Formatter, format};

    use serde::{de::Visitor, Deserializer, Serializer};

    struct NumHexVisitor;

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{value:#x}"))
        } else {
            serializer.serialize_u64(*value)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(NumHexVisitor)
        } else {
            deserializer.deserialize_u64(NumHexVisitor)
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

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_literal::hex;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestStruct(#[serde(with = "u64")] pub u64);

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn bin_ser() {
        let r = bincode::serialize(&TestStruct(0x1234)).unwrap();
        assert_eq!(r, hex!("3412000000000000"));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn bin_deser() {
        let r = bincode::deserialize::<TestStruct>(&hex!("3412000000000000")).unwrap();
        assert_eq!(r.0, 0x1234);
    }
}
