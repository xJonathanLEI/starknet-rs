use crate::types::FieldElement;

pub const MAINNET: FieldElement = FieldElement::from_mont([
    17696389056366564951,
    18446744073709551615,
    18446744073709551615,
    502562008147966918,
]);

pub const TESTNET: FieldElement = FieldElement::from_mont([
    3753493103916128178,
    18446744073709548950,
    18446744073709551615,
    398700013197595345,
]);

#[cfg(test)]
mod test {
    use crate::utils::cairo_short_string_to_felt;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_chain_ids() {
        for (text, felt) in [("SN_MAIN", MAINNET), ("SN_GOERLI", TESTNET)].into_iter() {
            assert_eq!(cairo_short_string_to_felt(text).unwrap(), felt);
        }
    }
}
