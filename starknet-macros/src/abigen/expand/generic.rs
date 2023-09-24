//! Utils functions for generic expansion.
use super::utils::str_to_ident;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

/// Expands the implementation line with generic types.
pub fn impl_with_gentys_tokens(entity_name: &Ident, gentys: &Vec<Ident>) -> TokenStream2 {
    let gentys_rust: Vec<Ident> = gentys
        .iter()
        .map(|g| str_to_ident(format!("R{}", g).as_str()))
        .collect();

    let mut tokens = vec![];

    tokens.push(quote! {
        impl<#(#gentys),* , #(#gentys_rust),*> starknet::contract::abi::CairoType for #entity_name<#(#gentys),*>
        where
    });

    for (i, g) in gentys.iter().enumerate() {
        let gr = &gentys_rust[i];
        tokens.push(quote!(#g: starknet::contract::abi::CairoType<RustType = #gr>,));
    }

    quote!(#(#tokens)*)
}

/// Expands the associated types lines for generic types.
pub fn rust_associated_type_gentys_tokens(entity_name: &Ident, gentys: &[Ident]) -> TokenStream2 {
    let gentys_rust: Vec<Ident> = gentys
        .iter()
        .map(|g| str_to_ident(format!("R{}", g).as_str()))
        .collect();

    quote!(type RustType = #entity_name<#(#gentys_rust),*>;)
}
