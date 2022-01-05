use super::super::serde::serialize_vec_u256_into_dec;

use ethereum_types::{H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct InvokeFunction {
    pub contract_address: H256,
    pub entry_point_selector: H256,
    #[serde(serialize_with = "serialize_vec_u256_into_dec")]
    pub calldata: Vec<U256>,
    #[serde(serialize_with = "serialize_vec_u256_into_dec")]
    pub signature: Vec<U256>,
}

#[derive(Debug, Deserialize)]
pub struct CallContractResult {
    pub result: Vec<U256>,
}
