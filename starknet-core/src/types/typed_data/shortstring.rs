//! Module for handling `shortstring` serialization/desesrialization.
//!
//! Technically this module shouldn't exist, or at least should be straightforward, as a very simple
//! Cairo short string encoding/decoding step would suffice. Unfortunately, starknet.js ships a bug:
//!
//! <https://github.com/starknet-io/starknet.js/issues/1039>
//!
//! Since starknet.js is widely used, it's essentially the de facto spec. We must reimplement the
//! bug here by conditionally encoding as Cairo short string only when the source string is not a
//! valid integer or decimal/hexadecimal repr.

use serde::de::Visitor;

use crate::{types::Felt, utils::cairo_short_string_to_felt};

struct ShortStringVisitor;

impl Visitor<'_> for ShortStringVisitor {
    type Value = Felt;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "string or integer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // This is to reimplement the `starknet.js` bug
        let decoded_as_raw = match v.strip_prefix("0x") {
            Some(hexadecimal) => {
                if hexadecimal.chars().all(|c| c.is_ascii_hexdigit()) {
                    Felt::from_hex(v).ok()
                } else {
                    None
                }
            }
            None => {
                if v.chars().all(|c| c.is_ascii_digit()) {
                    Felt::from_dec_str(v).ok()
                } else {
                    None
                }
            }
        };

        match decoded_as_raw {
            Some(raw) => Ok(raw),
            None => cairo_short_string_to_felt(v).map_err(|_| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &"valid Cairo short string",
                )
            }),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Felt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(ShortStringVisitor)
}
