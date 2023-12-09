use serde::Deserialize;
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
    #[serde_as(as = "Option<UfeHex>")]
    pub block_hash: Option<FieldElement>,
    #[serde_as(as = "Option<UfeHex>")]
    pub new_root: Option<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub old_root: FieldElement,
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    #[serde_as(as = "HashMap<UfeHex, _>")]
    pub storage_diffs: HashMap<FieldElement, Vec<StorageDiff>>,
    pub deployed_contracts: Vec<DeployedContract>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub old_declared_contracts: Vec<FieldElement>,
    pub declared_classes: Vec<DeclaredContract>,
    #[serde(default)]
    #[serde_as(as = "HashMap<UfeHex, UfeHex>")]
    pub nonces: HashMap<FieldElement, FieldElement>,
    pub replaced_classes: Vec<DeployedContract>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageDiff {
    #[serde_as(as = "UfeHex")]
    pub key: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub value: FieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContract {
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclaredContract {
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub compiled_class_hash: FieldElement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser() {
        let raw =
            include_str!("../../../test-data/raw_gateway_responses/get_state_update/1_success.txt");

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();

        let storage_diff = &state_update
            .state_diff
            .storage_diffs
            .get(
                &FieldElement::from_hex_be(
                    "0xdc2e5d3d73589a12037d1cdf1ba3f69bde2e8983faa0a5c6b3b051b2c46e14",
                )
                .unwrap(),
            )
            .unwrap()[0];

        assert_eq!(
            storage_diff.key,
            FieldElement::from_hex_be(
                "0x23444ef42446d7a7ebaaceea3dedfa11c3306fa839f98611e5efcd38ea59350"
            )
            .unwrap()
        );
        assert_eq!(
            storage_diff.value,
            FieldElement::from_hex_be("0x7c7").unwrap()
        );

        let deployed_contract = &state_update.state_diff.deployed_contracts[0];

        assert_eq!(
            deployed_contract.address,
            FieldElement::from_hex_be(
                "0xa251264114855c3d59281ad5a912730fbba38dddbcce7abce115440db7868f"
            )
            .unwrap()
        );
        assert_eq!(
            deployed_contract.class_hash,
            FieldElement::from_hex_be(
                "048498ebae1afc22157322db4bb7814b668c7ee20237cc8be64d934649679da1"
            )
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_pending_state_update_deser() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/2_pending_block.txt"
        );

        serde_json::from_str::<StateUpdate>(raw).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_nonce_changes() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/4_with_nonce_changes.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.nonces.len(), 1);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_declare_v2() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/5_with_declare_v2.txt"
        );
        serde_json::from_str::<StateUpdate>(raw).unwrap();
    }

    #[test]
    #[ignore = "block with the same criteria not found in goerli-integration yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_replaced_classes() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/6_with_replaced_classes.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.replaced_classes.len(), 1);
    }
}
