use hex_literal::hex;
use starknet_types_core::felt::Felt;

const EC_ORDER: [u8; 32] = hex!("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");

/// Deterministically generate ephemeral scalar `k` based on RFC 6979.
///
/// ### Parameters
///
/// - `message_hash`: Message hash.
/// - `private_key`: Private key.
/// - `seed`: Extra seed for additional entropy.
pub fn generate_k(message_hash: &Felt, private_key: &Felt, seed: Option<&Felt>) -> Felt {
    // Convert seed to bytes
    let seed_bytes = seed.map_or([0u8; 32], |s| s.to_bytes_be());

    // Find the index of the first non-zero byte in the seed
    let mut first_non_zero_index = 32;
    for (ind, &element) in seed_bytes.iter().enumerate() {
        if element != 0u8 {
            first_non_zero_index = ind;
            break;
        }
    }

    // Convert GenericArray to [u8; 32]
    let mut k_bytes = [0u8; 32];
    k_bytes.copy_from_slice(
        rfc6979::generate_k::<sha2::Sha256, rfc6979::consts::U32>(
            (&private_key.to_bytes_be()).into(),
            &EC_ORDER.into(),
            (&message_hash.to_bytes_be()).into(),
            &seed_bytes[first_non_zero_index..],
        )
        .as_slice(),
    );

    // Convert bytes to Felt
    Felt::from_bytes_be(&k_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Rfc6979TestVecotr<'a> {
        msg_hash: &'a str,
        priv_key: &'a str,
        seed: &'a str,
        k: &'a str,
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_generate_k_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_padded.json"));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_generate_k_not_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_not_padded.json"));
    }

    fn test_generate_k_from_json_str(json_str: &'static str) {
        let test_vectors: Vec<Rfc6979TestVecotr<'_>> = serde_json::from_str(json_str).unwrap();

        for test_vector in &test_vectors {
            let msg_hash = field_element_from_be_hex(test_vector.msg_hash);
            let priv_key = field_element_from_be_hex(test_vector.priv_key);
            let seed = field_element_from_be_hex(test_vector.seed);
            let expected_k = field_element_from_be_hex(test_vector.k);

            let k = generate_k(&msg_hash, &priv_key, Some(&seed));

            assert_eq!(k, expected_k);
        }
    }
}
