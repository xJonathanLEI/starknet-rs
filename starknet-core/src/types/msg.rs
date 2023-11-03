use alloc::vec::Vec;

use sha3::{Digest, Keccak256};
use starknet_ff::FieldElement;

use super::EthAddress;

#[derive(Debug, Clone)]
pub struct MsgToL2 {
    pub from_address: EthAddress,
    pub to_address: FieldElement,
    pub selector: FieldElement,
    pub payload: Vec<FieldElement>,
    pub nonce: u64,
}

impl MsgToL2 {
    /// Calculates the message hash based on the algorithm documented here:
    ///
    /// https://docs.starknet.io/documentation/architecture_and_concepts/L1-L2_Communication/messaging-mechanism/
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();

        // FromAddress
        hasher.update([0u8; 12]);
        hasher.update(self.from_address.as_bytes());

        // ToAddress
        hasher.update(self.to_address.to_bytes_be());

        // Nonce
        hasher.update([0u8; 24]);
        hasher.update(self.nonce.to_be_bytes());

        // Selector
        hasher.update(self.selector.to_bytes_be());

        // Payload.length
        hasher.update([0u8; 24]);
        hasher.update((self.payload.len() as u64).to_be_bytes());

        // Payload
        for item in self.payload.iter() {
            hasher.update(item.to_bytes_be());
        }

        let hash = hasher.finalize();

        // Because we know hash is always 32 bytes
        unsafe { *(hash[..].as_ptr() as *const [u8; 32]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_msg_to_l2_hash() {
        // Goerli-1 tx: 0374286ae28f201e61ffbc5b022cc9701208640b405ea34ea9799f97d5d2d23c

        let msg = MsgToL2 {
            from_address: EthAddress::from_hex("0xc3511006C04EF1d78af4C8E0e74Ec18A6E64Ff9e")
                .unwrap(),
            to_address: FieldElement::from_hex_be(
                "0x73314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
            )
            .unwrap(),
            selector: FieldElement::from_hex_be(
                "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
            )
            .unwrap(),
            payload: vec![
                FieldElement::from_hex_be(
                    "0x689ead7d814e51ed93644bc145f0754839b8dcb340027ce0c30953f38f55d7",
                )
                .unwrap(),
                FieldElement::from_hex_be("0x2c68af0bb140000").unwrap(),
                FieldElement::from_hex_be("0x0").unwrap(),
            ],
            nonce: 775628,
        };

        let expected_hash: [u8; 32] =
            hex::decode("c51a543ef9563ad2545342b390b67edfcddf9886aa36846cf70382362fc5fab3")
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(msg.hash(), expected_hash);
    }
}
