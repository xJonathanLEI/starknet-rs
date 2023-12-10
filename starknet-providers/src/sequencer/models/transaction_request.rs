use serde::{Deserialize, Serialize, Serializer};
use serde_with::serde_as;
use starknet_core::{
    serde::unsigned_field_element::{UfeHex, UfeHexOption},
    types::FieldElement,
};
use std::sync::Arc;

use super::{
    contract::{CompressedLegacyContractClass, CompressedSierraClass},
    serde_impls::u64_hex,
    transaction::{DataAvailabilityMode, ResourceBoundsMapping},
    L1Address,
};

/// 2 ^ 128 + 1
const QUERY_VERSION_ONE: FieldElement = FieldElement::from_mont([
    18446744073700081633,
    17407,
    18446744073709551584,
    576460752142433776,
]);

/// 2 ^ 128 + 2
const QUERY_VERSION_TWO: FieldElement = FieldElement::from_mont([
    18446744073700081601,
    17407,
    18446744073709551584,
    576460752142433232,
]);

/// 2 ^ 128 + 3
const QUERY_VERSION_THREE: FieldElement = FieldElement::from_mont([
    18446744073700081569,
    17407,
    18446744073709551584,
    576460752142432688,
]);

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum AddTransactionResultCode {
    #[serde(rename = "TRANSACTION_RECEIVED")]
    TransactionReceived,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionRequest {
    Declare(DeclareTransaction),
    InvokeFunction(InvokeFunctionTransaction),
    DeployAccount(DeployAccountTransaction),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DeclareTransaction {
    V1(DeclareV1Transaction),
    V2(DeclareV2Transaction),
    V3(DeclareV3Transaction),
}

#[derive(Debug)]
pub struct DeclareV1Transaction {
    pub contract_class: Arc<CompressedLegacyContractClass>,
    /// The address of the account contract sending the declaration transaction.
    pub sender_address: FieldElement,
    /// The maximal fee to be paid in Wei for declaring a contract class.
    pub max_fee: FieldElement,
    /// Additional information given by the caller that represents the signature of the transaction.
    pub signature: Vec<FieldElement>,
    /// A sequential integer used to distinguish between transactions and order them.
    pub nonce: FieldElement,
    pub is_query: bool,
}

#[derive(Debug)]
pub struct DeclareV2Transaction {
    pub contract_class: Arc<CompressedSierraClass>,
    /// Hash of the compiled class obtained by running `starknet-sierra-compile` on the Sierra
    /// class. This is required because at the moment, Sierra compilation is not proven, allowing
    /// the sequencer to run arbitrary code if this is not signed. It's expected that in the future
    /// this will no longer be required.
    pub compiled_class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction.
    pub sender_address: FieldElement,
    /// The maximal fee to be paid in Wei for declaring a contract class.
    pub max_fee: FieldElement,
    /// Additional information given by the caller that represents the signature of the transaction.
    pub signature: Vec<FieldElement>,
    /// A sequential integer used to distinguish between transactions and order them.
    pub nonce: FieldElement,
    pub is_query: bool,
}

#[derive(Debug)]
pub struct DeclareV3Transaction {
    pub contract_class: Arc<CompressedSierraClass>,
    /// Hash of the compiled class obtained by running `starknet-sierra-compile` on the Sierra
    /// class. This is required because at the moment, Sierra compilation is not proven, allowing
    /// the sequencer to run arbitrary code if this is not signed. It's expected that in the future
    /// this will no longer be required.
    pub compiled_class_hash: FieldElement,
    /// The address of the account contract sending the declaration transaction.
    pub sender_address: FieldElement,
    /// Additional information given by the caller that represents the signature of the transaction.
    pub signature: Vec<FieldElement>,
    /// A sequential integer used to distinguish between transactions and order them.
    pub nonce: FieldElement,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: u64,
    pub paymaster_data: Vec<FieldElement>,
    pub account_deployment_data: Vec<FieldElement>,
    pub is_query: bool,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InvokeFunctionTransaction {
    V1(InvokeFunctionV1Transaction),
    V3(InvokeFunctionV3Transaction),
}

#[derive(Debug)]
pub struct InvokeFunctionV1Transaction {
    pub sender_address: FieldElement,
    pub calldata: Vec<FieldElement>,
    pub signature: Vec<FieldElement>,
    pub max_fee: FieldElement,
    pub nonce: FieldElement,
    pub is_query: bool,
}

#[derive(Debug)]
pub struct InvokeFunctionV3Transaction {
    pub sender_address: FieldElement,
    pub calldata: Vec<FieldElement>,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: u64,
    pub paymaster_data: Vec<FieldElement>,
    pub account_deployment_data: Vec<FieldElement>,
    pub is_query: bool,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DeployAccountTransaction {
    V1(DeployAccountV1Transaction),
    V3(DeployAccountV3Transaction),
}

#[derive(Debug)]
pub struct DeployAccountV1Transaction {
    pub class_hash: FieldElement,
    pub contract_address_salt: FieldElement,
    pub constructor_calldata: Vec<FieldElement>,
    // The maximal fee to be paid in Wei for executing the transaction.
    pub max_fee: FieldElement,
    // The signature of the transaction.
    pub signature: Vec<FieldElement>,
    // The nonce of the transaction.
    pub nonce: FieldElement,
    pub is_query: bool,
}

#[derive(Debug)]
pub struct DeployAccountV3Transaction {
    pub class_hash: FieldElement,
    pub contract_address_salt: FieldElement,
    pub constructor_calldata: Vec<FieldElement>,
    // The signature of the transaction.
    pub signature: Vec<FieldElement>,
    // The nonce of the transaction.
    pub nonce: FieldElement,
    pub nonce_data_availability_mode: DataAvailabilityMode,
    pub fee_data_availability_mode: DataAvailabilityMode,
    pub resource_bounds: ResourceBoundsMapping,
    pub tip: u64,
    pub paymaster_data: Vec<FieldElement>,
    pub is_query: bool,
}

impl Serialize for DeclareV1Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            contract_class: &'a CompressedLegacyContractClass,
            #[serde_as(as = "UfeHex")]
            sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            max_fee: &'a FieldElement,
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_ONE
            } else {
                FieldElement::ONE
            },
            contract_class: &self.contract_class,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for DeclareV2Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            contract_class: &'a CompressedSierraClass,
            #[serde_as(as = "UfeHex")]
            compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            sender_address: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            max_fee: &'a FieldElement,
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_TWO
            } else {
                FieldElement::TWO
            },
            contract_class: &self.contract_class,
            compiled_class_hash: &self.compiled_class_hash,
            sender_address: &self.sender_address,
            max_fee: &self.max_fee,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for DeclareV3Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            contract_class: &'a CompressedSierraClass,
            #[serde_as(as = "UfeHex")]
            compiled_class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            sender_address: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
            nonce_data_availability_mode: &'a DataAvailabilityMode,
            fee_data_availability_mode: &'a DataAvailabilityMode,
            resource_bounds: &'a ResourceBoundsMapping,
            #[serde(with = "u64_hex")]
            tip: &'a u64,
            #[serde_as(as = "Vec<UfeHex>")]
            paymaster_data: &'a Vec<FieldElement>,
            #[serde_as(as = "Vec<UfeHex>")]
            account_deployment_data: &'a Vec<FieldElement>,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_THREE
            } else {
                FieldElement::THREE
            },
            contract_class: &self.contract_class,
            compiled_class_hash: &self.compiled_class_hash,
            sender_address: &self.sender_address,
            signature: &self.signature,
            nonce: &self.nonce,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for InvokeFunctionV1Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            #[serde_as(as = "UfeHex")]
            sender_address: &'a FieldElement,
            calldata: &'a Vec<FieldElement>,
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            max_fee: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_ONE
            } else {
                FieldElement::ONE
            },
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            signature: &self.signature,
            max_fee: &self.max_fee,
            nonce: &self.nonce,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for InvokeFunctionV3Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            #[serde_as(as = "UfeHex")]
            sender_address: &'a FieldElement,
            calldata: &'a Vec<FieldElement>,
            #[serde_as(as = "Vec<UfeHex>")]
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
            nonce_data_availability_mode: &'a DataAvailabilityMode,
            fee_data_availability_mode: &'a DataAvailabilityMode,
            resource_bounds: &'a ResourceBoundsMapping,
            #[serde(with = "u64_hex")]
            tip: &'a u64,
            #[serde_as(as = "Vec<UfeHex>")]
            paymaster_data: &'a Vec<FieldElement>,
            #[serde_as(as = "Vec<UfeHex>")]
            account_deployment_data: &'a Vec<FieldElement>,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_THREE
            } else {
                FieldElement::THREE
            },
            sender_address: &self.sender_address,
            calldata: &self.calldata,
            signature: &self.signature,
            nonce: &self.nonce,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
            account_deployment_data: &self.account_deployment_data,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for DeployAccountV1Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            #[serde_as(as = "UfeHex")]
            class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            contract_address_salt: &'a FieldElement,
            constructor_calldata: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            max_fee: &'a FieldElement,
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_ONE
            } else {
                FieldElement::ONE
            },
            class_hash: &self.class_hash,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            max_fee: &self.max_fee,
            signature: &self.signature,
            nonce: &self.nonce,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

impl Serialize for DeployAccountV3Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[serde_as]
        #[derive(Serialize)]
        struct Versioned<'a> {
            #[serde_as(as = "UfeHex")]
            version: FieldElement,
            #[serde_as(as = "UfeHex")]
            class_hash: &'a FieldElement,
            #[serde_as(as = "UfeHex")]
            contract_address_salt: &'a FieldElement,
            #[serde_as(as = "Vec<UfeHex>")]
            constructor_calldata: &'a Vec<FieldElement>,
            #[serde_as(as = "Vec<UfeHex>")]
            signature: &'a Vec<FieldElement>,
            #[serde_as(as = "UfeHex")]
            nonce: &'a FieldElement,
            nonce_data_availability_mode: &'a DataAvailabilityMode,
            fee_data_availability_mode: &'a DataAvailabilityMode,
            resource_bounds: &'a ResourceBoundsMapping,
            #[serde(with = "u64_hex")]
            tip: &'a u64,
            #[serde_as(as = "Vec<UfeHex>")]
            paymaster_data: &'a Vec<FieldElement>,
        }

        let versioned = Versioned {
            version: if self.is_query {
                QUERY_VERSION_THREE
            } else {
                FieldElement::THREE
            },
            class_hash: &self.class_hash,
            contract_address_salt: &self.contract_address_salt,
            constructor_calldata: &self.constructor_calldata,
            signature: &self.signature,
            nonce: &self.nonce,
            nonce_data_availability_mode: &self.nonce_data_availability_mode,
            fee_data_availability_mode: &self.fee_data_availability_mode,
            resource_bounds: &self.resource_bounds,
            tip: &self.tip,
            paymaster_data: &self.paymaster_data,
        };

        Versioned::serialize(&versioned, serializer)
    }
}

fn l1_addr_as_dec<S>(value: &L1Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut buffer = [0u8; 32];
    buffer[12..].copy_from_slice(&value.0);

    // Unwrapping is safe here as it's never out of range
    let addr_in_felt = FieldElement::from_bytes_be(&buffer).unwrap();

    serializer.serialize_str(&addr_in_felt.to_string())
}
