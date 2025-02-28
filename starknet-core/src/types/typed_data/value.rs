use alloc::{borrow::ToOwned, string::*, vec::*};

use indexmap::IndexMap;
use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[cfg(feature = "std")]
type RandomState = std::hash::RandomState;
#[cfg(not(feature = "std"))]
type RandomState = foldhash::fast::RandomState;

const DEFAULT_INDEXMAP_CAPACITY: usize = 5;

/// The primitive representation of the SNIP-12 message value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// String value.
    String(String),
    /// Unsigned integer value.
    UnsignedInteger(u128),
    /// Signed integer value.
    SignedInteger(i128),
    /// Boolean value.
    Boolean(bool),
    /// Map value.
    Object(ObjectValue),
    /// Sequence value.
    Array(ArrayValue),
}

/// A map/object value for SNIP-12 message representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectValue {
    /// Fields of the object.
    pub fields: IndexMap<String, Value, RandomState>,
}

/// A sequence/array value for SNIP-12 message representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayValue {
    /// Elements of the array.
    pub elements: Vec<Value>,
}

/// The unit enum for identifying [`Value`] variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    /// String value.
    String,
    /// Unsigned integer value.
    UnsignedInteger,
    /// Signed integer value.
    SignedInteger,
    /// Boolean value.
    Boolean,
    /// Map value.
    Object,
    /// Sequence value.
    Array,
}

impl Value {
    /// Gets the type of value.
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::String(_) => ValueKind::String,
            Self::UnsignedInteger(_) => ValueKind::UnsignedInteger,
            Self::SignedInteger(_) => ValueKind::SignedInteger,
            Self::Boolean(_) => ValueKind::Boolean,
            Self::Object(_) => ValueKind::Object,
            Self::Array(_) => ValueKind::Array,
        }
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "integer, string, map or sequence")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Boolean(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::UnsignedInteger(v.into()))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::UnsignedInteger(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::SignedInteger(v.into()))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::SignedInteger(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(v.to_owned()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut elements = Vec::new();
        while let Some(element) = seq.next_element()? {
            elements.push(element);
        }
        Ok(Value::Array(ArrayValue { elements }))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut fields =
            IndexMap::with_capacity_and_hasher(DEFAULT_INDEXMAP_CAPACITY, Default::default());
        while let Some((key, value)) = map.next_entry()? {
            fields.insert(key, value);
        }
        Ok(Value::Object(ObjectValue { fields }))
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

impl core::fmt::Display for ValueKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::UnsignedInteger => write!(f, "unsigned_integer"),
            Self::SignedInteger => write!(f, "signed_integer"),
            Self::Boolean => write!(f, "boolean"),
            Self::Object => write!(f, "object"),
            Self::Array => write!(f, "array"),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(s) => serializer.serialize_str(s),
            Value::UnsignedInteger(u) => serializer.serialize_u128(*u),
            Value::SignedInteger(i) => serializer.serialize_i128(*i),
            Value::Boolean(b) => serializer.serialize_bool(*b),
            Value::Object(o) => o.serialize(serializer),
            Value::Array(a) => a.serialize(serializer),
        }
    }
}

impl Serialize for ObjectValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.fields.serialize(serializer)
    }
}

impl Serialize for ArrayValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.elements.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod serialize {
        use super::*;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        fn should_return_serialized_value() {
            // Given
            let initial_value = r###"{
  "Name": "some name",
  "Some Array": [1, 2, 3, 4],
  "Some Object": {
    "Some Selector": "transfer",
    "Some Contract Address": "0x0123"
  }
}"###;
            let value = serde_json::from_str::<Value>(initial_value).unwrap();

            // When
            let result = serde_json::to_string(&value).unwrap();

            // Then
            let expected_json: serde_json::Value = serde_json::from_str(initial_value).unwrap();
            let result_json: serde_json::Value = serde_json::from_str(&result).unwrap();
            assert_eq!(result_json, expected_json);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_value_deser() {
        let raw = r###"{
  "Name": "some name",
  "Some Array": [1, 2, 3, 4],
  "Some Object": {
    "Some Selector": "transfer",
    "Some Contract Address": "0x0123"
  }
}"###;

        let value = serde_json::from_str::<Value>(raw).unwrap();

        match value {
            Value::Object(value) => {
                assert_eq!(value.fields.len(), 3);
                assert_eq!(
                    value.fields.get("Name").unwrap(),
                    &Value::String("some name".into())
                );
                assert_eq!(
                    value.fields.get("Some Array").unwrap(),
                    &Value::Array(ArrayValue {
                        elements: vec![
                            Value::UnsignedInteger(1),
                            Value::UnsignedInteger(2),
                            Value::UnsignedInteger(3),
                            Value::UnsignedInteger(4),
                        ]
                    })
                );
                assert_eq!(
                    value.fields.get("Some Object").unwrap(),
                    &Value::Object(ObjectValue {
                        fields: [
                            (
                                String::from("Some Selector"),
                                Value::String("transfer".into())
                            ),
                            (
                                String::from("Some Contract Address"),
                                Value::String("0x0123".into())
                            ),
                        ]
                        .into_iter()
                        .collect()
                    })
                );
            }
            _ => panic!("unexpected value type"),
        }
    }
}
