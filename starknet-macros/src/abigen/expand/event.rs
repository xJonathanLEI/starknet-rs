//! Events expansion.
use super::{
    Expandable, ExpandableEvent,
    utils::{str_to_ident, str_to_litstr, str_to_type}
};

use starknet_contract::abi::parser::{
    CairoEvent, CairoEventInner,
    abi_types::{AbiType, AbiTypeAny}
};
use starknet_core::types::contract::EventFieldKind;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl ExpandableEvent for CairoEvent {
    fn expand_decl(&self) -> TokenStream2 {
        let decl = match &self.inner {
            CairoEventInner::Struct(s) => s.expand_decl(),
            CairoEventInner::Enum(e) => e.expand_decl(),
        };
        quote!(#decl)
    }

    fn expand_impl(&self, events: &[CairoEvent]) -> TokenStream2 {
        let mut tokens = vec![];

        let inner_imp = match &self.inner {
            CairoEventInner::Struct(s) => s.expand_impl(),
            CairoEventInner::Enum(e) => e.expand_impl(),
        };

        tokens.push(quote!(#inner_imp));

        // Generate the get_selector() method for this event.
        let name_ident = str_to_ident(&self.get_name());
        let name_str = str_to_litstr(&self.get_name());
        let selector = quote! {
            impl #name_ident {
                pub fn get_selector() -> starknet::core::types::FieldElement {
                    starknet::macros::selector!(#name_str)
                }
            }
        };

        tokens.push(selector);

        // Stop here if it's not the Event enum.
        if self.get_name() != "Event" {
            return quote! {
                #(#tokens)*
            };
        }

        // If it's the Event enum, we can generate the TryFrom<EmittedEvent>.

        // It should always be an enum here.
        if let CairoEventInner::Enum(inner) = &self.inner {
            let mut variants_tokens = vec![];

            for (v_name, _) in &inner.variants {
                // Get the corresponding CairoEvent in the array to access it's fields.
                let cev = events
                    .iter()
                    .find(|&e| &e.get_name() == v_name)
                    .unwrap_or_else(|| panic!("Event variant {} was not found in events", v_name));

                let _cev_fields_kinds = cev.count_fields_kinds();

                let mut desers_tokens = vec![];
                let mut names_tokens = vec![];
                let v_ident = str_to_ident(v_name);
                let v_name_str = str_to_litstr(v_name);

                // Let's write the deserialization of each member/variants
                // of the current event.
                match &cev.inner {
                    CairoEventInner::Struct(s) => {
                        for (idx, (name, abi_type)) in s.members.iter().enumerate() {
                            let kind = &cev.fields_kinds[idx];
                            let name_str = str_to_litstr(name);
                            let name = str_to_ident(name);
                            let ty = str_to_type(&abi_type.to_rust_type_path());
                            let ty_punctuated = match abi_type {
                                AbiTypeAny::Tuple(_) => quote!(<#ty>),
                                _ => quote!(#ty),
                            };

                            match kind {
                                EventFieldKind::Key => {
                                    desers_tokens.push(quote! {
                                        let #name = match #ty_punctuated::deserialize(&event.keys, key_offset) {
                                            Ok(v) => v,
                                            Err(e) => return Err(format!("Could not deserialize field {} for {}: {:?}", #name_str, #v_name_str, e)),
                                        };
                                        key_offset += #ty_punctuated::serialized_size(&#name);
                                    });
                                }
                                EventFieldKind::Data => {
                                    desers_tokens.push(quote! {
                                        let #name = match #ty_punctuated::deserialize(&event.data, data_offset) {
                                            Ok(v) => v,
                                            Err(e) => return Err(format!("Could not deserialize field {} for {}: {:?}", #name_str, #v_name_str, e)),
                                        };
                                        data_offset += #ty_punctuated::serialized_size(&#name);
                                    });
                                }
                                _ => {}
                            };

                            names_tokens.push(quote!(#name));
                        }
                    }
                    CairoEventInner::Enum(e) => {
                        for (idx, (name, abi_type)) in e.variants.iter().enumerate() {
                            let kind = &cev.fields_kinds[idx];
                            let name_str = str_to_litstr(name);
                            let name = str_to_ident(name);
                            let ty = str_to_type(&abi_type.to_rust_type_path());
                            let ty_punctuated = match abi_type {
                                AbiTypeAny::Tuple(_) => quote!(<#ty>),
                                _ => quote!(#ty),
                            };

                            match kind {
                                EventFieldKind::Key => {
                                    desers_tokens.push(quote! {
                                        let #name = match #ty_punctuated::deserialize(&event.keys, key_offset) {
                                            Ok(v) => v,
                                            Err(e) => return Err(format!("Could not deserialize field {} for {}: {:?}", #name_str, #v_name_str, e)),
                                        };
                                        key_offset += #ty_punctuated::serialized_size(&#name);
                                    });
                                }
                                EventFieldKind::Data => {
                                    desers_tokens.push(quote! {
                                        let #name = match #ty_punctuated::deserialize(&event.data, data_offset) {
                                            Ok(v) => v,
                                            Err(e) => return Err(format!("Could not deserialize field {} for {}: {:?}", #name_str, #v_name_str, e)),
                                        };
                                        data_offset += #ty_punctuated::serialized_size(&#name);
                                    });
                                }
                                _ => {}
                            };

                            names_tokens.push(quote!(#name));
                        }
                    }
                };

                let variant = quote! {
                    if selector == #v_ident::get_selector() {
                        // TODO: add a validation to check keys len and data len.
                        // To have a nice error message if the event is not formatted as
                        // expected.

                        // We skip the selector.
                        let mut key_offset = 1;
                        let mut data_offset = 0;

                        #(#desers_tokens)*

                        return Ok(Event::#v_ident(#v_ident {
                            #(#names_tokens),*
                        }))
                    };
                };

                variants_tokens.push(variant);
            }

            // TODO: change for custom type instead of str for error?
            let try_from = quote! {
                impl TryFrom<starknet::core::types::EmittedEvent> for Event {
                    type Error = String;

                    fn try_from(event: starknet::core::types::EmittedEvent) -> Result<Self, Self::Error> {
                        if event.keys.is_empty() {
                            return Err("Missing event selector, no keys found".to_string());
                        }
                        let selector = event.keys[0];

                        #(#variants_tokens)*

                        Err(format!("Could not match any event from selector {:#064x}", selector))
                    }
                }
            };

            tokens.push(try_from);
        }

        quote! {
            #(#tokens)*
        }
    }
}
