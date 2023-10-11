//! CairoType implementation for starknet types.
//!
//! They are alf `FieldElement` under the hood.
use crate::abi::cairo_types::{CairoType, Result};
use starknet_core::types::FieldElement;

/// ContractAddress.
#[derive(Debug, PartialEq)]
pub struct ContractAddress(pub FieldElement);

impl From<FieldElement> for ContractAddress {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl From<ContractAddress> for FieldElement {
    fn from(item: ContractAddress) -> Self {
        item.0
    }
}

impl CairoType for ContractAddress {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(ContractAddress(FieldElement::deserialize(felts, offset)?))
    }
}

/// ClassHash.
#[derive(Debug, PartialEq)]
pub struct ClassHash(pub FieldElement);

impl From<FieldElement> for ClassHash {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl From<ClassHash> for FieldElement {
    fn from(item: ClassHash) -> Self {
        item.0
    }
}

impl CairoType for ClassHash {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(ClassHash(FieldElement::deserialize(felts, offset)?))
    }
}

/// EthAddress.
#[derive(Debug, PartialEq)]
pub struct EthAddress(pub FieldElement);

impl From<FieldElement> for EthAddress {
    fn from(item: FieldElement) -> Self {
        Self(item)
    }
}

impl From<EthAddress> for FieldElement {
    fn from(item: EthAddress) -> Self {
        item.0
    }
}

impl CairoType for EthAddress {
    type RustType = Self;

    fn serialize(rust: &Self::RustType) -> Vec<FieldElement> {
        FieldElement::serialize(&rust.0)
    }

    fn deserialize(felts: &[FieldElement], offset: usize) -> Result<Self::RustType> {
        Ok(EthAddress(FieldElement::deserialize(felts, offset)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_address_serialize() {
        let contract_address = ContractAddress(FieldElement::from(1_u32));
        let felts = ContractAddress::serialize(&contract_address);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(1_u32));
    }

    #[test]
    fn test_contract_address_deserialize() {
        let felts = vec![FieldElement::from(1_u32)];
        let contract_address = ContractAddress::deserialize(&felts, 0).unwrap();
        assert_eq!(contract_address, ContractAddress(FieldElement::from(1_u32)))
    }

    #[test]
    fn test_class_hash_serialize() {
        let class_hash = ClassHash(FieldElement::from(1_u32));
        let felts = ClassHash::serialize(&class_hash);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(1_u32));
    }

    #[test]
    fn test_class_hash_deserialize() {
        let felts = vec![FieldElement::from(1_u32)];
        let class_hash = ClassHash::deserialize(&felts, 0).unwrap();
        assert_eq!(class_hash, ClassHash(FieldElement::from(1_u32)))
    }

    #[test]
    fn test_eth_address_serialize() {
        let eth_address = EthAddress(FieldElement::from(1_u32));
        let felts = EthAddress::serialize(&eth_address);
        assert_eq!(felts.len(), 1);
        assert_eq!(felts[0], FieldElement::from(1_u32));
    }

    #[test]
    fn test_eth_address_deserialize() {
        let felts = vec![FieldElement::from(1_u32)];
        let eth_address = EthAddress::deserialize(&felts, 0).unwrap();
        assert_eq!(eth_address, EthAddress(FieldElement::from(1_u32)))
    }

    #[test]
    fn test_contract_address_from() {
        let contract_address = ContractAddress::from(FieldElement::from(1_u32));
        assert_eq!(contract_address, ContractAddress(FieldElement::from(1_u32)))
    }

    #[test]
    fn test_class_hash_from() {
        let class_hash = ClassHash::from(FieldElement::from(1_u32));
        assert_eq!(class_hash, ClassHash(FieldElement::from(1_u32)))
    }

    #[test]
    fn test_eth_address_from() {
        let eth_address = EthAddress::from(FieldElement::from(1_u32));
        assert_eq!(eth_address, EthAddress(FieldElement::from(1_u32)))
    }
}
