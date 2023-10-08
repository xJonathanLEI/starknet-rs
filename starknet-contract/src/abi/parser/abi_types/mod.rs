//! ABI types base module.
//!
//! The idea of those types is to handle the parsing of any valid
//! flatten cairo type that can also contain nested types.
use std::iter::Peekable;
use std::str::Chars;

pub mod basic;
pub use basic::AbiBasic;

pub mod array;
pub use array::AbiArray;

pub mod generic;
pub use generic::AbiGeneric;

pub mod tuple;
pub use tuple::AbiTuple;

/// If a generic type is flagged as frozen, it means
/// that at least one type in the ABI were found
/// different from the generic one. In that case, we
/// don't want it to be modified again, even if it matches
/// an other generic type.
///
/// # Example
///
/// struct MyStruct<T> {
///   a: felt252,
///   b: T,
/// }
///
/// In this scenario, when `MyStruct<felt252>` is used,
/// we can't know which of a or b is generic. So both will be which is fine.
/// But if in the same ABI we have `MyStruct<u8>`, then it will
/// be possible to detect that b is generic, a is not.
/// So if we parse `MyStruct<u8>` first, we do want to FREEZE a
/// as NOT being generic. Like this even if for `MyStruct<felt252>` there is
/// a match, it will be ignored.
const GENTY_FROZEN: &str = "_";

#[derive(Debug, PartialEq, Clone)]
pub enum AbiTypeAny {
    Basic(AbiBasic),
    Array(AbiArray),
    // Generics is for struct and enums.
    Generic(AbiGeneric),
    Tuple(AbiTuple),
}

pub trait AbiType {
    /// Gets the generic type if the type is generic,
    /// the type name otherwise.
    fn get_genty(&self) -> String;

    /// Compares the generic state between two `AbiTypeAny`.
    /// As the ABI does not provide information about the type genericity,
    /// we must compare several types with the same name to successfully identify
    /// the one that are generic.
    fn compare_generic(&mut self, other: &AbiTypeAny);

    /// Applies a generic type for the given cairo type Vec(cairo_type, generic_type).
    /// Returns the generic type applied and true if the type is generic,
    /// false and the type itself otherwise.
    fn apply_generic(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool);

    /// Gets the full cairo type. A "full" type includes the type
    /// and possible nested types.
    fn get_cairo_type_full(&self) -> String;

    /// Returns only the cairo type name.
    fn get_cairo_type_name(&self) -> String;

    /// Gets the rust type from the `AbiType`.
    /// This always includes all possible nested types and their genericity.
    fn to_rust_type(&self) -> String;

    /// Get the rust type item path from the `AbiType`.
    /// This always includes all possible nested types and their genericity.
    fn to_rust_type_path(&self) -> String;
}

impl AbiType for AbiTypeAny {
    fn compare_generic(&mut self, other: &AbiTypeAny) {
        match self {
            AbiTypeAny::Basic(a) => a.compare_generic(other),
            AbiTypeAny::Array(a) => a.compare_generic(other),
            AbiTypeAny::Generic(a) => a.compare_generic(other),
            AbiTypeAny::Tuple(a) => a.compare_generic(other),
        }
    }

    fn get_genty(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_genty(),
            AbiTypeAny::Array(a) => a.get_genty(),
            AbiTypeAny::Generic(a) => a.get_genty(),
            AbiTypeAny::Tuple(a) => a.get_genty(),
        }
    }

    fn apply_generic(&mut self, cairo_types_gentys: Vec<(&str, &str)>) -> (String, bool) {
        match self {
            AbiTypeAny::Basic(a) => a.apply_generic(cairo_types_gentys),
            AbiTypeAny::Array(a) => a.apply_generic(cairo_types_gentys),
            AbiTypeAny::Generic(a) => a.apply_generic(cairo_types_gentys),
            AbiTypeAny::Tuple(a) => a.apply_generic(cairo_types_gentys),
        }
    }

    fn get_cairo_type_full(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_cairo_type_full(),
            AbiTypeAny::Array(a) => a.get_cairo_type_full(),
            AbiTypeAny::Generic(a) => a.get_cairo_type_full(),
            AbiTypeAny::Tuple(a) => a.get_cairo_type_full(),
        }
    }

    fn get_cairo_type_name(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.get_cairo_type_name(),
            AbiTypeAny::Array(a) => a.get_cairo_type_name(),
            AbiTypeAny::Generic(a) => a.get_cairo_type_name(),
            AbiTypeAny::Tuple(a) => a.get_cairo_type_name(),
        }
    }

    fn to_rust_type(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.to_rust_type(),
            AbiTypeAny::Array(a) => a.to_rust_type(),
            AbiTypeAny::Generic(a) => a.to_rust_type(),
            AbiTypeAny::Tuple(a) => a.to_rust_type(),
        }
    }

    fn to_rust_type_path(&self) -> String {
        match self {
            AbiTypeAny::Basic(a) => a.to_rust_type_path(),
            AbiTypeAny::Array(a) => a.to_rust_type_path(),
            AbiTypeAny::Generic(a) => a.to_rust_type_path(),
            AbiTypeAny::Tuple(a) => a.to_rust_type_path(),
        }
    }
}

/// Utils functions for `AbiTypeAny` to be called
/// without testing the enum variant.
impl AbiTypeAny {
    /// Returns true if the type is a generic,
    /// false otherwise.
    pub fn is_generic(&self) -> bool {
        matches!(self, Self::Generic(_))
    }

    /// Returns true if the type is the unit type,
    /// false otherwise.
    pub fn is_unit(&self) -> bool {
        match self {
            Self::Basic(b) => b.get_cairo_type_full() == "()",
            _ => false,
        }
    }

    /// Parses a string to build an `AbiTypeAny`.
    pub fn from_string(type_string: &str) -> Self {
        let mut chars = type_string.chars().peekable();
        Self::parse_type(&mut chars)
    }

    /// Parses any cairo type from the given string.
    /// This function handles the possible nested types.
    fn parse_type(chars: &mut Peekable<Chars>) -> Self {
        let mut generic_types = Vec::new();
        let mut current_type = String::new();

        while let Some(c) = chars.peek() {
            match c {
                '<' => {
                    chars.next();
                    // In cairo, a generic type is always preceeded by a separator "::".
                    let generic_type =
                        Self::parse_generic(current_type.trim_end_matches("::"), chars);
                    generic_types.push(generic_type);
                    current_type.clear();
                }
                '>' => {
                    break;
                }
                '(' => {
                    chars.next();
                    let tuple_type = Self::parse_tuple(chars);
                    generic_types.push(tuple_type);
                }
                ')' => {
                    break;
                }
                ',' => {
                    break;
                }
                ' ' => {
                    // Ignore white spaces.
                    chars.next();
                }
                _ => {
                    current_type.push(*c);
                    chars.next();
                }
            }
        }

        if !current_type.is_empty() {
            generic_types.push(AbiTypeAny::Basic((&current_type).into()));
        }

        if generic_types.is_empty() {
            // TODO: check if this one may be handled as Basic("()");
            Self::Basic("()".into())
        } else if generic_types.len() == 1 {
            // Basic, Array or Generic with 1 inner type.
            generic_types.pop().unwrap()
        } else if chars.nth(0) == Some('(') {
            // Tuple.
            Self::Tuple(AbiTuple::new(generic_types))
        } else {
            unreachable!();
        }
    }

    /// Parses generic types detected between angle brackets.
    fn parse_generic(current_type: &str, chars: &mut Peekable<Chars>) -> Self {
        let mut inners = vec![];

        while let Some(c) = chars.peek() {
            match c {
                '>' => {
                    chars.next();
                    break;
                }
                ',' => {
                    chars.next();
                }
                _ => {
                    inners.push(Self::parse_type(chars));
                }
            }
        }

        if inners.is_empty() {
            panic!("Array/Span/Generic type expects at least one inner type");
        }

        // Array and Span are processed exactly the same, using `Vec`.
        let is_array = current_type.contains("core::array");

        if is_array {
            if inners.len() == 1 {
                Self::Array(AbiArray::new(current_type, inners[0].clone()))
            } else {
                panic!("Array/Span expect exactly one inner type");
            }
        } else {
            Self::Generic(AbiGeneric::new(current_type, inners))
        }
    }

    /// Parses a tuple, which can also contains nested types.
    fn parse_tuple(chars: &mut Peekable<Chars>) -> Self {
        let mut tuple_values = Vec::new();

        if chars.next_if(|&x| x == ')').is_some() {
            return Self::Basic("()".into());
        }

        while let Some(c) = chars.peek() {
            match c {
                ' ' => {
                    chars.next();
                }
                ',' => {
                    chars.next();
                }
                ')' => {
                    chars.next();
                    break;
                }
                _ => {
                    let v = Self::parse_type(chars);
                    tuple_values.push(v);
                }
            }
        }

        Self::Tuple(AbiTuple::new(tuple_values))
    }
}
