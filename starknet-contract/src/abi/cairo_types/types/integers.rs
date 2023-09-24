//! CairoType implementation for integers (signed/unsigned).
use crate::abi::cairo_types::{CairoType, Result};
use starknet_core::types::FieldElement;

macro_rules! implement_trait_for_unsigned {
    ($type:ty) => {
        impl CairoType for $type {
            type RustType = Self;

            fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
                vec![FieldElement::from(*rust)]
            }

            fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
                let temp: u128 = felts[offset].try_into().unwrap();
                Ok(temp as $type)
            }
        }
    };
}

macro_rules! implement_trait_for_signed {
    ($type:ty) => {
        impl CairoType for $type {
            type RustType = Self;

            fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
                vec![FieldElement::from(*rust as usize)]
            }

            fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
                let temp: u128 = felts[offset].try_into().unwrap();
                Ok(temp as $type)
            }
        }
    };
}

implement_trait_for_unsigned!(u8);
implement_trait_for_unsigned!(u16);
implement_trait_for_unsigned!(u32);
implement_trait_for_unsigned!(u64);
implement_trait_for_unsigned!(u128);
implement_trait_for_unsigned!(usize);

implement_trait_for_signed!(i8);
implement_trait_for_signed!(i16);
implement_trait_for_signed!(i32);
implement_trait_for_signed!(i64);
implement_trait_for_signed!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_u8() {
        let v = 12_u8;
        let felts = u8::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(12 as u8));
    }

    #[test]
    fn test_deserialize_u8() {
        let felts = vec![FieldElement::from(12_u8), FieldElement::from(10_u8)];
        assert_eq!(u8::deserialize(&felts, 0).unwrap(), 12);
        assert_eq!(u8::deserialize(&felts, 1).unwrap(), 10);
    }

    #[test]
    fn test_serialize_u16() {
        let v = 12_u16;
        let felts = u16::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(12 as u16));
    }

    #[test]
    fn test_deserialize_u16() {
        let felts = vec![FieldElement::from(12_u16), FieldElement::from(10_u8)];
        assert_eq!(u16::deserialize(&felts, 0).unwrap(), 12);
        assert_eq!(u16::deserialize(&felts, 1).unwrap(), 10);
    }

    #[test]
    fn test_serialize_u32() {
        let v = 123_u32;
        let felts = u32::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u32));
    }

    #[test]
    fn test_deserialize_u32() {
        let felts = vec![FieldElement::from(123_u32), FieldElement::from(99_u32)];
        assert_eq!(u32::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u32::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_u64() {
        let v = 123_u64;
        let felts = u64::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u64));
    }

    #[test]
    fn test_deserialize_u64() {
        let felts = vec![FieldElement::from(123_u64), FieldElement::from(99_u64)];
        assert_eq!(u64::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u64::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_u128() {
        let v = 123_u128;
        let felts = u128::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u128));
    }

    #[test]
    fn test_deserialize_u128() {
        let felts = vec![FieldElement::from(123_u128), FieldElement::from(99_u128)];
        assert_eq!(u128::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(u128::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_usize() {
        let v = 123;
        let felts = usize::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(123 as u128));
    }

    #[test]
    fn test_deserialize_usize() {
        let felts = vec![FieldElement::from(123_u128), FieldElement::from(99_u64)];
        assert_eq!(usize::deserialize(&felts, 0).unwrap(), 123);
        assert_eq!(usize::deserialize(&felts, 1).unwrap(), 99);
    }

    #[test]
    fn test_serialize_i8() {
        let v = i8::MAX;
        let felts = i8::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(i8::MAX as u8));
    }

    #[test]
    fn test_deserialize_i8() {
        let felts = vec![
            FieldElement::from(i8::MAX as u8),
            FieldElement::from(i8::MAX as u8),
        ];
        assert_eq!(i8::deserialize(&felts, 0).unwrap(), i8::MAX);
        assert_eq!(i8::deserialize(&felts, 1).unwrap(), i8::MAX);
    }

    #[test]
    fn test_serialize_i16() {
        let v = i16::MAX;
        let felts = i16::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(i16::MAX as u16));
    }

    #[test]
    fn test_deserialize_i16() {
        let felts = vec![
            FieldElement::from(i16::MAX as u16),
            FieldElement::from(i16::MAX as u16),
        ];
        assert_eq!(i16::deserialize(&felts, 0).unwrap(), i16::MAX);
        assert_eq!(i16::deserialize(&felts, 1).unwrap(), i16::MAX);
    }

    #[test]
    fn test_serialize_i32() {
        let v = i32::MAX;
        let felts = i32::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(i32::MAX as u32));
    }

    #[test]
    fn test_deserialize_i32() {
        let felts = vec![
            FieldElement::from(i32::MAX as u32),
            FieldElement::from(i32::MAX as u32),
        ];
        assert_eq!(i32::deserialize(&felts, 0).unwrap(), i32::MAX);
        assert_eq!(i32::deserialize(&felts, 1).unwrap(), i32::MAX);
    }

    #[test]
    fn test_serialize_i64() {
        let v = i64::MAX;
        let felts = i64::serialize(&v);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(i64::MAX as u64));
    }

    #[test]
    fn test_deserialize_i64() {
        let felts = vec![
            FieldElement::from(i64::MAX as u64),
            FieldElement::from(i64::MAX as u64),
        ];
        assert_eq!(i64::deserialize(&felts, 0).unwrap(), i64::MAX);
        assert_eq!(i64::deserialize(&felts, 1).unwrap(), i64::MAX);
    }

    #[test]
    fn test_deserialize_i128() {
        let felts = vec![
            FieldElement::from(i128::MAX as u128),
            FieldElement::from(i128::MAX as u128),
        ];
        assert_eq!(i128::deserialize(&felts, 0).unwrap(), i128::MAX);
        assert_eq!(i128::deserialize(&felts, 1).unwrap(), i128::MAX);
    }
}
