use starknet_crypto::{pedersen_hash, poseidon_hash, PedersenHasher, PoseidonHasher};

use crate::{codec::FeltWriter, types::Felt};

/// SNIP-12 revision-dependant hasher that can be used to encode data.
pub trait TypedDataHasher: FeltWriter + Default {
    fn update(&mut self, msg: Felt);

    fn finalize(self) -> Felt;

    fn hash_two_elements(x: Felt, y: Felt) -> Felt;
}

impl TypedDataHasher for PedersenHasher {
    fn update(&mut self, msg: Felt) {
        Self::update(self, msg);
    }

    fn finalize(self) -> Felt {
        Self::finalize(&self)
    }

    fn hash_two_elements(x: Felt, y: Felt) -> Felt {
        pedersen_hash(&x, &y)
    }
}

impl TypedDataHasher for PoseidonHasher {
    fn update(&mut self, msg: Felt) {
        Self::update(self, msg);
    }

    fn finalize(self) -> Felt {
        Self::finalize(self)
    }

    fn hash_two_elements(x: Felt, y: Felt) -> Felt {
        poseidon_hash(x, y)
    }
}
