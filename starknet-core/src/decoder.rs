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

pub fn decode_param(param: &ParamType, data: &[FieldElement], offset: usize) -> DecodeResult {
    match *param {
        ParamType::FieldElement => DecodeResult {
            token: Token::FieldElement(data[offset]),
            new_offset: offset + 1,
        },
        ParamType::Array => {
            // TODO: don't use unwrap
            let size: usize = u32::try_from(data[offset]).unwrap() as usize;
            DecodeResult {
                token: Token::Array(data[(offset + 1)..(offset + size + 1)].to_vec()),
                new_offset: offset + size + 1,
            }
        }
        ParamType::Tuple(size) => DecodeResult {
            token: Token::Tuple(data[offset..(offset + size)].to_vec()),
            new_offset: offset + size,
        },
    }
}

pub fn decode(types: &[ParamType], data: &[FieldElement]) -> Vec<Token> {
    let mut tokens = vec![];
    let mut offset = 0;

    for param in types {
        let res = decode_param(param, data, offset);
        offset = res.new_offset;
        tokens.push(res.token);
    }

    tokens
}

mod test {
    use starknet_crypto::FieldElement;

    use super::{decode, decode_param, DecodeResult, ParamType, Token};

    #[test]
    fn decode_param_field_element() {
        let result = decode_param(&ParamType::FieldElement, &[FieldElement::ONE], 0);
        let expected_result = DecodeResult {
            token: Token::FieldElement(FieldElement::ONE),
            new_offset: 1,
        };
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn decode_param_field_element_empty_data() {
        decode_param(&ParamType::FieldElement, &[], 0);
    }

    #[test]
    fn decode_param_array() {
        let result = decode_param(
            &ParamType::Array,
            &[FieldElement::TWO, FieldElement::ONE, FieldElement::THREE],
            0,
        );
        let expected_result = DecodeResult {
            token: Token::Array(vec![FieldElement::ONE, FieldElement::THREE]),
            new_offset: 3,
        };
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn decode_param_array_empty_data() {
        decode_param(&ParamType::Array, &[], 0);
    }

    #[test]
    #[should_panic(expected = "range end index 3 out of range for slice of length 2")]
    fn decode_param_array_insufficient_data() {
        decode_param(
            &ParamType::Array,
            &[FieldElement::TWO, FieldElement::THREE],
            0,
        );
    }

    #[test]
    #[should_panic(expected = "ValueOutOfRange")]
    fn decode_param_array_invalid_size() {
        let result = decode_param(
            &ParamType::Array,
            &[FieldElement::MAX, FieldElement::ONE, FieldElement::THREE],
            0,
        );
        let expected_result = DecodeResult {
            token: Token::Array(vec![FieldElement::ONE, FieldElement::THREE]),
            new_offset: 3,
        };
        assert_eq!(result, expected_result)
    }

    #[test]
    fn decode_param_tuple() {
        let result = decode_param(
            &ParamType::Tuple(2),
            &[FieldElement::TWO, FieldElement::THREE],
            0,
        );
        let expected_result = DecodeResult {
            token: Token::Tuple(vec![FieldElement::TWO, FieldElement::THREE]),
            new_offset: 2,
        };
        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "range end index 2 out of range for slice of length 0")]
    fn decode_param_tuple_empty_data() {
        decode_param(&ParamType::Tuple(2), &[], 0);
    }

    #[test]
    #[should_panic(expected = "range end index 4 out of range for slice of length 3")]
    fn decode_param_tuple_insufficient_data() {
        decode_param(
            &ParamType::Tuple(3),
            &[FieldElement::TWO, FieldElement::THREE, FieldElement::ONE],
            1,
        );
    }

    #[test]
    fn decode_data() {
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

        let result = decode(&types, &data);

        assert_eq!(result, expected_result)
    }

    #[test]
    fn decode_data_exceeds_types() {
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

        let result = decode(&types, &data);

        assert_eq!(result, expected_result)
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 6 but the index is 6")]
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

        let expected_result = vec![
            Token::FieldElement(FieldElement::ONE),
            Token::Array(vec![FieldElement::ONE, FieldElement::THREE]),
            Token::Tuple(vec![FieldElement::TWO, FieldElement::THREE]),
        ];

        let result = decode(&types, &data);

        assert_eq!(result, expected_result)
    }
}
