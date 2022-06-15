use proc_macro::TokenStream;
use starknet_core::utils::get_selector_from_name;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let selector_value = get_selector_from_name(&str_value).expect("invalid selector name");
    let selector_raw = selector_value.into_mont();

    format!(
        "::starknet::core::types::FieldElement::from_mont([{}, {}, {}, {}])",
        selector_raw[0], selector_raw[1], selector_raw[2], selector_raw[3],
    )
    .parse()
    .unwrap()
}
