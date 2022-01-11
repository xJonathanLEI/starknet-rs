use ethereum_types::{H256, U256};
use starkware_crypto_sys::hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PedersenHashError {
    #[error("data must not be empty")]
    EmptyData,
    #[error("StarkWare crypto-cpp error: {code}")]
    StarkWareCryptoError { code: i32 },
}

pub fn compute_hash_on_elements(data: &[U256]) -> Result<H256, PedersenHashError> {
    if data.is_empty() {
        return Err(PedersenHashError::EmptyData);
    }

    let mut current_hash = [0u8; 32];
    let mut buffer_y = [0u8; 32];

    let data_len = U256::from(data.len());
    for item in data.iter() {
        item.to_little_endian(&mut buffer_y);
        current_hash = match hash(&current_hash, &buffer_y) {
            Ok(result) => result,
            Err(err_code) => {
                return Err(PedersenHashError::StarkWareCryptoError { code: err_code })
            }
        };
    }

    data_len.to_little_endian(&mut buffer_y);
    current_hash = match hash(&current_hash, &buffer_y) {
        Ok(result) => result,
        Err(err_code) => return Err(PedersenHashError::StarkWareCryptoError { code: err_code }),
    };

    current_hash.reverse();
    Ok(H256::from_slice(&current_hash[..]))
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
        let expected_hash = "0x025cde77210b1c223b2c6e69db6e9021aa1599177ab177474d5326cd2a62cb69"
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
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .parse::<U256>()
                .unwrap(),
        ]) {
            Err(PedersenHashError::StarkWareCryptoError { code: _ }) => {}
            _ => panic!("Should throw error on out of range data"),
        };
    }
}
