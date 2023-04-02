use super::{super::serde::unsigned_field_element::UfeHex, FieldElement};

use serde::Deserialize;
use serde_with::serde_as;
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
    #[serde_as(as = "Option<UfeHex>")]
    pub block_hash: Option<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub new_root: FieldElement,
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
            include_str!("../../test-data/raw_gateway_responses/get_state_update/1_success.txt");

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();

        let storage_diff = &state_update
            .state_diff
            .storage_diffs
            .get(
                &FieldElement::from_hex_be(
                    "0x243b1e9ae747179e11ac685548ee1d6c5691ee9bda33ab0adee6f4838bddc55",
                )
                .unwrap(),
            )
            .unwrap()[0];

        assert_eq!(
            storage_diff.key,
            FieldElement::from_hex_be(
                "0x37501df619c4fc4e96f6c0243f55e3abe7d1aca7db9af8f3740ba3696b3fdac"
            )
            .unwrap()
        );
        assert_eq!(
            storage_diff.value,
            FieldElement::from_hex_be("0x1a").unwrap()
        );

        let deployed_contract = &state_update.state_diff.deployed_contracts[0];

        assert_eq!(
            deployed_contract.address,
            FieldElement::from_hex_be(
                "0x7da57050effcee2a29d8ed3e3e42f9371bb827cbf96c1d2bcedbefd9004c72c"
            )
            .unwrap()
        );
        assert_eq!(
            deployed_contract.class_hash,
            FieldElement::from_hex_be(
                "02c3348ad109f7f3967df6494b3c48741d61675d9a7915b265aa7101a631dc33"
            )
            .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_nonce_changes() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_state_update/4_with_nonce_changes.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.nonces.len(), 1);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_declare_v2() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_state_update/5_with_declare_v2.txt"
        );
        serde_json::from_str::<StateUpdate>(raw).unwrap();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_replaced_classes() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_state_update/6_with_replaced_classes.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.replaced_classes.len(), 1);
    }
}
