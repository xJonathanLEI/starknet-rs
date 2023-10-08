//! Enums expansion, taking in account generic types if any.
use super::{
    generic,
    utils::{str_to_ident, str_to_type},
    Expandable,
};

use starknet_contract::abi::parser::{
    abi_types::{AbiType, AbiTypeAny},
    CairoEnum,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

impl Expandable for CairoEnum {
    fn expand_decl(&self) -> TokenStream2 {
        let enum_name = str_to_ident(&self.get_name());

        let mut variants: Vec<TokenStream2> = vec![];

        for (name, abi_type) in &self.variants {
            let name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type());
            if abi_type.is_unit() {
                variants.push(quote!(#name));
            } else {
                variants.push(quote!(#name(#ty)));
            }
        }

        if self.is_generic() {
            let gentys: Vec<Ident> = self.get_gentys().iter().map(|g| str_to_ident(g)).collect();

            quote! {
                #[derive(Debug, PartialEq)]
                pub enum #enum_name<#(#gentys),*> {
                    #(#variants),*
                }
            }
        } else {
            quote! {
                #[derive(Debug, PartialEq)]
                pub enum #enum_name {
                    #(#variants),*
                }
            }
        }
    }

    fn expand_impl(&self) -> TokenStream2 {
        let name_str = &self.get_name();
        let enum_name = str_to_ident(name_str);

        let mut serialized_sizes: Vec<TokenStream2> = vec![];
        let mut serializations: Vec<TokenStream2> = vec![];
        let mut deserializations: Vec<TokenStream2> = vec![];

        for (i, (name, abi_type)) in self.variants.iter().enumerate() {
            let variant_name = str_to_ident(name);
            let ty = str_to_type(&abi_type.to_rust_type_path());

            // Tuples type used as rust type item path must be surrounded
            // by angle brackets.
            let ty_punctuated = match abi_type {
                AbiTypeAny::Tuple(_) => quote!(<#ty>),
                _ => quote!(#ty),
            };

            if abi_type.is_unit() {
                serializations.push(quote! {
                    #enum_name::#variant_name => usize::serialize(&#i)
                });
                deserializations.push(quote! {
                    #i => Ok(#enum_name::#variant_name)
                });
                serialized_sizes.push(quote! {
                    #enum_name::#variant_name => 1
                });
            } else {
                serializations.push(quote! {
                    #enum_name::#variant_name(val) => {
                        let mut temp = vec![];
                        temp.extend(usize::serialize(&#i));
                        temp.extend(#ty_punctuated::serialize(val));
                        temp
                    }
                });
                deserializations.push(quote! {
                    #i => Ok(#enum_name::#variant_name(#ty_punctuated::deserialize(__felts, __offset + 1)?))
                });
                // +1 because we have to handle the variant index also.
                serialized_sizes.push(quote! {
                    #enum_name::#variant_name(val) => #ty_punctuated::serialized_size(val) + 1
                })
            }
        }

        deserializations.push(quote! {
            _ => panic!("Index not handle for enum {}", #name_str)
        });

        let gentys: Vec<Ident> = self.get_gentys().iter().map(|g| str_to_ident(g)).collect();

        let impl_line = if self.is_generic() {
            generic::impl_with_gentys_tokens(&enum_name, &gentys)
        } else {
            quote!(impl starknet::contract::abi::CairoType for #enum_name)
        };

        let rust_type = if self.is_generic() {
            generic::rust_associated_type_gentys_tokens(&enum_name, &gentys)
        } else {
            quote!(
                type RustType = Self;
            )
        };

        quote! {
            #impl_line {

                #rust_type

                const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;

                #[inline]
                fn serialized_size(__rust: &Self::RustType) -> usize {
                    match __rust {
                        #(#serialized_sizes),*
                    }
                }

                fn serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::FieldElement> {
                    match __rust {
                        #(#serializations),*
                    }
                }

                fn deserialize(__felts: &[starknet::core::types::FieldElement], __offset: usize) -> starknet::contract::abi::cairo_types::Result<Self::RustType> {
                    let __index:u128 = __felts[__offset].try_into().unwrap();
                    match __index as usize {
                        #(#deserializations),*
                    }

                }
            }
        }
    }
}
