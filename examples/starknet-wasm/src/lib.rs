#![allow(clippy::unused_unit)]
#![allow(unexpected_cfgs)]

use starknet_crypto::Felt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_public_key(private_key_hex: &str) -> String {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let private_key = Felt::from_hex(private_key_hex).unwrap();
    let public_key = starknet_crypto::get_public_key(&private_key);

    format!("{public_key:#064x}")
}
