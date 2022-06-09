use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::{serde::unsigned_field_element::UfeHex, types::FieldElement};

/// Function call information
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    /// The parameters passed to the function
    #[serde_as(as = "Vec<UfeHex>")]
    pub calldata: Vec<FieldElement>,
}

/// Block hash or tag
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockHashOrTag {
    Hash(#[serde_as(as = "UfeHex")] FieldElement),
    Tag(BlockTag),
}

/// A tag specifying a dynamic reference to a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    // The current spec doesn't allow `pending` but is probably inaccurate:
    //   https://github.com/starkware-libs/starknet-specs/blob/bcce12075ef4cde19fc62b47ed2162292e0ed70d/api/starknet_api_openrpc.json#L821-L828
    #[serde(rename = "pending")]
    Pending,
}
