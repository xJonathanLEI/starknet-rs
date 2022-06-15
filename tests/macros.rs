use starknet::{core::utils::get_selector_from_name, macros::selector};

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn selector_can_generate_correct_selector() {
    let macro_value = selector!("balanceOf");
    let function_call_value = get_selector_from_name("balanceOf").unwrap();

    assert_eq!(macro_value, function_call_value);
}
