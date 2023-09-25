use super::{AbiType, AbiTypeAny, GENTY_FROZEN};

#[derive(Debug, PartialEq, Clone)]
pub struct AbiGeneric {
    pub cairo_type: String,
    pub genty: String,
    pub inners: Vec<AbiTypeAny>,
}

impl AbiGeneric {
    /// Initializes a new instance.
    pub fn new(cairo_type: &str, inners: Vec<AbiTypeAny>) -> Self {
        AbiGeneric {
            cairo_type: cairo_type.to_string(),
            genty: String::new(),
            inners,
        }
    }

    /// Gets the definition of the type with it's generic types.
    pub fn get_rust_generic_def(&self, suffix: &str) -> String {
        let gentys = self.get_gentys_only();
        format!(
            "{}<{}{}>",
            self.get_cairo_type_name(),
            gentys.join(", "),
            suffix
        )
    }

    /// Returns only the generic types list.
    pub fn get_gentys_only(&self) -> Vec<String> {
        // Starts to 'A'.
        let ascii: u8 = 65;

        let mut gentys = vec![];
        for (i, _) in self.inners.iter().enumerate() {
            gentys.push(((ascii + i as u8) as char).to_string());
        }

        gentys
    }

    /// Returns the list of tuple, containing the (cairo_type, generic_type)
    /// for each generic type.
    pub fn get_cairo_types_gentys(&self) -> Vec<(String, String)> {
        // Starts to 'A'.
        let ascii: u8 = 65;

        let mut cairo_types_gentys = vec![];
        for (i, inner) in self.inners.iter().enumerate() {
            let genty = ((ascii + i as u8) as char).to_string();
            cairo_types_gentys.push((inner.get_cairo_type_full(), genty));
        }

        cairo_types_gentys
    }
}

impl AbiType for AbiGeneric {
    fn get_genty(&self) -> String {
        self.genty.clone()
    }

    fn compare_generic(&mut self, other: &AbiTypeAny) {
        match other {
            AbiTypeAny::Generic(_) => {
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
        // Check if the whole struct is the generic.
        for (cairo_type, genty) in &cairo_types_gentys {
            if &self.get_cairo_type_full() == cairo_type {
                self.genty = genty.to_string();
                return (genty.to_string(), true);
            }
        }

        let mut struct_has_generic = false;
        let mut s = format!("{}::<", self.cairo_type);
        let arr_len = self.inners.len();

        for (idx, inner) in self.inners.iter_mut().enumerate() {
            let (type_str, is_generic) = inner.apply_generic(cairo_types_gentys.clone());

            if is_generic && !struct_has_generic {
                struct_has_generic = true;
            }

            s.push_str(&type_str);

            if idx < arr_len - 1 {
                s.push_str(", ");
            }
        }
        s.push('>');

        (s, struct_has_generic)
    }

    fn get_cairo_type_full(&self) -> String {
        let mut s = format!("{}::<", self.cairo_type);

        for (idx, inner) in self.inners.iter().enumerate() {
            s.push_str(&inner.get_cairo_type_full());

            if idx < self.inners.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push('>');
        s
    }

    fn get_cairo_type_name(&self) -> String {
        // TODO: need to opti that with regex?
        let f = self
            .cairo_type
            .split('<')
            .nth(0)
            .unwrap_or(&self.cairo_type)
            .to_string();
        f.split("::").last().unwrap_or(&f).to_string()
    }

    fn to_rust_type(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            let joined_inners = self
                .inners
                .iter()
                .map(|i| i.to_rust_type())
                .collect::<Vec<_>>()
                .join(", ");

            format!("{}<{}>", self.get_cairo_type_name(), joined_inners)
        }
    }

    fn to_rust_type_path(&self) -> String {
        if !self.genty.is_empty() && self.genty != GENTY_FROZEN {
            self.genty.clone()
        } else {
            let joined_inners = self
                .inners
                .iter()
                .map(|i| i.to_rust_type())
                .collect::<Vec<_>>()
                .join(", ");

            format!("{}::<{}>", self.get_cairo_type_name(), joined_inners)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::parser::abi_types::{AbiArray, AbiTypeAny};

    fn get_default() -> AbiGeneric {
        AbiGeneric::new(
            "contract1::MyStruct",
            vec![AbiTypeAny::Basic("core::felt252".into())],
        )
    }

    fn get_default_multiple() -> AbiGeneric {
        AbiGeneric::new(
            "contract1::MyStruct",
            vec![
                AbiTypeAny::Basic("core::felt252".into()),
                AbiTypeAny::Basic("core::integer::u32".into()),
            ],
        )
    }

    #[test]
    fn cairo_type() {
        let t = get_default();
        assert_eq!(t.cairo_type, "contract1::MyStruct");
    }

    #[test]
    fn get_cairo_type_full() {
        let t = get_default();
        assert_eq!(
            t.get_cairo_type_full(),
            "contract1::MyStruct::<core::felt252>"
        );
    }

    #[test]
    fn cairo_type_name_only() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_name(), "MyStruct");
    }

    #[test]
    fn to_rust_type() {
        let t = get_default();
        assert_eq!(
            t.to_rust_type(),
            "MyStruct<starknet::core::types::FieldElement>"
        );
    }

    #[test]
    fn to_rust_type_path() {
        let t = get_default();
        assert_eq!(
            t.to_rust_type_path(),
            "MyStruct::<starknet::core::types::FieldElement>"
        );
    }

    #[test]
    fn from_string() {
        let t = AbiTypeAny::from_string("contract1::MyStruct::<core::felt252>");
        assert_eq!(t, AbiTypeAny::Generic(get_default()));
    }

    #[test]
    fn from_string_array_tuple() {
        let t = AbiTypeAny::from_string("contract1::MyStruct::<core::array::Array<core::felt252>, (core::felt252, core::integer::u32)>");
        assert_eq!(
            t,
            AbiTypeAny::Generic(AbiGeneric::new(
                "contract1::MyStruct",
                vec![
                    AbiTypeAny::Array(AbiArray::new(
                        "core::array::Array",
                        AbiTypeAny::Basic("core::felt252".into())
                    )),
                    AbiTypeAny::Tuple(
                        vec![
                            AbiTypeAny::Basic("core::felt252".into()),
                            AbiTypeAny::Basic("core::integer::u32".into()),
                        ]
                        .into()
                    )
                ]
            ))
        );
    }

    #[test]
    fn get_cairo_type_full_multiple() {
        let t = get_default_multiple();
        assert_eq!(
            t.get_cairo_type_full(),
            "contract1::MyStruct::<core::felt252, core::integer::u32>"
        );
    }

    #[test]
    fn to_rust_type_multiple() {
        let t = get_default_multiple();
        assert_eq!(
            t.to_rust_type(),
            "MyStruct<starknet::core::types::FieldElement, u32>"
        );
    }

    #[test]
    fn to_rust_type_path_multiple() {
        let t = get_default_multiple();
        assert_eq!(
            t.to_rust_type_path(),
            "MyStruct::<starknet::core::types::FieldElement, u32>"
        );
    }

    #[test]
    fn from_string_multiple() {
        let t = AbiTypeAny::from_string("contract1::MyStruct::<core::felt252, core::integer::u32>");
        assert_eq!(t, AbiTypeAny::Generic(get_default_multiple()));
    }

    #[test]
    fn generic_generic() {
        let mut t = AbiTypeAny::from_string("contract1::MyStruct::<core::felt252>");
        assert_eq!(
            t.apply_generic(vec![("contract1::MyStruct::<core::felt252>", "A")]),
            ("A".to_string(), true)
        );
    }

    #[test]
    fn generic_inner() {
        let mut t = AbiTypeAny::from_string("contract1::MyStruct::<core::felt252>");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("contract1::MyStruct::<A>".to_string(), true)
        );
    }

    #[test]
    fn generic_generic_multiple() {
        let mut t =
            AbiTypeAny::from_string("contract1::MyStruct::<core::felt252, core::integer::u32>");
        assert_eq!(
            t.apply_generic(vec![(
                "contract1::MyStruct::<core::felt252, core::integer::u32>",
                "A"
            )]),
            ("A".to_string(), true)
        );
    }

    #[test]
    fn generic_inner_multiple() {
        let mut t =
            AbiTypeAny::from_string("contract1::MyStruct::<core::felt252, core::integer::u32>");
        assert_eq!(
            t.apply_generic(vec![("core::integer::u32", "A")]),
            ("contract1::MyStruct::<core::felt252, A>".to_string(), true)
        );
    }

    #[test]
    fn generic_inner_multiple_array() {
        let mut t = AbiTypeAny::from_string(
            "contract1::MyStruct::<core::array::Array<core::felt252>, core::integer::u32>",
        );
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            (
                "contract1::MyStruct::<core::array::Array::<A>, core::integer::u32>".to_string(),
                true
            )
        );
    }

    #[test]
    fn generic_inner_multiple_ab() {
        let mut t =
            AbiTypeAny::from_string("contract1::MyStruct::<core::felt252, core::integer::u32>");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A"), ("core::integer::u32", "B")]),
            ("contract1::MyStruct::<A, B>".to_string(), true)
        );
    }
}
