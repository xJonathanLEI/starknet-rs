use starknet_types_core::felt::Felt;

pub fn field_element_from_be_hex(hex: &str) -> Felt {
    let decoded = hex::decode(hex.trim_start_matches("0x")).unwrap();

    if decoded.len() > 32 {
        panic!("hex string too long");
    }

    let mut buffer = [0u8; 32];
    buffer[(32 - decoded.len())..].copy_from_slice(&decoded[..]);

    Felt::from_bytes_be(&buffer)
}
