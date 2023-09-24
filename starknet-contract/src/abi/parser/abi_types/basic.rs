//! Basic types are all cairo types that are not Array/Span,
//! generic Struct/Enum or tuple.
//!
//! To support recursion, the basic type stored the generic type
//! that is assigned to it, if it belongs to a generic struct/enum.
use super::{AbiType, AbiTypeAny, GENTY_FROZEN};

#[derive(Debug, PartialEq, Clone)]
pub struct AbiBasic {
    cairo_type: String,
    genty: String,
}

impl AbiBasic {
    /// Initializes a new instance.
    pub fn new(cairo_type: &str) -> Self {
        AbiBasic {
            cairo_type: cairo_type.to_string(),
            genty: String::new(),
        }
    }

    /// Maps a basic type to a built-in type that may already contains
    /// a `CairoType` implementation. If not, it's the name of the type itself.
    fn to_rust_or_cairo_builtin_type(&self) -> String {
        let s = self.get_cairo_type_name();
        match s.as_str() {
            "felt252" => "starknet::core::types::FieldElement".to_string(),
            "ContractAddress" => {
                "starknet::contract::abi::cairo_types::ContractAddress".to_string()
            }
            "ClassHash" => "starknet::contract::abi::cairo_types::ClassHash".to_string(),
            "EthAddress" => "starknet::contract::abi::cairo_types::EthAddress".to_string(),
            _ => s.clone(),
        }
    }
}

impl From<&str> for AbiBasic {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<&String> for AbiBasic {
    fn from(s: &String) -> Self {
        Self::new(s)
    }
}

impl AbiType for AbiBasic {
    fn get_genty(&self) -> String {
        self.genty.clone()
    }

    fn compare_generic(&mut self, other: &AbiTypeAny) {
        if self.genty != GENTY_FROZEN {
            self.genty = other.get_genty();
        }
    }

    fn apply_generic(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        // A basic type can only match one of the given types.
        // It will return the first match we can find, if any.
        for (cairo_type, genty) in cairo_types_gentys {
            if self.cairo_type == cairo_type {
                self.genty = genty.to_string();
                return (genty.to_string(), true);
            }
        }

        self.genty = GENTY_FROZEN.to_string();
        (self.cairo_type.clone(), false)
    }

    fn get_cairo_type_full(&self) -> String {
        self.cairo_type.clone()
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
            self.to_rust_or_cairo_builtin_type()
        }
    }

    fn to_rust_type_path(&self) -> String {
        self.to_rust_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::parser::abi_types::AbiTypeAny;

    fn get_default() -> AbiBasic {
        AbiBasic::new("core::felt252")
    }

    #[test]
    fn get_cairo_type_full() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_full(), "core::felt252");
    }

    #[test]
    fn cairo_type_name_only() {
        let t = get_default();
        assert_eq!(t.get_cairo_type_name(), "felt252");
    }

    #[test]
    fn to_rust_type() {
        let t = get_default();
        assert_eq!(t.to_rust_type(), "starknet::core::types::FieldElement");
    }

    #[test]
    fn to_rust_type_path() {
        let t = get_default();
        assert_eq!(t.to_rust_type_path(), "starknet::core::types::FieldElement");
    }
    // TODO: add more tests for other built-in types.

    #[test]
    fn from_string() {
        let t = AbiTypeAny::from_string("core::felt252");
        assert_eq!(t, AbiTypeAny::Basic("core::felt252".into()));
    }

    #[test]
    fn from_string_generic() {
        let mut t = AbiTypeAny::from_string("core::felt252");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("A".to_string(), true)
        );
    }

    #[test]
    fn from_string_not_generic() {
        let mut t = AbiTypeAny::from_string("core::u32");
        assert_eq!(
            t.apply_generic(vec![("core::felt252", "A")]),
            ("core::u32".to_string(), false)
        );
    }
}
