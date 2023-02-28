// Code ported from the build.rs script here:
//   https://github.com/eqlabs/pathfinder/blob/00a1a74a90a7b8a7f1d07ac3e616be1cb39cf8f1/crates/stark_poseidon/build.rs

use std::fmt::Write;

use proc_macro::TokenStream;
use starknet_ff::FieldElement;

mod params;

const FULL_ROUNDS: usize = 8;
const PARTIAL_ROUNDS: usize = 83;

pub fn poseidon_consts() -> TokenStream {
    let round_keys = params::RAW_ROUND_KEYS
        .iter()
        .map(|key| key.map(|num| FieldElement::from_dec_str(num).expect("Invalid round key")))
        .collect::<Vec<_>>();

    let flat = round_keys.iter().flatten().cloned().collect::<Vec<_>>();
    let comp = compress_roundkeys(&round_keys);

    let mut buffer = String::new();

    writeln!(buffer, "const FULL_ROUNDS: usize = {FULL_ROUNDS};").unwrap();
    writeln!(buffer, "const PARTIAL_ROUNDS: usize = {PARTIAL_ROUNDS};").unwrap();

    writeln!(buffer).unwrap();

    writeln!(buffer, "{}", generate_code("POSEIDON_CONSTS", &flat)).unwrap();
    writeln!(buffer).unwrap();

    writeln!(buffer, "{}", generate_code("POSEIDON_COMP_CONSTS", &comp)).unwrap();

    buffer.parse().expect("Invalid code generated")
}

pub fn compress_roundkeys(rcs: &[[FieldElement; 3]]) -> Vec<FieldElement> {
    let mut result = Vec::new();

    // Add first full rounds
    result.extend(rcs[..FULL_ROUNDS / 2].iter().flatten());

    // Add compressed partial rounds and first of the last full rounds
    result.extend(compress_roundkeys_partial(rcs));

    // Add last full rounds except the first of them
    result.extend(
        rcs[(FULL_ROUNDS / 2 + PARTIAL_ROUNDS + 1)..]
            .iter()
            .flatten(),
    );

    result
}

pub fn compress_roundkeys_partial(rcs: &[[FieldElement; 3]]) -> Vec<FieldElement> {
    let mut result = Vec::new();

    let mut idx = FULL_ROUNDS / 2;
    let mut state: [FieldElement; 3] = [FieldElement::ZERO; 3];

    // Add keys for partial rounds
    for _ in 0..PARTIAL_ROUNDS {
        // AddRoundKey
        state[0] += rcs[idx][0];
        state[1] += rcs[idx][1];
        state[2] += rcs[idx][2];

        // Add last state
        result.push(state[2]);

        // Reset last state
        state[2] = FieldElement::ZERO;

        // MixLayer
        let t = state[0] + state[1] + state[2];
        state[0] = t + FieldElement::TWO * state[0];
        state[1] = t - FieldElement::TWO * state[1];
        state[2] = t - FieldElement::THREE * state[2];

        idx += 1;
    }

    // Add keys for first of the last full rounds
    state[0] += rcs[idx][0];
    state[1] += rcs[idx][1];
    state[2] += rcs[idx][2];
    result.push(state[0]);
    result.push(state[1]);
    result.push(state[2]);

    result
}

pub fn generate_code(name: &str, rcs: &[FieldElement]) -> String {
    let mut buf = String::with_capacity(1024 * 1024);

    writeln!(buf, "pub const {}: [FieldElement; {}] = [", name, rcs.len()).unwrap();

    rcs.iter().for_each(|num| {
        writeln!(
            buf,
            "FieldElement::from_mont([{}]),",
            num.into_mont().map(|ele| format!("{ele}")).join(",")
        )
        .unwrap();
    });

    writeln!(buf, "];").unwrap();
    buf
}
