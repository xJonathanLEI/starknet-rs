use proc_macro::TokenStream;
use starknet_core::{
    types::FieldElement,
    utils::{cairo_short_string_to_felt, get_selector_from_name},
};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let selector_value = get_selector_from_name(&str_value).expect("invalid selector name");
    let selector_raw = selector_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        selector_raw[0],
        selector_raw[1],
        selector_raw[2],
        selector_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn short_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = cairo_short_string_to_felt(&str_value).expect("invalid Cairo short string");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = if str_value.starts_with("0x") {
        FieldElement::from_hex_be(&str_value).expect("invalid FieldElement value")
    } else {
        FieldElement::from_dec_str(&str_value).expect("invalid FieldElement value")
    };

    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt_dec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = FieldElement::from_dec_str(&str_value).expect("invalid FieldElement value");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn felt_hex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let str_value = input.value();

    let felt_value = FieldElement::from_hex_be(&str_value).expect("invalid FieldElement value");
    let felt_raw = felt_value.into_mont();

    format!(
        "{}::from_mont([{}, {}, {}, {}])",
        field_element_path(),
        felt_raw[0],
        felt_raw[1],
        felt_raw[2],
        felt_raw[3],
    )
    .parse()
    .unwrap()
}

#[cfg(feature = "use_imported_type")]
fn field_element_path() -> &'static str {
    "FieldElement"
}

#[cfg(not(feature = "use_imported_type"))]
fn field_element_path() -> &'static str {
    "::starknet::core::types::FieldElement"
}
