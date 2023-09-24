//! CairoType implementation for `Vec`.
//! They are used for Array and Span cairo types.
use crate::abi::cairo_types::{CairoType, Error, Result};
use starknet_core::types::FieldElement;

impl<T, RT> CairoType for Vec<T>
where
    T: CairoType<RustType = RT>,
{
    type RustType = Vec<RT>;

    const SERIALIZED_SIZE: Option<usize> = None;

    #[inline]
    fn serialized_size(rust: &Self::RustType) -> usize {
        let data = rust;
        // 1 + because the length is always the first felt.
        1 + data.iter().map(T::serialized_size).sum::<usize>()
    }

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out: Vec<FieldElement> = vec![rust.len().into()];
        rust.iter().for_each(|r| out.extend(T::serialize(r)));
        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        let len: usize = usize::from_str_radix(format!("{:x}", felts[offset]).as_str(), 16)
            .map_err(|_| {
                Error::Deserialize("First felt of an array must fit into usize".to_string())
            })?;

        let mut out: Vec<RT> = vec![];
        let mut offset = offset + 1;

        loop {
            if out.len() == len {
                break;
            }

            let rust: RT = T::deserialize(felts, offset)?;
            offset += T::serialized_size(&rust);
            out.push(rust);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_array() {
        let v: Vec<u32> = vec![1, 2, 3];
        let felts = Vec::<u32>::serialize(&v);
        assert_eq!(felts.len(), 4);
        assert_eq!(felts[0], FieldElement::from(3_u32));
        assert_eq!(felts[1], FieldElement::ONE);
        assert_eq!(felts[2], FieldElement::TWO);
        assert_eq!(felts[3], FieldElement::THREE);
    }

    #[test]
    fn test_deserialize_array() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::from(2_u32),
            FieldElement::from(123_u32),
            FieldElement::from(9988_u32),
        ];

        let vals = Vec::<u32>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], 123_u32);
        assert_eq!(vals[1], 9988_u32);
    }

    #[test]
    fn test_serialize_array_nested() {
        let v: Vec<Vec<u32>> = vec![vec![1, 2], vec![3]];
        let felts = Vec::<Vec<u32>>::serialize(&v);
        assert_eq!(felts.len(), 6);
        assert_eq!(felts[0], FieldElement::TWO);
        assert_eq!(felts[1], FieldElement::TWO);
        assert_eq!(felts[2], FieldElement::ONE);
        assert_eq!(felts[3], FieldElement::TWO);
        assert_eq!(felts[4], FieldElement::ONE);
        assert_eq!(felts[5], FieldElement::THREE);
    }

    #[test]
    fn test_deserialize_array_nested() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::TWO,
            FieldElement::TWO,
            FieldElement::ONE,
            FieldElement::TWO,
            FieldElement::ONE,
            FieldElement::THREE,
        ];

        let vals = Vec::<Vec<u32>>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 2);
        assert_eq!(vals[0], vec![1, 2]);
        assert_eq!(vals[1], vec![3]);
    }

    #[test]
    fn test_serialize_array_tuple() {
        let v: Vec<(u32, FieldElement)> = vec![(12, FieldElement::TWO)];
        let felts = Vec::<(u32, FieldElement)>::serialize(&v);
        assert_eq!(felts.len(), 3);
        assert_eq!(felts[0], FieldElement::from(1_u32));
        assert_eq!(felts[1], FieldElement::from(12_u32));
        assert_eq!(felts[2], FieldElement::TWO);
    }

    #[test]
    fn test_deserialize_array_tuple() {
        let felts: Vec<FieldElement> = vec![
            FieldElement::from(1_u32),
            FieldElement::from(12_u32),
            FieldElement::TWO,
        ];

        let vals = Vec::<(u32, FieldElement)>::deserialize(&felts, 0).unwrap();
        assert_eq!(vals.len(), 1);
        assert_eq!(vals[0], (12, FieldElement::TWO));
    }
}
