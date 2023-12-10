//! Support for string compatibility with Cairo `ByteArray`.
//! https://github.com/starkware-libs/cairo/blob/a4de08fbd75fa1d58c69d054d6b3d99aaf318f90/corelib/src/byte_array.cairo
//!
//! The basic concept of this `ByteArray` is relying on a string being
//! represented as an array of bytes packed by 31 bytes in a felt.
//! To support any string even if the length is not a multiple of 31,
//! the `ByteArray` struct has a `pending_word` field, which is the last
//! word that is always shorter than 31 bytes.
//!
//! In the data structure, everything is represented as a felt to be compatible
//! with the Cairo implementation.
use alloc::string::FromUtf8Error;
use core::str::{self, FromStr};

use starknet_ff::FieldElement;

const MAX_WORD_LEN: usize = 31;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ByteArray {
    pub data: Vec<FieldElement>,
    pub pending_word: FieldElement,
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
    fn internal_from_string(string: &str) -> Self {
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
                // Safe to unwrap as pending word always fit in a felt.
                FieldElement::from_byte_slice_be(&r).unwrap(),
                len,
            )
        } else {
            (FieldElement::ZERO, 0)
        };

        let mut data = vec![];
        for chunk in full_chunks {
            // Safe to unwrap as full chunks are 31 bytes long, always fit in a felt.
            data.push(FieldElement::from_byte_slice_be(chunk).unwrap())
        }

        Self {
            data,
            pending_word,
            pending_word_len,
        }
    }

    /// Converts `ByteArray` instance into a UTF-8 encoded string on success.
    /// Returns error if the `ByteArray` contains an invalid UTF-8 string.
    pub fn to_string(&self) -> Result<String, FromUtf8Error> {
        let mut s = String::new();

        for d in &self.data {
            // Chunks are always 31 bytes long (MAX_WORD_LEN).
            s.push_str(&felt_to_utf8(d, MAX_WORD_LEN)?);
        }

        if self.pending_word_len > 0 {
            s.push_str(&felt_to_utf8(&self.pending_word, self.pending_word_len)?);
        }

        Ok(s)
    }
}

/// Converts a felt into a UTF-8 string.
/// Returns an error if the felt contains an invalid UTF-8 string.
///
/// # Arguments
///
/// * `felt` - The `FieldElement` to convert. In the context of `ByteArray` this
///            felt always contains at most 31 bytes.
/// * `len` - The number of bytes in the felt, at most 31. In the context
///           of `ByteArray`, we don't need to check `len` as the `MAX_WORD_LEN`
///           already protect against that.
fn felt_to_utf8(felt: &FieldElement, len: usize) -> Result<String, FromUtf8Error> {
    let mut buffer = vec![];

    // ByteArray always enforce to have the first byte equal to 0.
    // That's why we start to 1.
    for byte in felt.to_bytes_be()[1 + MAX_WORD_LEN - len..].iter() {
        buffer.push(*byte)
    }

    String::from_utf8(buffer)
}

impl FromStr for ByteArray {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::internal_from_string(s))
    }
}

#[cfg(test)]
mod tests {
    use super::ByteArray;
    use starknet_ff::FieldElement;

    #[test]
    fn test_from_string_empty_string_default() {
        let b = "".parse::<ByteArray>().unwrap();
        assert_eq!(b, ByteArray::default());
    }

    #[test]
    fn test_from_string_only_pending_word() {
        let b = "ABCD".parse::<ByteArray>().unwrap();
        assert_eq!(
            b,
            ByteArray {
                data: vec![],
                pending_word: FieldElement::from_hex_be(
                    "0x0000000000000000000000000000000000000000000000000000000041424344"
                )
                .unwrap(),
                pending_word_len: 4,
            }
        );
    }

    #[test]
    fn test_from_string_max_pending_word_len() {
        // pending word is at most 30 bytes long.
        let b = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234"
            .parse::<ByteArray>()
            .unwrap();
        assert_eq!(
            b,
            ByteArray {
                data: vec![],
                pending_word: FieldElement::from_hex_be(
                    "0x00004142434445464748494a4b4c4d4e4f505152535455565758595a31323334"
                )
                .unwrap(),
                pending_word_len: 30,
            }
        );
    }

    #[test]
    fn test_from_string_data_only() {
        let b = "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345"
            .parse::<ByteArray>()
            .unwrap();
        assert_eq!(
            b,
            ByteArray {
                data: vec![FieldElement::from_hex_be(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                )
                .unwrap()],
                pending_word: FieldElement::ZERO,
                pending_word_len: 0,
            }
        );
    }

    #[test]
    fn test_from_string_data_only_multiple() {
        let b = "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345"
            .parse::<ByteArray>()
            .unwrap();
        assert_eq!(
            b,
            ByteArray {
                data: vec![
                    FieldElement::from_hex_be(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                ],
                pending_word: FieldElement::ZERO,
                pending_word_len: 0,
            }
        );
    }

    #[test]
    fn test_from_string_data_and_pending_word() {
        let b = "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCD"
            .parse::<ByteArray>()
            .unwrap();
        assert_eq!(
            b,
            ByteArray {
                data: vec![
                    FieldElement::from_hex_be(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                    FieldElement::from_hex_be(
                        "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435"
                    )
                    .unwrap(),
                ],
                pending_word: FieldElement::from_hex_be(
                    "0x0000000000000000000000000000000000000000000000000000000041424344"
                )
                .unwrap(),
                pending_word_len: 4,
            }
        );
    }

    #[test]
    fn test_to_string_empty_string_default() {
        let b = ByteArray::default();
        assert_eq!(b.to_string().unwrap(), "");
    }

    #[test]
    fn test_to_string_only_pending_word() {
        let b = ByteArray {
            data: vec![],
            pending_word: FieldElement::from_hex_be(
                "0x0000000000000000000000000000000000000000000000000000000041424344",
            )
            .unwrap(),
            pending_word_len: 4,
        };

        assert_eq!(b.to_string().unwrap(), "ABCD");
    }

    #[test]
    fn test_to_string_max_pending_word_len() {
        let b = ByteArray {
            data: vec![],
            pending_word: FieldElement::from_hex_be(
                "0x00004142434445464748494a4b4c4d4e4f505152535455565758595a31323334",
            )
            .unwrap(),
            pending_word_len: 30,
        };

        assert_eq!(b.to_string().unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234");
    }

    #[test]
    fn test_to_string_data_only() {
        let b = ByteArray {
            data: vec![FieldElement::from_hex_be(
                "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
            )
            .unwrap()],
            pending_word: FieldElement::ZERO,
            pending_word_len: 0,
        };

        assert_eq!(b.to_string().unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345");
    }

    #[test]
    fn test_to_string_data_only_multiple() {
        let b = ByteArray {
            data: vec![
                FieldElement::from_hex_be(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
            ],
            pending_word: FieldElement::ZERO,
            pending_word_len: 0,
        };

        assert_eq!(
            b.to_string().unwrap(),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ12345ABCDEFGHIJKLMNOPQRSTUVWXYZ12345"
        );
    }

    #[test]
    fn test_to_string_data_and_pending_word() {
        let b = ByteArray {
            data: vec![
                FieldElement::from_hex_be(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x004142434445464748494a4b4c4d4e4f505152535455565758595a3132333435",
                )
                .unwrap(),
            ],
            pending_word: FieldElement::from_hex_be(
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
    fn test_to_string_invalid_utf8() {
        let b = ByteArray {
            data: vec![],
            pending_word: FieldElement::from_hex_be(
                "0x00000000000000000000000000000000000000000000000000000000ffffffff",
            )
            .unwrap(),
            pending_word_len: 4,
        };

        b.to_string().unwrap();
    }
}
