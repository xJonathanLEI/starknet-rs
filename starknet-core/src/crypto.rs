use ethereum_types::{H256, U256};
use starknet_crypto::{pedersen_hash, FieldElement};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PedersenHashError {
    #[error("data must not be empty")]
    EmptyData,
    #[error("element out of range: {0}")]
    ElementOutOfRange(U256),
}

pub fn compute_hash_on_elements(data: &[U256]) -> Result<H256, PedersenHashError> {
    if data.is_empty() {
        return Err(PedersenHashError::EmptyData);
    }

    // unwrap() is safe here as it'll always succeed
    let mut current_hash = FieldElement::from_bytes_be([0u8; 32]).unwrap();

    for item in data.iter() {
        current_hash = pedersen_hash(
            &current_hash,
            &u256_to_field_element(item).ok_or(PedersenHashError::ElementOutOfRange(*item))?,
        );
    }

    let data_len = U256::from(data.len());
    current_hash = pedersen_hash(
        &current_hash,
        &u256_to_field_element(&data_len).ok_or(PedersenHashError::ElementOutOfRange(data_len))?,
    );

    Ok(H256::from_slice(&current_hash.to_bytes_le()[..]))
}

fn u256_to_field_element(num: &U256) -> Option<FieldElement> {
    let mut buffer = [0u8; 32];
    num.to_big_endian(&mut buffer);
    FieldElement::from_bytes_be(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_on_elements() {
        // Generated with `cairo-lang`
        let hash = compute_hash_on_elements(&[
            "0xaa".parse::<U256>().unwrap(),
            "0xbb".parse::<U256>().unwrap(),
            "0xcc".parse::<U256>().unwrap(),
            "0xdd".parse::<U256>().unwrap(),
        ])
        .unwrap();
        let expected_hash = "025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69"
            .parse::<H256>()
            .unwrap();

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn test_compute_hash_on_elements_empty_data() {
        match compute_hash_on_elements(&[]) {
            Err(PedersenHashError::EmptyData) => {}
            _ => panic!("Should throw error on empty data"),
        };
    }

    #[test]
    fn test_compute_hash_on_elements_out_of_range() {
        match compute_hash_on_elements(&[
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
        ]) {
            Err(PedersenHashError::ElementOutOfRange(_)) => {}
            _ => panic!("Should throw error on out of range data"),
        };
    }
}
