use super::{
    super::serde::{
        deserialize_h256_from_hex, deserialize_option_h256_from_hex, serialize_vec_u256_into_dec,
        serialize_vec_u8_into_base64,
    },
    AbiEntry,
};

use ethereum_types::{H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum TransactionRequest {
    #[serde(rename = "DEPLOY")]
    Deploy(DeployTransaction),
    #[serde(rename = "INVOKE_FUNCTION")]
    InvokeFunction(InvokeFunctionTransaction),
}

#[derive(Debug, Deserialize)]
pub struct AddTransactionResult {
    pub code: AddTransactionResultCode,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub transaction_hash: H256,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_h256_from_hex")]
    pub address: Option<H256>,
}

#[derive(Debug, Deserialize)]
pub enum AddTransactionResultCode {
    #[serde(rename = "TRANSACTION_RECEIVED")]
    TransactionReceived,
}

#[derive(Debug, Serialize)]
pub struct DeployTransaction {
    pub contract_address_salt: H256,
    pub contract_definition: ContractDefinition,
    #[serde(serialize_with = "serialize_vec_u256_into_dec")]
    pub constructor_calldata: Vec<U256>,
}

#[derive(Debug, Serialize)]
pub struct InvokeFunctionTransaction {
    pub contract_address: H256,
    pub entry_point_selector: H256,
    #[serde(serialize_with = "serialize_vec_u256_into_dec")]
    pub calldata: Vec<U256>,
    #[serde(serialize_with = "serialize_vec_u256_into_dec")]
    pub signature: Vec<U256>,
}

#[derive(Debug, Serialize)]
pub struct ContractDefinition {
    #[serde(serialize_with = "serialize_vec_u8_into_base64")]
    pub program: Vec<u8>,
    pub entry_points_by_type: EntryPointsByType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<AbiEntry>>,
}

#[derive(Debug, Serialize)]
pub struct EntryPointsByType {
    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Vec<EntryPoint>,
    #[serde(rename = "EXTERNAL")]
    pub external: Vec<EntryPoint>,
    #[serde(rename = "L1_HANDLER")]
    pub l1_handler: Vec<EntryPoint>,
}

#[derive(Debug, Serialize)]
pub struct EntryPoint {
    pub selector: H256,
    pub offset: U256,
}
