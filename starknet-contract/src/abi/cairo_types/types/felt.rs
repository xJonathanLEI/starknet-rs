use crate::abi::cairo_types::{CairoType, Result};
use starknet_core::types::FieldElement;

impl CairoType for () {
    type RustType = Self;

    fn serialize(_rust: &Self::RustType) -> Vec<FieldElement> {
        vec![]
    }

    fn deserialize(_felts: &[FieldElement], _offset: usize) -> Result<Self::RustType> {
        Ok(())
    }
}

impl CairoType for FieldElement {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![*rust]
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(felts[offset])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_field_element() {
        let f = FieldElement::ZERO;
        let felts = FieldElement::serialize(&f);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ZERO);
    }

    #[test]
    fn test_deserialize_field_element() {
        let felts = vec![FieldElement::ZERO, FieldElement::ONE, FieldElement::TWO];
        assert_eq!(
            FieldElement::deserialize(&felts, 0).unwrap(),
            FieldElement::ZERO
        );
        assert_eq!(
            FieldElement::deserialize(&felts, 1).unwrap(),
            FieldElement::ONE
        );
        assert_eq!(
            FieldElement::deserialize(&felts, 2).unwrap(),
            FieldElement::TWO
        );
    }
}
