use alloc::{string::*, vec::*};

use num_traits::ToPrimitive;

use crate::{
    codec::{Decode, Encode, Error as CodecError, FeltWriter},
    types::Felt,
};

const BYTES_PER_SLOT: usize = 31;
const FELT_BYTE_SIZE: usize = 32;

/// The `ByteArray` type in Cairo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteArray(Vec<u8>);

impl ByteArray {
    /// Returns the number of bytes in the array, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the array contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&str> for ByteArray {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl From<ByteArray> for Vec<u8> {
    fn from(value: ByteArray) -> Self {
        value.0
    }
}

impl TryFrom<ByteArray> for String {
    type Error = FromUtf8Error;

    fn try_from(value: ByteArray) -> Result<Self, Self::Error> {
        Self::from_utf8(value.0)
    }
}

impl Encode for ByteArray {
    fn encode<W: FeltWriter>(&self, writer: &mut W) -> Result<(), CodecError> {
        writer.write((self.len() / BYTES_PER_SLOT).into());
        let mut chunks_iter = self.0.chunks_exact(BYTES_PER_SLOT);
        for full_slot in chunks_iter.by_ref() {
            writer.write(Felt::from_bytes_be_slice(full_slot));
        }

        let last_chunk = chunks_iter.remainder();
        writer.write(Felt::from_bytes_be_slice(last_chunk));
        writer.write(last_chunk.len().into());

        Ok(())
    }
}

impl<'a> Decode<'a> for ByteArray {
    fn decode_iter<T>(iter: &mut T) -> Result<Self, CodecError>
    where
        T: Iterator<Item = &'a Felt>,
    {
        let length = iter.next().ok_or_else(CodecError::input_exhausted)?;
        let length = length
            .to_usize()
            .ok_or_else(|| CodecError::value_out_of_range(length, "usize"))?;

        let mut result = Vec::<u8>::with_capacity(length * BYTES_PER_SLOT + BYTES_PER_SLOT - 1);

        for _ in 0..length {
            let full_slot = iter.next().ok_or_else(CodecError::input_exhausted)?;
            result.extend_from_slice(&full_slot.to_bytes_be()[1..]);
        }

        let pending_word = iter.next().ok_or_else(CodecError::input_exhausted)?;
        let pending_word_len = iter.next().ok_or_else(CodecError::input_exhausted)?;
        let pending_word_len = pending_word_len
            .to_usize()
            .ok_or_else(|| CodecError::value_out_of_range(pending_word_len, "usize"))?;
        result
            .extend_from_slice(&pending_word.to_bytes_be()[(FELT_BYTE_SIZE - pending_word_len)..]);

        Ok(Self(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_string_encode_decode_roundtrip() {
        let test_data = [
            (
                "",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                ],
            ),
            (
                "abc",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000616263",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    ),
                ],
            ),
            (
                "000000000011111111112222222222",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000303030303030303030303131313131313131313132323232323232323232",
                    ),
                    Felt::from_hex_unchecked(
                        "0x000000000000000000000000000000000000000000000000000000000000001e",
                    ),
                ],
            ),
            (
                "0000000000111111111122222222223",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0030303030303030303030313131313131313131313232323232323232323233",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                ],
            ),
            (
                "00000000001111111111222222222233",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0030303030303030303030313131313131313131313232323232323232323233",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000033",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ),
                ],
            ),
            (
                "00000000001111111111222222222233333333334444444444\
                 55555555556666666666777777777788888888889999999999",
                vec![
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0030303030303030303030313131313131313131313232323232323232323233",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0033333333333333333334343434343434343434353535353535353535353636",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0036363636363636363737373737373737373738383838383838383838393939",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000039393939393939",
                    ),
                    Felt::from_hex_unchecked(
                        "0x0000000000000000000000000000000000000000000000000000000000000007",
                    ),
                ],
            ),
        ];

        for (string, expected) in test_data {
            let bytes: ByteArray = string.into();

            let mut encoded = vec![];
            bytes.encode(&mut encoded).unwrap();
            assert_eq!(encoded, expected);

            let decoded = ByteArray::decode(&expected).unwrap();
            assert_eq!(String::try_from(decoded).unwrap(), string);
        }
    }
}
