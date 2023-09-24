//! Struct expansion, taking in account generic types if any.
use super::{
    generic, Expandable,
    utils::{str_to_ident, str_to_type}
};

use starknet_contract::abi::parser::{
    CairoStruct,
    abi_types::{AbiType, AbiTypeAny},
};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

impl Expandable for CairoStruct {
    fn expand_decl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.get_name());

        let mut members: Vec<TokenStream2> = vec![];
        for (name, abi_type) in &self.members {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type());

            members.push(quote!(#name: #ty));
        }

        if self.is_generic() {
            let gentys: Vec<Ident> = self.get_gentys().iter().map(|g| str_to_ident(g)).collect();

            quote! {
                #[derive(Debug, PartialEq)]
                pub struct #struct_name<#(#gentys),*> {
                    #(pub #members),*
                }
            }
        } else {
            quote! {
                #[derive(Debug, PartialEq)]
                pub struct #struct_name {
                    #(pub #members),*
                }
            }
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let struct_name = str_to_ident(&self.get_name());

        let mut sizes: Vec<TokenStream2> = vec![];
        let mut sers: Vec<TokenStream2> = vec![];
        let mut desers: Vec<TokenStream2> = vec![];
        let mut names: Vec<TokenStream2> = vec![];

        let mut is_first = true;
        for (name, abi_type) in &self.members {
            let name = str_to_ident(name);
            names.push(quote!(#name));

            let ty = str_to_type(&abi_type.to_rust_type_path());

            // Tuples type used as rust type item path must be surrounded
            // by angle brackets.
            let ty_punctuated = match abi_type {
                AbiTypeAny::Tuple(_) => quote!(<#ty>),
                _ => quote!(#ty),
            };

            if is_first {
                sizes.push(quote!(#ty_punctuated::serialized_size(&rust.#name)));
                is_first = false;
            } else {
                sizes.push(quote!(+ #ty_punctuated::serialized_size(&rust.#name)));
            }

            sers.push(quote!(out.extend(#ty_punctuated::serialize(&rust.#name));));

            desers.push(quote! {
                let #name = #ty_punctuated::deserialize(felts, offset)?;
                offset += #ty_punctuated::serialized_size(&#name);
            });
        }

        let gentys: Vec<Ident> = self.get_gentys().iter().map(|g| str_to_ident(g)).collect();

        let impl_line = if self.is_generic() {
            generic::impl_with_gentys_tokens(&struct_name, &gentys)
        } else {
            quote!(impl cairo_types::CairoType for #struct_name)
        };

        let rust_type = if self.is_generic() {
            generic::rust_associated_type_gentys_tokens(&struct_name, &gentys)
        } else {
            quote!(
                type RustType = Self;
            )
        };

        quote! {
            #impl_line {

                #rust_type

                const SERIALIZED_SIZE: std::option::Option<usize> = None;

                #[inline]
                fn serialized_size(rust: &Self::RustType) -> usize {
                    #(#sizes) *
                }

                fn serialize(rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
                    let mut out: Vec<starknet::core::types::FieldElement> = vec![];
                    #(#sers)*
                    out
                }

                fn deserialize(felts: &[starknet::core::types::FieldElement], offset: usize) -> cairo_types::Result<Self::RustType> {
                    let mut offset = offset;
                    #(#desers)*
                    Ok(#struct_name {
                        #(#names),*
                    })
                }
            }
        }
    }
}
