#![cfg(test)]

use crate::field_element::{FieldElement, FieldElementRepr};

use ff::PrimeField;

pub fn field_element_from_be_hex(hex: &str) -> FieldElement {
    let decoded = hex::decode(hex.trim_start_matches("0x")).unwrap();

    if decoded.len() > 32 {
        panic!("hex string too long");
    }

    let mut buffer = [0u8; 32];
    buffer[(32 - decoded.len())..].copy_from_slice(&decoded[..]);

    FieldElement::from_repr(FieldElementRepr(buffer)).unwrap()
}
