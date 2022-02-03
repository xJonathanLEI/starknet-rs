use super::{
    UnsignedFieldElement,
    {
        super::serde::unsigned_field_element::{
            hex, hex_option, pending_block_hash::deserialize as pending_block_hash_de,
        },
        TransactionStatus,
    },
};

use serde::Deserialize;

pub enum TransactionId {
    Hash(UnsignedFieldElement),
    Number(u64),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Deploy(DeployTransaction),
    InvokeFunction(InvokeFunctionTransaction),
}

#[derive(Debug, Deserialize)]
pub enum Transaction {
    Brief(BriefTransaction),
    Full(FullTransaction),
}

#[derive(Debug, Deserialize)]
pub struct BriefTransaction {
    #[serde(default, with = "hex_option")]
    pub block_hash: Option<UnsignedFieldElement>,
    #[serde(alias = "tx_status")]
    pub status: TransactionStatus,
}

#[derive(Debug, Deserialize)]
pub struct FullTransaction {
    pub block_number: Option<u64>,
    pub transaction: Option<TransactionType>,
    pub status: TransactionStatus,
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
    pub constructor_calldata: Vec<UnsignedFieldElement>,
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub contract_address_salt: UnsignedFieldElement,
    #[serde(with = "hex")]
    pub transaction_hash: UnsignedFieldElement,
}

#[derive(Debug, Deserialize)]
pub struct InvokeFunctionTransaction {
    #[serde(with = "hex")]
    pub contract_address: UnsignedFieldElement,
    pub entry_point_type: EntryPointType,
    #[serde(with = "hex")]
    pub entry_point_selector: UnsignedFieldElement,
    pub calldata: Vec<UnsignedFieldElement>,
    pub signature: Vec<UnsignedFieldElement>,
    #[serde(with = "hex")]
    pub transaction_hash: UnsignedFieldElement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_full_invoke_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/1_invoke.txt");
        let tx: FullTransaction = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39099));
        if let TransactionType::InvokeFunction(invoke) = tx.transaction.unwrap() {
            assert_eq!(invoke.signature.len(), 2);
        } else {
            panic!("Did not deserialize TransactionType::InvokeFunction properly")
        }
    }

    #[test]
    fn test_deser_full_deploy_transaction() {
        let raw =
            include_str!("../../test-data/raw_gateway_responses/get_transaction/2_deploy.txt");
        let tx: FullTransaction = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, Some(39181));
        if let TransactionType::Deploy(deploy) = tx.transaction.unwrap() {
            assert_eq!(deploy.constructor_calldata.len(), 2)
        } else {
            panic!("Did not deserialize TransactionType::Deploy properly");
        }
    }

    #[test]
    fn test_deser_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction/3_not_received.txt"
        );
        let tx: FullTransaction = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.block_number, None);
        assert_eq!(tx.status, TransactionStatus::NotReceived);
    }

    #[test]
    fn test_deser_brief_accepted() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/1_accepted.txt"
        );

        let tx: BriefTransaction = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::AcceptedOnL1);
        assert_eq!(
            tx.block_hash,
            Some(
                UnsignedFieldElement::from_hex_str(
                    "0xca6e3e44d58747b398a0b4e882245c6bc9f5cd666674824e14929708fb8d09"
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn test_deser_brief_not_received() {
        let raw = include_str!(
            "../../test-data/raw_gateway_responses/get_transaction_status/2_not_received.txt"
        );

        let tx: BriefTransaction = serde_json::from_str(raw).unwrap();

        assert_eq!(tx.status, TransactionStatus::NotReceived);
        assert!(tx.block_hash.is_none());
    }
}
