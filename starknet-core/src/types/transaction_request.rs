use super::{
    super::serde::{
        byte_array::base64::serialize as base64_ser,
        unsigned_field_element::{UfeHex, UfeHexOption},
    },
    AbiEntry, FieldElement,
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AddTransactionResult {
    pub code: AddTransactionResultCode,
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: FieldElement,
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
    pub address: Option<FieldElement>,
    #[serde(default)]
    #[serde_as(as = "UfeHexOption")]
    pub class_hash: Option<FieldElement>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum AddTransactionResultCode {
    #[serde(rename = "TRANSACTION_RECEIVED")]
    TransactionReceived,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionRequest {
    Declare(DeclareTransaction),
    Deploy(DeployTransaction),
    InvokeFunction(InvokeFunctionTransaction),
}

/// Represents a transaction in the StarkNet network that is originated from an action of an
/// account.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountTransaction {
    Declare(DeclareTransaction),
    InvokeFunction(InvokeFunctionTransaction),
}

/// Represents a contract function call in the StarkNet network.
#[serde_as]
#[derive(Debug, Serialize)]
pub struct CallFunction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    pub calldata: Vec<FieldElement>,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DeclareTransaction {
    pub contract_class: ContractDefinition,
    /// The address of the account contract sending the declaration transaction.
    #[serde_as(as = "UfeHex")]
    pub sender_address: FieldElement,
    /// The maximal fee to be paid in Wei for declaring a contract class.
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
    /// Additional information given by the caller that represents the signature of the transaction.
    pub signature: Vec<FieldElement>,
    /// A sequential integer used to distinguish between transactions and order them.
    #[serde_as(as = "UfeHex")]
    pub nonce: FieldElement,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DeployTransaction {
    pub constructor_calldata: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub contract_address_salt: FieldElement,
    pub contract_definition: ContractDefinition,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct InvokeFunctionTransaction {
    #[serde_as(as = "UfeHex")]
    pub contract_address: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub entry_point_selector: FieldElement,
    pub calldata: Vec<FieldElement>,
    pub signature: Vec<FieldElement>,
    #[serde_as(as = "UfeHex")]
    pub max_fee: FieldElement,
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
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct EntryPointsByType {
    pub constructor: Vec<EntryPoint>,
    pub external: Vec<EntryPoint>,
    pub l1_handler: Vec<EntryPoint>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct EntryPoint {
    #[serde_as(as = "UfeHex")]
    pub selector: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub offset: FieldElement,
}
