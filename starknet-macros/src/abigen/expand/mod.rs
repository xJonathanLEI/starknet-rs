pub(crate) mod contract;
pub(crate) mod r#enum;
pub(crate) mod event;
pub(crate) mod function;
pub(crate) mod generic;
pub(crate) mod r#struct;
pub(crate) mod utils;

use starknet_contract::abi::parser::CairoEvent;
use proc_macro2::TokenStream as TokenStream2;

pub trait Expandable {
    fn expand_decl(&self) -> TokenStream2;
    fn expand_impl(&self) -> TokenStream2;
}

pub trait ExpandableEvent {
    fn expand_decl(&self) -> TokenStream2;
    fn expand_impl(&self, events: &[CairoEvent]) -> TokenStream2;
}
