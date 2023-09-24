//! Utils function for expansion.
use syn::{Ident, LitStr, Type};

///
pub fn str_to_ident(str_in: &str) -> Ident {
    Ident::new(str_in, proc_macro2::Span::call_site())
}

///
pub fn str_to_type(str_in: &str) -> Type {
    syn::parse_str(str_in).unwrap_or_else(|_| panic!("Can't convert {} to syn::Type", str_in))
}

///
pub fn str_to_litstr(str_in: &str) -> LitStr {
    LitStr::new(str_in, proc_macro2::Span::call_site())
}
