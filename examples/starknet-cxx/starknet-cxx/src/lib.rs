//! This is a quick demo on exposing `starknet-crypto` to C++ with the cxx bridge:
//!   <https://github.com/xJonathanLEI/starknet-rs/issues/325>
//!
//! This wrapper crate expose functions that operate on strings, which is bad and probably hurts
//! performance. It's possible to make the C++ side create `Felt` instances and operate on
//! those instead, which is much more idiomatic. That said, this demo wrapper crate seems to already
//! offer decent performance.
//!
//! Moreover, this crate does not implement error handling and always just panics on error, which
//! is likely not what you want in production.
//!
//! However, the goal of this crate is just to demonstrate using the library from C++, NOT to
//! create idiomatic bindings, which is way too much work to maintain as an example, and should be
//! a project of its own.

use starknet_core::{crypto::Signature, types::Felt};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn pedersen_hash(x: &str, y: &str) -> String;

        fn ecdsa_sign(private_key: &str, message: &str) -> String;
    }
}

pub fn pedersen_hash(x: &str, y: &str) -> String {
    // WARNING: no error handling here
    let x = Felt::from_hex(x).unwrap();
    let y = Felt::from_hex(y).unwrap();

    format!("{:#064x}", starknet_core::crypto::pedersen_hash(&x, &y))
}

fn ecdsa_sign(private_key: &str, message: &str) -> String {
    // WARNING: no error handling here
    let private_key = Felt::from_hex(private_key).unwrap();
    let message = Felt::from_hex(message).unwrap();

    let signature: Signature = starknet_core::crypto::ecdsa_sign(&private_key, &message)
        // WARNING: no error handling here
        .unwrap()
        .into();

    format!("0x{signature}")
}
