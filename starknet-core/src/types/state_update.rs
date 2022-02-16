use super::{super::serde::unsigned_field_element::UfeHex, UnsignedFieldElement};

use serde::Deserialize;
use serde_with::serde_as;
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct StateUpdate {
    #[serde_as(as = "Option<UfeHex>")]
    pub block_hash: Option<UnsignedFieldElement>,
    #[serde_as(as = "UfeHex")]
    pub new_root: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
    pub old_root: UnsignedFieldElement,
    pub state_diff: StateDiff,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct StateDiff {
    #[serde_as(as = "HashMap<UfeHex, _>")]
    pub storage_diffs: HashMap<UnsignedFieldElement, Vec<StorageDiff>>,
    pub deployed_contracts: Vec<DeployedContract>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct StorageDiff {
    #[serde_as(as = "UfeHex")]
    pub key: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
    pub value: UnsignedFieldElement,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct DeployedContract {
    #[serde_as(as = "UfeHex")]
    pub address: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
    pub contract_hash: UnsignedFieldElement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_update_deser() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_state_update/1_success.txt");

        let state_update: StateUpdate = serde_json::from_str(raw).unwrap();

        let storage_diff = &state_update
            .state_diff
            .storage_diffs
            .get(
                &UnsignedFieldElement::from_hex_str(
                    "0x243b1e9ae747179e11ac685548ee1d6c5691ee9bda33ab0adee6f4838bddc55",
                )
                .unwrap(),
            )
            .unwrap()[0];

        assert_eq!(
            storage_diff.key,
            UnsignedFieldElement::from_hex_str(
                "0x37501df619c4fc4e96f6c0243f55e3abe7d1aca7db9af8f3740ba3696b3fdac"
            )
            .unwrap()
        );
        assert_eq!(
            storage_diff.value,
            UnsignedFieldElement::from_hex_str("0x1a").unwrap()
        );

        let deployed_contract = &state_update.state_diff.deployed_contracts[0];

        assert_eq!(
            deployed_contract.address,
            UnsignedFieldElement::from_hex_str(
                "0x7da57050effcee2a29d8ed3e3e42f9371bb827cbf96c1d2bcedbefd9004c72c"
            )
            .unwrap()
        );
        assert_eq!(
            deployed_contract.contract_hash,
            UnsignedFieldElement::from_hex_str(
                "02c3348ad109f7f3967df6494b3c48741d61675d9a7915b265aa7101a631dc33"
            )
            .unwrap()
        );
    }
}
