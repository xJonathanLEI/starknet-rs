use super::super::serde::{deserialize_h256_from_hex, deserialize_vec_u256_from_dec};

use ethereum_types::{H256, U256};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Transaction {
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransaction),
    #[serde(rename = "INVOKE_FUNCTION")]
    InvokeFunction(InvokeFunctionTransaction),
}

#[derive(Debug, Deserialize)]
pub enum EntryPointType {
    #[serde(rename = "EXTERNAL")]
    External,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct DeployTransaction {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    transaction_hash: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    contract_address: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    contract_address_salt: H256,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    constructor_calldata: Vec<U256>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct InvokeFunctionTransaction {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    transaction_hash: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    contract_address: H256,
    entry_point_type: EntryPointType,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    entry_point_selector: H256,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    calldata: Vec<U256>,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    signature: Vec<U256>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {}
