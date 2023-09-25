//! CairoType implementation for bool.
use crate::abi::cairo_types::{CairoType, Result};
use starknet_core::types::FieldElement;

impl CairoType for bool {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        vec![FieldElement::from(*rust as u32)]
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        if felts[offset] == FieldElement::ONE {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_bool() {
        let v = true;
        let felts = bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ONE);

        let v = false;
        let felts = bool::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ZERO);
    }

    #[test]
    fn test_deserialize_bool() {
        let felts = vec![FieldElement::ZERO, FieldElement::ONE, FieldElement::TWO];
        assert!(!bool::deserialize(&felts, 0).unwrap());
        assert!(bool::deserialize(&felts, 1).unwrap());
        assert!(!bool::deserialize(&felts, 2).unwrap());
    }
}
