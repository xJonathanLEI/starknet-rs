use super::{AbiType, AbiTypeAny, GENTY_FROZEN};

#[derive(Debug, PartialEq, Clone)]
pub struct AbiTuple {
    pub inners: Vec<AbiTypeAny>,
    pub genty: String,
}

impl AbiTuple {
    pub fn new(inners: Vec<AbiTypeAny>) -> Self {
        AbiTuple {
            inners,
            genty: String::new(),
        }
    }
}

impl From<Vec<AbiTypeAny>> for AbiTuple {
    fn from(v: Vec<AbiTypeAny>) -> Self {
        Self::new(v)
    }
}

impl AbiType for AbiTuple {
    fn get_genty(&self) -> String {
        self.genty.clone()
    }

    fn compare_generic(&mut self, other: &AbiTypeAny) {
        match other {
            AbiTypeAny::Tuple(_) => {
                if self.genty != GENTY_FROZEN {
                    self.genty = other.get_genty();
                }
            }
            _ => {
                for inner in &mut self.inners {
                    inner.compare_generic(other);
                }
            }
        };
    }

    fn apply_generic(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        // Check if the whole tuple is the generic.
        for (cairo_type, genty) in &cairo_types_gentys {
            if &self.get_cairo_type_full() == cairo_type {
                self.genty = genty.to_string();
                return (genty.to_string(), true);
            }
        }

        let mut tuple_has_generic = false;
        let mut s = "(".to_string();
        let arr_len = self.inners.len();

        for (idx, inner) in self.inners.iter_mut().enumerate() {
            let (type_str, is_generic) = inner.apply_generic(cairo_types_gentys.clone());

            if is_generic && !tuple_has_generic {
                tuple_has_generic = true;
            }

            s.push_str(&type_str);

            if idx < arr_len - 1 {
                s.push_str(", ");
            }
        }
        s.push(')');

        (s, tuple_has_generic)
    }

    fn get_cairo_type_full(&self) -> String {
        let mut s = "(".to_string();
        for (idx, inner) in self.inners.iter().enumerate() {
            s.push_str(&inner.get_cairo_type_full());

            if idx < self.inners.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push(')');
        s
    }

    fn get_cairo_type_name(&self) -> String {
        "|tuple|".to_string()
    }

    fn to_rust_type(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            let mut s = "(".to_string();
            for (idx, inner) in self.inners.iter().enumerate() {
                s.push_str(&inner.to_rust_type());

                if idx < self.inners.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push(')');
            s
        }
    }

    fn to_rust_type_path(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            let mut s = "(".to_string();
            for (idx, inner) in self.inners.iter().enumerate() {
                s.push_str(&inner.to_rust_type_path());

                if idx < self.inners.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push(')');
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::parser::abi_types::{AbiArray, AbiTypeAny};

    fn get_default() -> AbiTuple {
        AbiTuple::new(vec![
            AbiTypeAny::Basic("core::felt252".into()),
            AbiTypeAny::Basic("core::integer::u32".into()),
        ])
    }

    #[test]
    fn get_cairo_type_full() {
        let t = get_default();
        assert_eq!(
            t.get_cairo_type_full(),
            "(core::felt252, core::integer::u32)"
        );
    }

    #[test]
    fn cairo_type_name_only() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_name(), "|tuple|");
    }

    #[test]
    fn to_rust_type() {
        let t = get_default();
        assert_eq!(
            t.to_rust_type(),
            "(starknet::core::types::FieldElement, u32)"
        );
    }

    #[test]
    fn to_rust_type_path() {
        let t = get_default();
        assert_eq!(
            t.to_rust_type_path(),
            "(starknet::core::types::FieldElement, u32)"
        );
    }

    #[test]
    fn from_string() {
        let t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(t, AbiTypeAny::Tuple(get_default()));
    }

    #[test]
    fn from_string_tuple_of_array() {
        let t =
            AbiTypeAny::from_string("(core::array::Array::<core::felt252>, core::integer::u32)");
        assert_eq!(
            t,
            AbiTypeAny::Tuple(
                vec![
                    AbiTypeAny::Array(AbiArray::new(
                        "core::array::Array",
                        AbiTypeAny::Basic("core::felt252".into())
                    )),
                    AbiTypeAny::Basic("core::integer::u32".into()),
                ]
                .into()
            )
        );
    }

    #[test]
    fn generic_tuple() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("(core::felt252, core::integer::u32)", "A")]),
            ("A".to_string(), true)
        );
    }

    #[test]
    fn generic_inner() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("(A, core::integer::u32)".to_string(), true)
        );
    }

    #[test]
    fn generic_inner_2() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("core::integer::u32", "A")]),
            ("(core::felt252, A)".to_string(), true)
        );
    }

    #[test]
    fn generic_tuple_not() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("(core::u32, core::u256)", "A")]),
            ("(core::felt252, core::integer::u32)".to_string(), false)
        );
    }

    #[test]
    fn generic_inner_not() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("core::u256", "A")]),
            ("(core::felt252, core::integer::u32)".to_string(), false)
        );
    }

    #[test]
    fn generic_inner_multiple() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A"), ("core::integer::u32", "B")]),
            ("(A, B)".to_string(), true)
        );
    }

    #[test]
    fn generic_inner_multiple_2() {
        let mut t = AbiTypeAny::from_string("(core::felt252, core::integer::u32)");
        assert_eq!(
            t.apply_generic(vec![("core::array", "A"), ("core::integer::u32", "B")]),
            ("(core::felt252, B)".to_string(), true)
        );
    }
}
