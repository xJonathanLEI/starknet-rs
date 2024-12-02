//! Support for [`String`] compatibility with Cairo `ByteArray`.
//! <https://github.com/starkware-libs/cairo/blob/0b86ece404b0922b76caca5d07a94ed41407f174/corelib/src/byte_array.cairo>.
//!
//! The basic concept of this `ByteArray` is relying on a string being
//! represented as an array of bytes packed by 31 bytes ([`Bytes31`]) in a [`Felt`].
//! To support any string even if the length is not a multiple of 31,
//! the `ByteArray` struct has a `pending_word` field, which is the last
//! word that is always shorter than 31 bytes.
use alloc::{
    str::{self},
    string::{FromUtf8Error, String},
    vec::Vec,
};

use crate::types::{Bytes31, Felt};

const MAX_WORD_LEN: usize = 31;

/// A struct representing a Cairo `ByteArray`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ByteArray {
    /// An array of full "words" of 31 bytes each.
    /// The first byte of each word in the byte array is the most significant byte in the word.
    pub data: Vec<Bytes31>,
    /// A `felt252` that actually represents a `bytes31`, with less than 31 bytes.
    /// It is represented as a `felt252` to improve performance of building the byte array.
    /// The first byte is the most significant byte among the `pending_word_len` bytes in the word.
    pub pending_word: Bytes31,
    /// The number of bytes in `pending_word`.
    /// Its value should be in the range [0, 30].
    pub pending_word_len: usize,
}

impl ByteArray {
    /// Converts a `String` into a `ByteArray`.
    /// The rust type `String` implies UTF-8 encoding,
    /// event if this function is not directly bound to this encoding.
    ///
    /// # Arguments
    ///
    /// * `string` - The always valid UTF-8 string to convert.
    fn from_string(string: &str) -> Self {
        let bytes = string.as_bytes();
        let chunks: Vec<_> = bytes.chunks(MAX_WORD_LEN).collect();

        let remainder = if bytes.len() % MAX_WORD_LEN != 0 {
            chunks.last().copied().map(|last| last.to_vec())
        } else {
            None
        };

        let full_chunks = if remainder.is_some() {
            &chunks[..chunks.len() - 1]
        } else {
            &chunks[..]
        };

        let (pending_word, pending_word_len) = if let Some(r) = remainder {
            let len = r.len();
            (
                // Safe to unwrap here as slices are at most 31 bytes long.
                Bytes31::try_from(Felt::from_bytes_be_slice(&r)).unwrap(),
                len,
            )
        } else {
            (Bytes31::try_from(Felt::ZERO).unwrap(), 0)
        };

        let mut data = Vec::new();
        for chunk in full_chunks {
            // Safe to unwrap here as slices are at most 31 bytes long.
            data.push(Bytes31::try_from(Felt::from_bytes_be_slice(chunk)).unwrap())
        }

        Self {
            data,
            pending_word,
            pending_word_len,
        }
    }

    /// Converts [`ByteArray`] instance into an UTF-8 encoded string on success.
    /// Returns error if the [`ByteArray`] contains an invalid UTF-8 string.
    fn to_string(&self) -> Result<String, FromUtf8Error> {
        let mut s = String::new();

        for d in &self.data {
            // Chunks are always 31 bytes long (MAX_WORD_LEN).
            s.push_str(&d.to_string(MAX_WORD_LEN)?);
        }

        if self.pending_word_len > 0 {
            s.push_str(&self.pending_word.to_string(self.pending_word_len)?);
        }

        Ok(s)
    }
}

impl TryFrom<ByteArray> for String {
    type Error = FromUtf8Error;

    fn try_from(value: ByteArray) -> Result<Self, Self::Error> {
        value.to_string()
    }
}

impl From<String> for ByteArray {
    fn from(value: String) -> Self {
        Self::from_string(&value)
    }
}

impl From<&str> for ByteArray {
    fn from(value: &str) -> Self {
        Self::from_string(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteArray, Bytes31, Felt};

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_empty() {
        let b = ByteArray::from_string("");
        assert_eq!(b, ByteArray::default());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_only_pending_word() {
        let b = ByteArray::from_string("ABCD");
        assert_eq!(
            b,
            ByteArray {
                data: vec![],
                pending_word: Bytes31::from_hex("0x41424344").unwrap(),
                pending_word_len: 4,
            }
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_max_pending_word_len() {
        // pending word is at most 30 bytes long.
        let b = ByteArray::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234");

        assert_eq!(
            b,
            ByteArray {
                data: vec![],
                pending_word: Bytes31::from_hex(
                    "0x00004142434445464748494a4b4c4d4e4f505152535455565758595a31323334"
                )
                .unwrap(),
                pending_word_len: 30,
            }
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_data_only() {
        let b = ByteArray::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ12345");

        assert_eq!(
            b,
            ByteArray {
                data: vec![Bytes31::from_hex(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                )
                .unwrap()],
                pending_word: Felt::ZERO.try_into().unwrap(),
                pending_word_len: 0,
            }
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_data_only_multiple_values() {
        let b = ByteArray::from_string(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345",
        );

        assert_eq!(
            b,
            ByteArray {
                data: vec![
                    Bytes31::from_hex(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                    Bytes31::from_hex(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                ],
                pending_word: Felt::ZERO.try_into().unwrap(),
                pending_word_len: 0,
            }
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_from_string_data_and_pending_word() {
        let b = ByteArray::from_string(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCD",
        );

        assert_eq!(
            b,
            ByteArray {
                data: vec![
                    Bytes31::from_hex(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                    Bytes31::from_hex(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                ],
                pending_word: Bytes31::from_hex(
                    "0x0000000000000000000000000000000000000000000000000000000041424344"
                )
                .unwrap(),
                pending_word_len: 4,
            }
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_empty() {
        let b = ByteArray::default();
        assert_eq!(b.to_string().unwrap(), "");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_only_pending_word() {
        let b = ByteArray {
            data: vec![],
            pending_word: Bytes31::from_hex(
                "0x0000000000000000000000000000000000000000000000000000000041424344",
            )
            .unwrap(),
            pending_word_len: 4,
        };

        assert_eq!(b.to_string().unwrap(), "ABCD");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_max_pending_word_len() {
        let b = ByteArray {
            data: vec![],
            pending_word: Bytes31::from_hex(
                "0x00004142434445464748494a4b4c4d4e4f505152535455565758595a31323334",
            )
            .unwrap(),
            pending_word_len: 30,
        };

        assert_eq!(b.to_string().unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_data_only() {
        let b = ByteArray {
            data: vec![Bytes31::from_hex(
                "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
            )
            .unwrap()],
            pending_word: Felt::ZERO.try_into().unwrap(),
            pending_word_len: 0,
        };

        assert_eq!(b.to_string().unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_data_only_multiple_values() {
        let b = ByteArray {
            data: vec![
                Bytes31::from_hex(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
                Bytes31::from_hex(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
            ],
            pending_word: Felt::ZERO.try_into().unwrap(),
            pending_word_len: 0,
        };

        assert_eq!(
            b.to_string().unwrap(),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_byte_array_to_string_data_and_pending_word() {
        let b = ByteArray {
            data: vec![
                Bytes31::from_hex(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
                Bytes31::from_hex(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
            ],
            pending_word: Bytes31::from_hex(
                "0x0000000000000000000000000000000000000000000000000000000041424344",
            )
            .unwrap(),
            pending_word_len: 4,
        };

        assert_eq!(
            b.to_string().unwrap(),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCD"
        );
    }

    #[test]
    #[should_panic]
    fn test_byte_array_to_string_invalid_utf8() {
        let invalid = Felt::from_bytes_be_slice(b"\xF0\x90\x80");

        let b = ByteArray {
            data: vec![],
            pending_word: invalid.try_into().unwrap(),
            pending_word_len: 4,
        };

        b.to_string().unwrap();
    }

    #[test]
    fn test_from_utf8() {
        let b: ByteArray = "🦀🌟".into();

        assert_eq!(
            b,
            ByteArray {
                data: vec![],
                pending_word: Bytes31::from_hex(
                    "0x000000000000000000000000000000000000000000000000f09fa680f09f8c9f",
                )
                .unwrap(),
                pending_word_len: 8,
            }
        );
    }
}
