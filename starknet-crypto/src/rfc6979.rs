use crypto_bigint::{ArrayEncoding, ByteArray, Integer, U256};
use hmac::digest::Digest;
use sha2::digest::{crypto_common::BlockSizeUser, FixedOutputReset, HashMarker};
use zeroize::{Zeroize, Zeroizing};

use crate::FieldElement;

const EC_ORDER: U256 =
    U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");

/// Deterministically generate ephemeral scalar `k` based on RFC 6979.
///
/// ### Arguments
///
/// * `message_hash`: message hash
/// * `private_key`: private key
/// * `seed`: extra seed for additional entropy
pub fn generate_k(
    message_hash: &FieldElement,
    private_key: &FieldElement,
    seed: Option<&FieldElement>,
) -> FieldElement {
    // The message hash padding as implemented in `cairo-lang` is not needed here. The hash is
    // padded in `cairo-lang` only to make sure the lowest 4 bits won't get truncated, but here it's
    // never getting truncated anyways.
    let message_hash = U256::from_be_slice(&message_hash.to_bytes_be()).to_be_byte_array();
    let private_key = U256::from_be_slice(&private_key.to_bytes_be());

    let seed_bytes = match seed {
        Some(seed) => seed.to_bytes_be(),
        None => [0u8; 32],
    };

    let mut first_non_zero_index = 32;
    for (ind, element) in seed_bytes.iter().enumerate() {
        if *element != 0u8 {
            first_non_zero_index = ind;
            break;
        }
    }

    let k = generate_k_shifted::<sha2::Sha256, _>(
        &private_key,
        &EC_ORDER,
        &message_hash,
        &seed_bytes[first_non_zero_index..],
    );

    let mut buffer = [0u8; 32];
    buffer[..].copy_from_slice(&k.to_be_byte_array()[..]);

    FieldElement::from_bytes_be(&buffer).unwrap()
}

// Modified from upstream `rfc6979::generate_k` with a hard-coded right bit shift. The more
// idiomatic way of doing this seems to be to implement `U252` which handles bit truncation
// interally.
// TODO: change to use upstream `generate_k` directly.
#[inline]
fn generate_k_shifted<D, I>(x: &I, n: &I, h: &ByteArray<I>, data: &[u8]) -> Zeroizing<I>
where
    D: Default + Digest + BlockSizeUser + FixedOutputReset + HashMarker,
    I: ArrayEncoding + Integer + Zeroize,
{
    let mut x = x.to_be_byte_array();
    let mut hmac_drbg = rfc6979::HmacDrbg::<D>::new(&x, h, data);
    x.zeroize();

    loop {
        let mut bytes = ByteArray::<I>::default();
        hmac_drbg.fill_bytes(&mut bytes);
        let k = I::from_be_byte_array(bytes) >> 4;

        if (!k.is_zero() & k.ct_lt(n)).into() {
            return Zeroizing::new(k);
        }
    }
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
        let test_vectors: Vec<Rfc6979TestVecotr> = serde_json::from_str(json_str).unwrap();

        for test_vector in test_vectors.iter() {
            let msg_hash = field_element_from_be_hex(test_vector.msg_hash);
            let priv_key = field_element_from_be_hex(test_vector.priv_key);
            let seed = field_element_from_be_hex(test_vector.seed);
            let expected_k = field_element_from_be_hex(test_vector.k);

            let k = generate_k(&msg_hash, &priv_key, Some(&seed));

            assert_eq!(k, expected_k);
        }
    }
}
