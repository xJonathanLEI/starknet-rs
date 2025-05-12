use serde::{de::Visitor, Deserialize, Serialize};

/// Revision of SNIP-12.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Revision {
    /// The legacy, deprecated revision of SNIP-12.
    V0,
    /// The current active revision of SNIP-12.
    V1,
}

impl core::fmt::Display for Revision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::V0 => write!(f, "0"),
            Self::V1 => write!(f, "1"),
        }
    }
}

struct RevisionVisitor;

impl Visitor<'_> for RevisionVisitor {
    type Value = Revision;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "string or integer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "0" => Ok(Revision::V0),
            "1" => Ok(Revision::V1),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"\"0\" or \"1\"",
            )),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(Revision::V0),
            1 => Ok(Revision::V1),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(v),
                &"0 or 1",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Revision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RevisionVisitor)
    }
}

impl Serialize for Revision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Revision::V0 => "0",
            Revision::V1 => "1",
        })
    }
}


