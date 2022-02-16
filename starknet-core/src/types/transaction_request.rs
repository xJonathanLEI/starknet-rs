use super::{
    super::serde::{
        byte_array::base64::serialize as base64_ser,
        unsigned_field_element::{UfeHex, UfeHexOption},
    },
    AbiEntry, UnsignedFieldElement,
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct AddTransactionResult {
    pub code: AddTransactionResultCode,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: UnsignedFieldElement,
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
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

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DeployTransaction {
    pub constructor_calldata: Vec<UnsignedFieldElement>,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: UnsignedFieldElement,
    pub contract_definition: ContractDefinition,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct InvokeFunctionTransaction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    #[serde_as(as = "UfeHex")]
    pub selector: UnsignedFieldElement,
    #[serde_as(as = "UfeHex")]
    pub offset: UnsignedFieldElement,
}
