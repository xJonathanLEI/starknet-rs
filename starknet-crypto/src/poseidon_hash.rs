// Code ported from the the implementation from pathfinder here:
//   https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/src/lib.rs

use starknet_types_core::{felt::Felt, hash::Poseidon};

/// A stateful hasher for Starknet Poseidon hash.
///
/// Using this hasher is the same as calling [`poseidon_hash_many`].
#[derive(Debug, Default)]
pub struct PoseidonHasher {
    state: [Felt; 3],
    buffer: Option<Felt>,
}

impl PoseidonHasher {
    /// Creates a new [`PoseidonHasher`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Absorbs message into the hash.
    pub fn update(&mut self, msg: Felt) {
        match self.buffer.take() {
            Some(previous_message) => {
                self.state[0] += previous_message;
                self.state[1] += msg;
                Poseidon::hades_permutation(&mut self.state);
            }
            None => {
                self.buffer = Some(msg);
            }
        }
    }

    /// Finishes and returns hash.
    pub fn finalize(mut self) -> Felt {
        // Applies padding
        match self.buffer.take() {
            Some(last_message) => {
                self.state[0] += last_message;
                self.state[1] += Felt::ONE;
            }
            None => {
                self.state[0] += Felt::ONE;
            }
        }
        Poseidon::hades_permutation(&mut self.state);

        self.state[0]
    }
}

/// Computes the Starknet Poseidon hash of x and y.
pub fn poseidon_hash(x: Felt, y: Felt) -> Felt {
    let mut state = [x, y, Felt::TWO];
    Poseidon::hades_permutation(&mut state);

    state[0]
}

/// Computes the Starknet Poseidon hash of a single [`Felt`].
pub fn poseidon_hash_single(x: Felt) -> Felt {
    let mut state = [x, Felt::ZERO, Felt::ONE];
    Poseidon::hades_permutation(&mut state);

    state[0]
}

/// Computes the Starknet Poseidon hash of an arbitrary number of [`Felt`]s.
///
/// Using this function is the same as using [`PoseidonHasher`].
pub fn poseidon_hash_many<'a, I: IntoIterator<Item = &'a Felt>>(msgs: I) -> Felt {
    let mut state = [Felt::ZERO, Felt::ZERO, Felt::ZERO];
    let mut iter = msgs.into_iter();

    loop {
        match iter.next() {
            Some(v) => state[0] += *v,
            None => {
                state[0] += Felt::ONE;
                break;
            }
        }

        match iter.next() {
            Some(v) => state[1] += *v,
            None => {
                state[1] += Felt::ONE;
                break;
            }
        }

        Poseidon::hades_permutation(&mut state);
    }
    Poseidon::hades_permutation(&mut state);

    state[0]
}

/// Poseidon permutation function.                                                            
pub fn poseidon_permute_comp(state: &mut [Felt; 3]) {
    Poseidon::hades_permutation(state)
}

#[cfg(test)]
mod tests {
    use starknet_types_core::hash::StarkHash;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_poseidon_hash() {
        // Test data generated from `cairo-lang` v0.11.0
        let test_data = [
            (
                Felt::from_hex("0xb662f9017fa7956fd70e26129b1833e10ad000fd37b4d9f4e0ce6884b7bbe")
                    .unwrap(),
                Felt::from_hex("0x1fe356bf76102cdae1bfbdc173602ead228b12904c00dad9cf16e035468bea")
                    .unwrap(),
                Felt::from_hex("0x75540825a6ecc5dc7d7c2f5f868164182742227f1367d66c43ee51ec7937a81")
                    .unwrap(),
            ),
            (
                Felt::from_hex("0xf4e01b2032298f86b539e3d3ac05ced20d2ef275273f9325f8827717156529")
                    .unwrap(),
                Felt::from_hex("0x587bc46f5f58e0511b93c31134652a689d761a9e7f234f0f130c52e4679f3a")
                    .unwrap(),
                Felt::from_hex("0xbdb3180fdcfd6d6f172beb401af54dd71b6569e6061767234db2b777adf98b")
                    .unwrap(),
            ),
        ];

        for (x, y, hash) in test_data {
            assert_eq!(Poseidon::hash(&x, &y), hash);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_poseidon_hash_single() {
        // Test data generated from `cairo-lang` v0.11.0
        let test_data = [
            (
                Felt::from_hex("0x9dad5d6f502ccbcb6d34ede04f0337df3b98936aaf782f4cc07d147e3a4fd6")
                    .unwrap(),
                Felt::from_hex("0x11222854783f17f1c580ff64671bc3868de034c236f956216e8ed4ab7533455")
                    .unwrap(),
            ),
            (
                Felt::from_hex("0x3164a8e2181ff7b83391b4a86bc8967f145c38f10f35fc74e9359a0c78f7b6")
                    .unwrap(),
                Felt::from_hex("0x79ad7aa7b98d47705446fa01865942119026ac748d67a5840f06948bce2306b")
                    .unwrap(),
            ),
        ];

        for (x, hash) in test_data {
            let mut state = [x, Felt::ZERO, Felt::ONE];
            Poseidon::hades_permutation(&mut state);
            assert_eq!(state[0], hash);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_poseidon_hash_many() {
        // Test data generated from `cairo-lang` v0.11.0
        let test_data = [
            (
                vec![
                    Felt::from_hex(
                        "0x9bf52404586087391c5fbb42538692e7ca2149bac13c145ae4230a51a6fc47",
                    )
                    .unwrap(),
                    Felt::from_hex(
                        "0x40304159ee9d2d611120fbd7c7fb8020cc8f7a599bfa108e0e085222b862c0",
                    )
                    .unwrap(),
                    Felt::from_hex(
                        "0x46286e4f3c450761d960d6a151a9c0988f9e16f8a48d4c0a85817c009f806a",
                    )
                    .unwrap(),
                ],
                Felt::from_hex("0x1ec38b38dc88bac7b0ed6ff6326f975a06a59ac601b417745fd412a5d38e4f7")
                    .unwrap(),
            ),
            (
                vec![
                    Felt::from_hex(
                        "0xbdace8883922662601b2fd197bb660b081fcf383ede60725bd080d4b5f2fd3",
                    )
                    .unwrap(),
                    Felt::from_hex(
                        "0x1eb1daaf3fdad326b959dec70ced23649cdf8786537cee0c5758a1a4229097",
                    )
                    .unwrap(),
                    Felt::from_hex(
                        "0x869ca04071b779d6f940cdf33e62d51521e19223ab148ef571856ff3a44ff1",
                    )
                    .unwrap(),
                    Felt::from_hex(
                        "0x533e6df8d7c4b634b1f27035c8676a7439c635e1fea356484de7f0de677930",
                    )
                    .unwrap(),
                ],
                Felt::from_hex("0x2520b8f910174c3e650725baacad4efafaae7623c69a0b5513d75e500f36624")
                    .unwrap(),
            ),
        ];

        for (input, hash) in test_data {
            // Direct function call
            assert_eq!(Poseidon::hash_array(&input), hash);

            // With hasher
            let mut hasher = PoseidonHasher::new();
            input.iter().for_each(|msg| hasher.update(*msg));
            assert_eq!(hasher.finalize(), hash);
        }
    }
}
