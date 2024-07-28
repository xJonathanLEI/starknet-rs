use starknet_types_core::felt::Felt;

/// The chain identifier for Starknet Mainnet. A Cairo short string encoding of `SN_MAIN`.
pub const MAINNET: Felt = Felt::from_raw([
    502562008147966918,
    18446744073709551615,
    18446744073709551615,
    17696389056366564951,
]);

/// The chain identifier for Starknet Goerli. A Cairo short string encoding of `SN_GOERLI`.
#[deprecated = "The Goerli testnet has been shutdown"]
pub const TESTNET: Felt = Felt::from_raw([
    398700013197595345,
    18446744073709551615,
    18446744073709548950,
    3753493103916128178,
]);

/// The chain identifier for Starknet Goerli 2. A Cairo short string encoding of `SN_GOERLI2`.
#[deprecated = "The Goerli testnet has been shutdown"]
pub const TESTNET2: Felt = Felt::from_raw([
    33650220878420990,
    18446744073709551615,
    18446744073708869172,
    1663542769632127759,
]);

/// The chain identifier for Starknet Sepolia. A Cairo short string encoding of `SN_SEPOLIA`.
pub const SEPOLIA: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);

#[cfg(test)]
mod test {
    use crate::utils::cairo_short_string_to_felt;

    use super::*;

    #[test]
    #[allow(deprecated)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_chain_ids() {
        for (text, felt) in [
            ("SN_MAIN", MAINNET),
            ("SN_GOERLI", TESTNET),
            ("SN_GOERLI2", TESTNET2),
            ("SN_SEPOLIA", SEPOLIA),
        ] {
            assert_eq!(cairo_short_string_to_felt(text).unwrap(), felt);
        }
    }
}
