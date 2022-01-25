use crate::FieldElement;

pub fn generate_k(
    message_hash: &FieldElement,
    private_key: &FieldElement,
    seed: &FieldElement,
) -> FieldElement {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Rfc6979TestVecotr {
        msg_hash: String,
        priv_key: String,
        seed: String,
        k: String,
    }

    #[test]
    fn test_generate_k_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_padded.json"));
    }

    #[test]
    fn test_generate_k_not_padded() {
        // Test vectors generated from `cairo-lang`
        test_generate_k_from_json_str(include_str!("../test-data/rfc6979_not_padded.json"));
    }

    fn test_generate_k_from_json_str(json_str: &'static str) {
        let test_vectors: Vec<Rfc6979TestVecotr> = serde_json::from_str(json_str).unwrap();

        for test_vector in test_vectors.iter() {
            let msg_hash = field_element_from_be_hex(&test_vector.msg_hash);
            let priv_key = field_element_from_be_hex(&test_vector.priv_key);
            let seed = field_element_from_be_hex(&test_vector.seed);
            let expected_k = field_element_from_be_hex(&test_vector.k);

            let k = generate_k(&msg_hash, &priv_key, &seed);

            assert_eq!(k, expected_k);
        }
    }
}
