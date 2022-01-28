use super::{
    UnsignedFieldElement,
    {
        super::serde::unsigned_field_element::{
            hex, pending_block_hash::deserialize as pending_block_hash_de,
        },
        TransactionStatusType,
    },
};

use serde::Deserialize;

pub enum TransactionId {
    Hash(UnsignedFieldElement),
    Number(u64),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Transaction {
    Deploy(DeployTransaction),
    InvokeFunction(InvokeFunctionTransaction),
}

#[derive(Debug, Deserialize)]
pub struct TransactionWithStatus {
    pub block_number: Option<u64>,
    pub transaction: Option<Transaction>,
    pub status: TransactionStatusType,
    #[serde(default, deserialize_with = "pending_block_hash_de")]
    pub block_hash: Option<UnsignedFieldElement>,
    pub transaction_index: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum EntryPointType {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "L1_HANDLER")]
    L1Handler,
}

#[derive(Debug, Deserialize)]
pub struct DeployTransaction {
    #[serde(with = "hex")]
    pub transaction_hash: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub contract_address_salt: UnsignedFieldElement,
    pub constructor_calldata: Vec<UnsignedFieldElement>,
}

#[derive(Debug, Deserialize)]
pub struct InvokeFunctionTransaction {
    #[serde(with = "hex")]
    pub transaction_hash: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    pub entry_point_type: EntryPointType,
    #[serde(with = "hex")]
    pub entry_point_selector: UnsignedFieldElement,
    pub calldata: Vec<UnsignedFieldElement>,
    pub signature: Vec<UnsignedFieldElement>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_with_status_deser_invoke() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/1_invoke.txt");
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39099));
        if let Transaction::InvokeFunction(invoke) = tx.transaction.unwrap() {
            assert_eq!(invoke.signature.len(), 2);
        } else {
            panic!("Did not deserialize Transaction::InvokeFunction properly")
        }
    }

    #[test]
    fn test_transaction_with_status_deser_deploy() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/2_deploy.txt");
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39181));
        if let Transaction::Deploy(deploy) = tx.transaction.unwrap() {
            assert_eq!(deploy.constructor_calldata.len(), 2)
        } else {
            panic!("Did not deserialize Transaction::Deploy properly");
        }
    }

    #[test]
    fn test_transaction_with_status_deser_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction/3_not_received.txt"
        );
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, None);
        assert_eq!(tx.status, TransactionStatusType::NotReceived);
    }
}
