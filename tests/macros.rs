use starknet::{
    core::utils::get_selector_from_name,
    macros::{felt, felt_dec, felt_hex, selector},
};
use starknet_core::types::FieldElement;

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn selector_can_generate_correct_selector() {
    let macro_value = selector!("balanceOf");
    let function_call_value = get_selector_from_name("balanceOf").unwrap();

    assert_eq!(macro_value, function_call_value);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn felt_with_dec_string() {
    let macro_value = felt!("1234567");
    let function_call_value = FieldElement::from_dec_str("1234567").unwrap();

    assert_eq!(macro_value, function_call_value);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn felt_with_hex_string() {
    let macro_value = felt!("0x123456789abcdef");
    let function_call_value = FieldElement::from_hex_be("0x123456789abcdef").unwrap();

    assert_eq!(macro_value, function_call_value);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn felt_dec() {
    let macro_value = felt_dec!("1234567");
    let function_call_value = FieldElement::from_dec_str("1234567").unwrap();

    assert_eq!(macro_value, function_call_value);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn felt_hex() {
    let macro_value = felt_hex!("0x123456789abcdef");
    let function_call_value = FieldElement::from_hex_be("0x123456789abcdef").unwrap();

    assert_eq!(macro_value, function_call_value);
}
