//! CairoType implementation for Result.
//!
//! https://github.com/starkware-libs/cairo/blob/main/corelib/src/result.cairo#L6
use crate::abi::cairo_types::{CairoType, Error as CairoError, Result as CairoResult};
use starknet_core::types::FieldElement;

impl<T, RT, E, RE> CairoType for Result<T, E>
where
    T: CairoType<RustType = RT>,
    E: CairoType<RustType = RE>,
{
    type RustType = Result<RT, RE>;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        let mut out = vec![];

        match rust {
            Result::Ok(r) => {
                out.push(FieldElement::ZERO);
                out.extend(T::serialize(r));
            }
            Result::Err(e) => {
                out.push(FieldElement::ONE);
                out.extend(E::serialize(e));
            }
        };

        out
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> CairoResult<Self::RustType> {
        let idx = felts[offset];

        if idx == FieldElement::ZERO {
            // + 1 as the offset value is the index of the enum.
            CairoResult::Ok(Ok(T::deserialize(felts, offset + 1)?))
        } else if idx == FieldElement::ONE {
            CairoResult::Ok(Err(E::deserialize(felts, offset + 1)?))
        } else {
            Err(CairoError::Deserialize(
                "Result is expected 0 or 1 index only".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet_core::types::FieldElement;

    #[test]
    fn test_result_ok_serialize() {
        let r = Ok(u32::MAX);
        let felts = Result::<u32, FieldElement>::serialize(&r);
        assert_eq!(felts.len(), 2);
        assert_eq!(felts[0], FieldElement::ZERO);
        assert_eq!(felts[1], FieldElement::from(u32::MAX));
    }

    #[test]
    fn test_result_ok_deserialize() {
        let felts = vec![FieldElement::ZERO, FieldElement::from(u32::MAX)];
        let r = Result::<u32, FieldElement>::deserialize(&felts, 0).unwrap();
        assert_eq!(r, Ok(u32::MAX));
    }

    #[test]
    fn test_result_ok_unit_serialize() {
        let r = Ok(());
        let felts = Result::<(), FieldElement>::serialize(&r);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::ZERO);
    }

    #[test]
    fn test_result_ok_unit_deserialize() {
        let felts = vec![FieldElement::ZERO];
        let r = Result::<(), FieldElement>::deserialize(&felts, 0).unwrap();
        assert_eq!(r, Ok(()));
    }

    #[test]
    fn test_result_err_serialize() {
        let r = Err(FieldElement::ONE);
        let felts = Result::<FieldElement, FieldElement>::serialize(&r);
        assert_eq!(felts.len(), 2);
        assert_eq!(felts[0], FieldElement::ONE);
        assert_eq!(felts[1], FieldElement::ONE);
    }

    #[test]
    fn test_result_err_deserialize() {
        let felts = vec![FieldElement::ONE, FieldElement::ONE];
        let r = Result::<FieldElement, FieldElement>::deserialize(&felts, 0).unwrap();
        assert_eq!(r, Err(FieldElement::ONE));
    }
}
