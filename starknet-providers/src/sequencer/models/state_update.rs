use serde::Deserialize;
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateUpdate {
    #[serde_as(as = "Option<UfeHex>")]
    pub block_hash: Option<Felt>,
    #[serde_as(as = "Option<UfeHex>")]
    pub new_root: Option<Felt>,
    #[serde_as(as = "UfeHex")]
    pub old_root: Felt,
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StateDiff {
    #[serde_as(as = "HashMap<UfeHex, _>")]
    pub storage_diffs: HashMap<Felt, Vec<StorageDiff>>,
    pub deployed_contracts: Vec<DeployedContract>,
    #[serde_as(as = "Vec<UfeHex>")]
    pub old_declared_contracts: Vec<Felt>,
    pub declared_classes: Vec<DeclaredContract>,
    #[serde(default)]
    #[serde_as(as = "HashMap<UfeHex, UfeHex>")]
    pub nonces: HashMap<Felt, Felt>,
    pub replaced_classes: Vec<DeployedContract>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct StorageDiff {
    #[serde_as(as = "UfeHex")]
    pub key: Felt,
    #[serde_as(as = "UfeHex")]
    pub value: Felt,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeployedContract {
    #[serde_as(as = "UfeHex")]
    pub address: Felt,
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DeclaredContract {
    #[serde_as(as = "UfeHex")]
    pub class_hash: Felt,
    #[serde_as(as = "UfeHex")]
    pub compiled_class_hash: Felt,
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
                &Felt::from_hex(
                    "0x197b9913e67947b0605934ec72db497d341a0199282c1da6d4aae46b17e0e76",
                )
                .unwrap(),
            )
            .unwrap()[0];

        assert_eq!(
            storage_diff.key,
            Felt::from_hex("0x34c0d833897dbc937dd35e9f49a5184aecd6ab47829de999f9587549f82d0e")
                .unwrap()
        );
        assert_eq!(
            storage_diff.value,
            Felt::from_hex("0x74fd8ca6e84097ad9ae00febfb690ad032fc4477155ec2193382c1b30b5e12b")
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
        assert_eq!(state_update.state_diff.nonces.len(), 2);
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
    #[ignore = "block with the same criteria not found in alpha-sepolia yet"]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_replaced_classes() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/6_with_replaced_classes.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.replaced_classes.len(), 1);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_state_update_deser_with_deployed_contracts() {
        let raw = include_str!(
            "../../../test-data/raw_gateway_responses/get_state_update/7_with_deployed_contracts.txt"
        );

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();
        assert_eq!(state_update.state_diff.deployed_contracts.len(), 5);
    }
}
