use super::{AbiType, AbiTypeAny, GENTY_FROZEN};

#[derive(Debug, PartialEq, Clone)]
pub struct AbiArray {
    pub cairo_type: String,
    pub genty: String,
    pub inner: Box<AbiTypeAny>,
}

impl AbiArray {
    pub fn new(cairo_type: &str, inner: AbiTypeAny) -> Self {
        AbiArray {
            cairo_type: cairo_type.to_string(),
            genty: String::new(),
            inner: Box::new(inner),
        }
    }
}

impl AbiType for AbiArray {
    fn get_genty(&self) -> String {
        self.genty.clone()
    }

    fn compare_generic(&mut self, other: &AbiTypeAny) {
        match other {
            AbiTypeAny::Array(_) => {
                if self.genty != GENTY_FROZEN {
                    self.genty = other.get_genty();
                }
            }
            _ => {
                self.inner.compare_generic(other);
            }
        };
    }

    fn apply_generic(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        // Check if the whole array is the generic.
        for (cairo_type, genty) in &cairo_types_gentys {
            if &self.get_cairo_type_full() == cairo_type {
                self.genty = genty.to_string();
                return (genty.to_string(), true);
            }
        }

        let (gen_str, is_generic) = self.inner.apply_generic(cairo_types_gentys);
        (
            format!("{}::<{}>", self.cairo_type.clone(), &gen_str),
            is_generic,
        )
    }

    fn get_cairo_type_full(&self) -> String {
        format!(
            "{}::<{}>",
            self.cairo_type.clone(),
            &self.inner.get_cairo_type_full()
        )
    }

    fn get_cairo_type_name(&self) -> String {
        self.cairo_type
            .split("::")
            .last()
            .unwrap_or(&self.cairo_type)
            .to_string()
    }

    fn to_rust_type(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            format!("Vec<{}>", &self.inner.to_rust_type())
        }
    }

    fn to_rust_type_path(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            format!("Vec::<{}>", &self.inner.to_rust_type())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_types::{AbiBasic, AbiTypeAny};

    fn get_default() -> AbiArray {
        AbiArray::new(
            "core::array::Array",
            AbiTypeAny::Basic(AbiBasic::new("core::felt252".into())),
        )
    }

    #[test]
    fn get_cairo_type_full() {
        let t = get_default();
        assert_eq!(
            t.get_cairo_type_full(),
            "core::array::Array::<core::felt252>"
        );
    }

    #[test]
    fn cairo_type_name_only() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_name(), "Array");
    }

    #[test]
    fn to_rust_type() {
        let t = get_default();
        assert_eq!(t.to_rust_type(), "Vec<starknet::core::types::FieldElement>");
    }

    #[test]
    fn to_rust_type_path() {
        let t = get_default();
        assert_eq!(
            t.to_rust_type_path(),
            "Vec::<starknet::core::types::FieldElement>"
        );
    }

    #[test]
    fn from_string() {
        let t = AbiTypeAny::from_string("core::array::Array::<core::felt252>");
        assert_eq!(t, AbiTypeAny::Array(get_default()));
    }

    #[test]
    fn from_string_array_tuple() {
        let t =
            AbiTypeAny::from_string("core::array::Array::<(core::felt252, core::integer::u32)>");
        assert_eq!(
            t,
            AbiTypeAny::Array(AbiArray::new(
                "core::array::Array",
                AbiTypeAny::Tuple(
                    vec![
                        AbiTypeAny::Basic(AbiBasic::new("core::felt252".into())),
                        AbiTypeAny::Basic(AbiBasic::new("core::integer::u32".into())),
                    ]
                    .into()
                )
            ))
        );
    }

    #[test]
    fn generic_array() {
        let mut t = AbiTypeAny::from_string("core::array::Array::<core::felt252>");
        assert_eq!(
            t.apply_generic(vec![("core::array::Array::<core::felt252>", "A")]),
            ("A".to_string(), true)
        );
    }

    #[test]
    fn generic_inner() {
        let mut t = AbiTypeAny::from_string("core::array::Array::<core::felt252>");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("core::array::Array::<A>".to_string(), true)
        );
    }

    #[test]
    fn generic_not() {
        let mut t = AbiTypeAny::from_string("core::array::Array::<core::u32>");
        assert_eq!(
            t.apply_generic(vec![("core::array::Array<core::felt252>", "A")]),
            ("core::array::Array::<core::u32>".to_string(), false)
        );
    }

    #[test]
    fn generic_not_inner() {
        let mut t = AbiTypeAny::from_string("core::array::Array::<core::u32>");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("core::array::Array::<core::u32>".to_string(), false)
        );
    }
}
