#[cfg(not(feature = "pedersen_no_lookup"))]
mod default;
#[cfg(not(feature = "pedersen_no_lookup"))]
pub use default::pedersen_hash;

#[cfg(feature = "pedersen_no_lookup")]
mod no_lookup;
#[cfg(feature = "pedersen_no_lookup")]
pub use no_lookup::pedersen_hash;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::field_element_from_be_hex;

    // Test case ported from:
    //   https://github.com/starkware-libs/starkex-for-spot-trading/blob/607f0b4ce507e1d95cd018d206a2797f6ba4aab4/src/starkware/crypto/starkware/crypto/signature/test/config/signature_test_data.json

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_pedersen_hash() {
        let test_data = [
            (
                "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
                "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a",
                "030e480bed5fe53fa909cc0f8c4d99b8f9f2c016be4c41e13a4848797979c662",
            ),
            (
                "058f580910a6ca59b28927c08fe6c43e2e303ca384badc365795fc645d479d45",
                "078734f65a067be9bdb39de18434d71e79f7b6466a4b66bbd979ab9e7515fe0b",
                "068cc0b76cddd1dd4ed2301ada9b7c872b23875d5ff837b3a87993e0d9996b87",
            ),
        ];

        for (in1, in2, expected_hash) in test_data {
            let in1 = field_element_from_be_hex(in1);
            let in2 = field_element_from_be_hex(in2);
            let expected_hash = field_element_from_be_hex(expected_hash);

            assert_eq!(pedersen_hash(&in1, &in2), expected_hash);
        }
    }
}
