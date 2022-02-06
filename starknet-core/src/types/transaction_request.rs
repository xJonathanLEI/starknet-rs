use super::{
    super::serde::{
        byte_array::base64::serialize as base64_ser,
        unsigned_field_element::{hex, hex_option},
    },
    AbiEntry, UnsignedFieldElement,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddTransactionResult {
    pub code: AddTransactionResultCode,
    #[serde(with = "hex")]
    pub transaction_hash: UnsignedFieldElement,
    #[serde(default, with = "hex_option")]
    pub address: Option<UnsignedFieldElement>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum AddTransactionResultCode {
    #[serde(rename = "TRANSACTION_RECEIVED")]
    TransactionReceived,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionRequest {
    Deploy(DeployTransaction),
    InvokeFunction(InvokeFunctionTransaction),
}

#[derive(Debug, Serialize)]
pub struct DeployTransaction {
    pub constructor_calldata: Vec<UnsignedFieldElement>,
    #[serde(with = "hex")]
    pub contract_address_salt: UnsignedFieldElement,
    pub contract_definition: ContractDefinition,
}

#[derive(Debug, Serialize)]
pub struct InvokeFunctionTransaction {
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub entry_point_selector: UnsignedFieldElement,
    pub calldata: Vec<UnsignedFieldElement>,
    pub signature: Vec<UnsignedFieldElement>,
}

#[derive(Debug, Serialize)]
pub struct ContractDefinition {
    #[serde(serialize_with = "base64_ser")]
    pub program: Vec<u8>,
    pub entry_points_by_type: EntryPointsByType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Vec<AbiEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EntryPointsByType {
    pub constructor: Vec<EntryPoint>,
    pub external: Vec<EntryPoint>,
    pub l1_handler: Vec<EntryPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    #[serde(with = "hex")]
    pub selector: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub offset: UnsignedFieldElement,
}
