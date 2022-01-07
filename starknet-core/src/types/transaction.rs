use super::{
    super::serde::{
        deserialize_h256_from_hex, deserialize_pending_block_hash, deserialize_vec_u256_from_dec,
    },
    TransactionStatusType,
};

use ethereum_types::{H256, U256};
use serde::Deserialize;

pub enum TransactionId {
    Hash(H256),
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
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_pending_block_hash")]
    pub block_hash: Option<H256>,
    pub transaction_index: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum EntryPointType {
    #[serde(rename = "EXTERNAL")]
    External,
}

#[derive(Debug, Deserialize)]
pub struct DeployTransaction {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub transaction_hash: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub contract_address: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub contract_address_salt: H256,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    pub constructor_calldata: Vec<U256>,
}

#[derive(Debug, Deserialize)]
pub struct InvokeFunctionTransaction {
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub transaction_hash: H256,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub contract_address: H256,
    pub entry_point_type: EntryPointType,
    #[serde(deserialize_with = "deserialize_h256_from_hex")]
    pub entry_point_selector: H256,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    pub calldata: Vec<U256>,
    #[serde(deserialize_with = "deserialize_vec_u256_from_dec")]
    pub signature: Vec<U256>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_with_status_deser() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/1_invoke.txt");
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39099));
        if let Transaction::InvokeFunction(invoke) = tx.transaction.unwrap() {
            assert_eq!(invoke.signature.len(), 2);
        } else {
            panic!("Did not deserialize Transaction::InvokeFunction properly")
        }

        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/2_deploy.txt");
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39181));
        if let Transaction::Deploy(deploy) = tx.transaction.unwrap() {
            assert_eq!(deploy.constructor_calldata.len(), 2)
        } else {
            panic!("Did not deserialize Transaction::Deploy properly");
        }

        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction/3_not_received.txt"
        );
        let tx: TransactionWithStatus = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, None);
        assert_eq!(tx.status, TransactionStatusType::NotReceived);
    }
}
