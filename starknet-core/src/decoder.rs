use crate::types::FieldElement;

pub enum ParamType {
    FieldElement,
    Array,
    Tuple(usize),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    FieldElement(FieldElement),
    Array(Vec<FieldElement>),
    Tuple(Vec<FieldElement>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct DecodeResult {
    token: Token,
    new_offset: usize,
}

mod decoder_error {

    #[derive(Debug, PartialEq)]
    pub enum DecoderError {
        InvalidLength,
        ValueOutOfRange,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for DecoderError {}

    impl core::fmt::Display for DecoderError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidLength => write!(f, "invalid length"),
                Self::ValueOutOfRange => write!(f, "number out of range"),
            }
        }
    }
}
pub use decoder_error::DecoderError;
use starknet_ff::ValueOutOfRangeError;

fn validate_data_length(
    data: &[FieldElement],
    offset: usize,
    len: usize,
) -> Result<(), DecoderError> {
    if offset + len > data.len() {
        Err(DecoderError::InvalidLength)
    } else {
        Ok(())
    }
}

fn decode_param(
    param: &ParamType,
    data: &[FieldElement],
    offset: usize,
    validate: bool,
) -> Result<DecodeResult, DecoderError> {
    match *param {
        ParamType::FieldElement => {
            if validate {
                validate_data_length(data, offset, 1)?;
            }

            Ok(DecodeResult {
                token: Token::FieldElement(data[offset]),
                new_offset: offset + 1,
            })
        }
        ParamType::Array => {
            if validate {
                validate_data_length(data, offset, 1)?;
            }

            let size: usize =
                u32::try_from(data[offset]).map_err(|_| DecoderError::ValueOutOfRange)? as usize;

            if validate {
                validate_data_length(data, offset, size + 1)?;
            }

            Ok(DecodeResult {
                token: Token::Array(data[(offset + 1)..(offset + size + 1)].to_vec()),
                new_offset: offset + size + 1,
            })
        }
        ParamType::Tuple(size) => {
            if validate {
                validate_data_length(data, offset, size)?;
            }

            Ok(DecodeResult {
                token: Token::Tuple(data[offset..(offset + size)].to_vec()),
                new_offset: offset + size,
            })
        }
    }
}

fn decode_impl(
    types: &[ParamType],
    data: &[FieldElement],
    offset: usize,
    validate: bool,
) -> Result<Vec<Token>, DecoderError> {
    let mut tokens = vec![];
    let mut offset = offset;

    for param in types {
        let res = decode_param(param, data, offset, validate)?;
        offset = res.new_offset;
        tokens.push(res.token);
    }

    Ok(tokens)
}

pub fn decode(types: &[ParamType], data: &[FieldElement]) -> Result<Vec<Token>, DecoderError> {
    decode_impl(types, data, 0, true)
}

impl TryFrom<&Token> for u32 {
    // TODO: add an error type to represent invalid token types like Array and Tuple !
    type Error = ValueOutOfRangeError;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::FieldElement(felt) => u32::try_from(*felt),
            _ => Err(ValueOutOfRangeError),
        }
    }
}

#[derive(Debug)]
pub struct Address(String);

impl TryFrom<&Token> for Address {
    // TODO: add an error type to represent invalid token types like Array and Tuple !
    type Error = ValueOutOfRangeError;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::FieldElement(felt) => Ok(Address(format!("{:#064x}", felt))),
            _ => Err(ValueOutOfRangeError),
        }
    }
}

impl TryFrom<&Token> for String {
    // TODO: add an error type to represent invalid token types like Array and Tuple !
    type Error = ValueOutOfRangeError;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::FieldElement(felt) => String::try_from(*felt),
            _ => Err(ValueOutOfRangeError),
        }
    }
}

impl TryFrom<Token> for Vec<u32> {
    type Error = ValueOutOfRangeError;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Array(v) => v.iter().map(|felt| u32::try_from(*felt)).collect(),
            _ => Err(ValueOutOfRangeError),
        }
    }
}

pub trait Decode {
    fn decode(tokens: &[Token]) -> Self;
}

#[cfg(test)]
mod test {
    use starknet_crypto::FieldElement;

    use super::{decode, decode_impl, decode_param, DecodeResult, DecoderError, ParamType, Token};

    #[test]
    fn decode_param_field_element() -> Result<(), DecoderError> {
        let result = decode_param(&ParamType::FieldElement, &[FieldElement::ONE], 0, true)?;
        let expected_result = DecodeResult {
            token: Token::FieldElement(FieldElement::ONE),
            new_offset: 1,
        };
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn decode_param_field_element_empty_data() {
        let result = decode_param(&ParamType::FieldElement, &[], 0, true);
        let expected_result = Err(DecoderError::InvalidLength);
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn decode_param_field_element_empty_data_no_validate() {
        decode_param(&ParamType::FieldElement, &[], 0, false).unwrap();
    }

    #[test]
    fn decode_param_array() -> Result<(), DecoderError> {
        let result = decode_param(
            &ParamType::Array,
            &[FieldElement::TWO, FieldElement::ONE, FieldElement::THREE],
            0,
            true,
        )?;
        let expected_result = DecodeResult {
            token: Token::Array(vec![FieldElement::ONE, FieldElement::THREE]),
            new_offset: 3,
        };
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn decode_param_array_empty_data() {
        let result = decode_param(&ParamType::Array, &[], 0, true);
        let expected_result = Err(DecoderError::InvalidLength);
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn decode_param_array_empty_data_no_validate() {
        decode_param(&ParamType::Array, &[], 0, false).unwrap();
    }

    #[test]
    fn decode_param_array_insufficient_data() {
        let result = decode_param(
            &ParamType::Array,
            &[FieldElement::TWO, FieldElement::THREE],
            0,
            true,
        );

        let expected_result = Err(DecoderError::InvalidLength);
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "range end index 3 out of range for slice of length 2")]
    fn decode_param_array_insufficient_data_no_validate() {
        decode_param(
            &ParamType::Array,
            &[FieldElement::TWO, FieldElement::THREE],
            0,
            false,
        )
        .unwrap();
    }

    #[test]
    fn decode_param_array_invalid_size() {
        let result = decode_param(
            &ParamType::Array,
            &[FieldElement::MAX, FieldElement::ONE, FieldElement::THREE],
            0,
            true,
        );
        let expected_result = Err(DecoderError::ValueOutOfRange);
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "ValueOutOfRange")]
    fn decode_param_array_invalid_size_no_validate() {
        decode_param(
            &ParamType::Array,
            &[FieldElement::MAX, FieldElement::ONE, FieldElement::THREE],
            0,
            true,
        )
        .unwrap();
    }

    #[test]
    fn decode_param_tuple() -> Result<(), DecoderError> {
        let result = decode_param(
            &ParamType::Tuple(2),
            &[FieldElement::TWO, FieldElement::THREE],
            0,
            true,
        )?;
        let expected_result = DecodeResult {
            token: Token::Tuple(vec![FieldElement::TWO, FieldElement::THREE]),
            new_offset: 2,
        };
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn decode_param_tuple_empty_data() {
        let result = decode_param(&ParamType::Tuple(2), &[], 0, true);
        let expected_result = Err(DecoderError::InvalidLength);
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "range end index 2 out of range for slice of length 0")]
    fn decode_param_tuple_empty_data_no_validate() {
        decode_param(&ParamType::Tuple(2), &[], 0, false).unwrap();
    }

    #[test]
    fn decode_param_tuple_insufficient_data() {
        let result = decode_param(
            &ParamType::Tuple(3),
            &[FieldElement::TWO, FieldElement::THREE, FieldElement::ONE],
            1,
            true,
        );
        let expected_result = Err(DecoderError::InvalidLength);
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "range end index 4 out of range for slice of length 3")]
    fn decode_param_tuple_insufficient_data_no_validate() {
        decode_param(
            &ParamType::Tuple(3),
            &[FieldElement::TWO, FieldElement::THREE, FieldElement::ONE],
            1,
            false,
        )
        .unwrap();
    }

    #[test]
    fn decode_data() -> Result<(), DecoderError> {
        let types = [
            ParamType::FieldElement,
            ParamType::Array,
            ParamType::Tuple(2),
        ];

        let data = [
            FieldElement::ONE,   // field element
            FieldElement::TWO,   // array length
            FieldElement::ONE,   // first element of the array
            FieldElement::THREE, // second element of the array
            FieldElement::TWO,   // first element of the tuple
            FieldElement::THREE, // second element of the tuple
        ];

        let expected_result = vec![
            Token::FieldElement(FieldElement::ONE),
            Token::Array(vec![FieldElement::ONE, FieldElement::THREE]),
            Token::Tuple(vec![FieldElement::TWO, FieldElement::THREE]),
        ];

        let result = decode(&types, &data)?;

        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn decode_data_exceeds_types() -> Result<(), DecoderError> {
        let types = [ParamType::FieldElement, ParamType::Tuple(3)];

        let data = [
            FieldElement::ONE,   // field element
            FieldElement::TWO,   // first element of the tuple
            FieldElement::ONE,   // second element of the tuple
            FieldElement::THREE, // third element of the tuple
            FieldElement::TWO,   // exceeded
            FieldElement::THREE, // exceeded
        ];

        let expected_result = vec![
            Token::FieldElement(FieldElement::ONE),
            Token::Tuple(vec![
                FieldElement::TWO,
                FieldElement::ONE,
                FieldElement::THREE,
            ]),
        ];

        let result = decode(&types, &data)?;

        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn decode_missing_data() {
        let types = [
            ParamType::FieldElement,
            ParamType::Array,
            ParamType::Tuple(2),
            ParamType::FieldElement,
        ];

        let data = [
            FieldElement::ONE,   // field element
            FieldElement::TWO,   // array length
            FieldElement::ONE,   // first element of the array
            FieldElement::THREE, // second element of the array
            FieldElement::TWO,   // first element of the tuple
            FieldElement::THREE, // second element of the tuple
                                 // missing last field element
        ];

        let result = decode(&types, &data);
        let expected_result = Err(DecoderError::InvalidLength);

        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 6 but the index is 6")]
    fn decode_missing_data_no_validate() {
        let types = [
            ParamType::FieldElement,
            ParamType::Array,
            ParamType::Tuple(2),
            ParamType::FieldElement,
        ];

        let data = [
            FieldElement::ONE,   // field element
            FieldElement::TWO,   // array length
            FieldElement::ONE,   // first element of the array
            FieldElement::THREE, // second element of the array
            FieldElement::TWO,   // first element of the tuple
            FieldElement::THREE, // second element of the tuple
                                 // missing last field element
        ];

        decode_impl(&types, &data, 0, false).unwrap();
    }
}
