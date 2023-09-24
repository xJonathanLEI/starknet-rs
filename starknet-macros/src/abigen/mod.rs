//! This crate contains all the logic to expand the parsed ABI types into
//! rust code.
//!
//! Important note, functions can't be generic when they are entry point
//! of a Cairo contracts.
//! For this reason, all the generic types are handles for structs and enums
//! generation only, and then applied on functions inputs/output.
//!
//! As the ABI as everything flatten, we must ensure that structs and enums are
//! checked for genericty to avoid duplicated types and detect correctly
//! the members/variants that are generic.
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

use std::collections::HashMap;

use starknet_contract::abi::cairo_types::{CAIRO_BASIC_ENUMS, CAIRO_BASIC_STRUCTS};
use starknet_contract::abi::parser::{CairoEnum, CairoEvent, CairoFunction, CairoStruct};
use starknet_core::types::contract::AbiEntry;

mod expand;
use expand::contract::CairoContract;
use expand::{Expandable, ExpandableEvent};

mod contract_abi;
use contract_abi::ContractAbi;

pub fn abigen_internal(input: TokenStream) -> TokenStream {
    let contract_abi = parse_macro_input!(input as ContractAbi);
    let contract_name = contract_abi.name;
    let abi = contract_abi.abi;

    let mut tokens: Vec<TokenStream2> = vec![];

    tokens.push(CairoContract::expand(contract_name.clone()));

    let mut structs: HashMap<String, CairoStruct> = HashMap::new();
    let mut enums: HashMap<String, CairoEnum> = HashMap::new();
    let mut functions = vec![];
    let mut events = vec![];

    for entry in &abi {
        match entry {
            AbiEntry::Struct(s) => {
                let cs = CairoStruct::new(&s.name, &s.members);

                if CAIRO_BASIC_STRUCTS.contains(&cs.get_name().as_str()) {
                    continue;
                }

                if let Some(ref mut existing_cs) = structs.get_mut(&cs.get_name()) {
                    cs.compare_generic_types(existing_cs);
                } else {
                    structs.insert(cs.get_name(), cs.clone());
                }
            }
            AbiEntry::Enum(e) => {
                let ce = CairoEnum::new(&e.name, &e.variants);

                if CAIRO_BASIC_ENUMS.contains(&ce.get_name().as_str()) {
                    continue;
                }

                if let Some(ref mut existing_ce) = enums.get_mut(&ce.get_name()) {
                    ce.compare_generic_types(existing_ce);
                } else {
                    enums.insert(ce.get_name(), ce.clone());
                }
            }
            AbiEntry::Function(f) => {
                // Functions cannot be generic when they are entry point.
                // From this statement, we can safely assume that any function name is
                // unique.
                let cf =
                    CairoFunction::new(&f.name, f.state_mutability.clone(), &f.inputs, &f.outputs);
                functions.push(cf.expand_impl());
            }
            AbiEntry::Event(ev) => {
                if let Some(cev) = CairoEvent::new(ev) {
                    events.push(cev);
                }
            }
            _ => continue,
        }
    }

    for (_, cs) in structs {
        tokens.push(cs.expand_decl());
        tokens.push(cs.expand_impl());
    }

    for (_, ce) in enums {
        tokens.push(ce.expand_decl());
        tokens.push(ce.expand_impl());
    }

    for ev in &events {
        tokens.push(ev.expand_decl());
        tokens.push(ev.expand_impl(&events));
    }

    tokens.push(quote! {
        impl<P> #contract_name<P>
        where
            P: starknet::providers::Provider + Send + Sync, <P as starknet::providers::Provider>::Error: 'static
        {
            #(#functions)*
        }
    });

    let expanded = quote! {
        #(#tokens)*
    };

    expanded.into()
}
