//! Procedural derive macros for the `starknet-core` crate.

#![deny(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Error as ParseError, Parse, ParseStream},
    parse_macro_input, DeriveInput, Fields, LitInt, LitStr, Meta, Token,
};

#[derive(Default)]
struct Args {
    core: Option<LitStr>,
}

impl Args {
    fn merge(&mut self, other: Self) {
        if let Some(core) = other.core {
            if self.core.is_some() {
                panic!("starknet attribute `core` defined more than once");
            } else {
                self.core = Some(core);
            }
        }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self, ParseError> {
        let mut core: Option<LitStr> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::core) {
                let _ = input.parse::<kw::core>()?;
                let _ = input.parse::<Token![=]>()?;
                let value = input.parse::<LitStr>()?;

                match core {
                    Some(_) => {
                        return Err(ParseError::new(
                            Span::call_site(),
                            "starknet attribute `core` defined more than once",
                        ))
                    }
                    None => {
                        core = Some(value);
                    }
                }
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(Self { core })
    }
}

mod kw {
    syn::custom_keyword!(core);
}

/// Derives the `Encode` trait.
#[proc_macro_derive(Encode, attributes(starknet))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = &input.ident;

    let core = derive_core_path(&input);

    let impl_block = match input.data {
        syn::Data::Struct(data) => {
            let field_impls = data.fields.iter().enumerate().map(|(ind_field, field)| {
                let field_ident = match &field.ident {
                    Some(field_ident) => quote! { self.#field_ident },
                    None => {
                        let ind_field = syn::Index::from(ind_field);
                        quote! { self.#ind_field }
                    }
                };
                let field_type = &field.ty;

                quote! {
                    <#field_type as #core::codec::Encode>::encode(&#field_ident, writer)?;
                }
            });

            quote! {
                #(#field_impls)*
            }
        }
        syn::Data::Enum(data) => {
            let variant_impls =
                data.variants
                    .iter()
                    .enumerate()
                    .map(|(ind_variant, variant)| {
                        let variant_ident = &variant.ident;
                        let ind_variant = int_to_felt(ind_variant, &core);

                        match &variant.fields {
                            Fields::Named(fields_named) => {
                                let names = fields_named
                                    .named
                                    .iter()
                                    .map(|field| field.ident.as_ref().unwrap());

                                let field_impls = fields_named.named.iter().map(|field| {
                                    let field_ident = field.ident.as_ref().unwrap();
                                    let field_type = &field.ty;

                                    quote! {
                                        <#field_type as #core::codec::Encode>
                                            ::encode(#field_ident, writer)?;
                                    }
                                });

                                quote! {
                                    Self::#variant_ident { #(#names),* } => {
                                        writer.write(#ind_variant);
                                        #(#field_impls)*
                                    },
                                }
                            }
                            Fields::Unnamed(fields_unnamed) => {
                                let names = fields_unnamed.unnamed.iter().enumerate().map(
                                    |(ind_field, _)| {
                                        syn::Ident::new(
                                            &format!("field_{ind_field}"),
                                            Span::call_site(),
                                        )
                                    },
                                );

                                let field_impls = fields_unnamed.unnamed.iter().enumerate().map(
                                    |(ind_field, field)| {
                                        let field_ident = syn::Ident::new(
                                            &format!("field_{ind_field}"),
                                            Span::call_site(),
                                        );
                                        let field_type = &field.ty;

                                        quote! {
                                            <#field_type as #core::codec::Encode>
                                                ::encode(#field_ident, writer)?;
                                        }
                                    },
                                );

                                quote! {
                                    Self::#variant_ident( #(#names),* ) => {
                                        writer.write(#ind_variant);
                                        #(#field_impls)*
                                    },
                                }
                            }
                            Fields::Unit => {
                                quote! {
                                    Self::#variant_ident => {
                                        writer.write(#ind_variant);
                                    },
                                }
                            }
                        }
                    });

            quote! {
                match self {
                    #(#variant_impls)*
                }
            }
        }
        syn::Data::Union(_) => panic!("union type not supported"),
    };

    quote! {
        #[automatically_derived]
        impl #core::codec::Encode for #ident {
            fn encode<W: #core::codec::FeltWriter>(&self, writer: &mut W)
                -> ::core::result::Result<(), #core::codec::Error> {
                #impl_block

                Ok(())
            }
        }
    }
    .into()
}

/// Derives the `Decode` trait.
#[proc_macro_derive(Decode, attributes(starknet))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = &input.ident;

    let core = derive_core_path(&input);

    let impl_block = match input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields_named) => {
                let field_impls = fields_named.named.iter().map(|field| {
                    let field_ident = &field.ident;
                    let field_type = &field.ty;

                    quote! {
                        #field_ident: <#field_type as #core::codec::Decode>
                            ::decode_iter(iter)?,
                    }
                });

                quote! {
                    Ok(Self {
                        #(#field_impls)*
                    })
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_impls = fields_unnamed.unnamed.iter().map(|field| {
                    let field_type = &field.ty;
                    quote! {
                        <#field_type as #core::codec::Decode>::decode_iter(iter)?
                    }
                });

                quote! {
                    Ok(Self (
                        #(#field_impls),*
                    ))
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(Self)
                }
            }
        },
        syn::Data::Enum(data) => {
            let variant_impls = data
                .variants
                .iter()
                .enumerate()
                .map(|(ind_variant, variant)| {
                    let variant_ident = &variant.ident;
                    let ind_variant = int_to_felt(ind_variant, &core);

                    let decode_impl = match &variant.fields {
                        Fields::Named(fields_named) => {
                            let field_impls = fields_named.named.iter().map(|field| {
                                let field_ident = field.ident.as_ref().unwrap();
                                let field_type = &field.ty;

                                quote! {
                                    #field_ident: <#field_type as #core::codec::Decode>
                                        ::decode_iter(iter)?,
                                }
                            });

                            quote! {
                                return Ok(Self::#variant_ident {
                                    #(#field_impls)*
                                });
                            }
                        }
                        Fields::Unnamed(fields_unnamed) => {
                            let field_impls = fields_unnamed.unnamed.iter().map(|field| {
                                let field_type = &field.ty;

                                quote! {
                                    <#field_type as #core::codec::Decode>::decode_iter(iter)?
                                }
                            });

                            quote! {
                                return Ok(Self::#variant_ident( #(#field_impls),* ));
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                return Ok(Self::#variant_ident);
                            }
                        }
                    };

                    quote! {
                        if tag == &#ind_variant {
                            #decode_impl
                        }
                    }
                });

            let ident = ident.to_string();

            quote! {
                let tag = iter.next().ok_or_else(#core::codec::Error::input_exhausted)?;

                #(#variant_impls)*

                Err(#core::codec::Error::unknown_enum_tag(tag, #ident))
            }
        }
        syn::Data::Union(_) => panic!("union type not supported"),
    };

    quote! {
        #[automatically_derived]
        impl<'a> #core::codec::Decode<'a> for #ident {
            fn decode_iter<T>(iter: &mut T) -> ::core::result::Result<Self, #core::codec::Error>
            where
                T: core::iter::Iterator<Item = &'a #core::types::Felt>
            {
                #impl_block
            }
        }
    }
    .into()
}

/// Determines the path to the `starknet-core` crate root.
fn derive_core_path(input: &DeriveInput) -> proc_macro2::TokenStream {
    let mut attr_args = Args::default();

    for attr in &input.attrs {
        if !attr.meta.path().is_ident("starknet") {
            continue;
        }

        match &attr.meta {
            Meta::Path(_) => {}
            Meta::List(meta_list) => {
                let args: Args = meta_list
                    .parse_args()
                    .expect("unable to parse starknet attribute args");

                attr_args.merge(args);
            }
            Meta::NameValue(_) => panic!("starknet attribute must not be name-value"),
        }
    }

    attr_args.core.map_or_else(
        || {
            #[cfg(not(feature = "import_from_starknet"))]
            quote! {
                ::starknet_core
            }

            // This feature is enabled by the `starknet` crate. When using `starknet` it's assumed
            // that users would not have imported `starknet-core` directly.
            #[cfg(feature = "import_from_starknet")]
            quote! {
                ::starknet::core
            }
        },
        |id| id.parse().expect("unable to parse core crate path"),
    )
}

/// Turns an integer into an optimal `TokenStream` that constructs a `Felt` with the same value.
fn int_to_felt(int: usize, core: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match int {
        0 => quote! { #core::types::Felt::ZERO },
        1 => quote! { #core::types::Felt::ONE },
        2 => quote! { #core::types::Felt::TWO },
        3 => quote! { #core::types::Felt::THREE },
        // TODO: turn the number into Montgomery repr and use const ctor instead.
        _ => {
            let literal = LitInt::new(&int.to_string(), Span::call_site());
            quote! { #core::types::Felt::from(#literal) }
        }
    }
}
