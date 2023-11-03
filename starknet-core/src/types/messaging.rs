use alloc::vec::Vec;

use num_bigint::BigUint;
use sha3::{Digest, Keccak256};
use starknet_ff::FieldElement;

use super::{eth_address::EthAddress, L1HandlerTransaction, MsgToL1};
use crate::utils::biguint_to_felts;

pub struct MsgToL2 {
    from_address: EthAddress,
    to_address: FieldElement,
    nonce: FieldElement,
    selector: FieldElement,
    payload: Vec<FieldElement>,
}

#[allow(clippy::vec_init_then_push)]
impl MsgToL2 {
    /// Returns the hash as a vector a FieldElements
    /// representing a serialized u256: [low, high].
    pub fn hash_as_felts(&self) -> Vec<FieldElement> {
        // TODO: is it safe to unwrap as keccak returns a valid u256?
        biguint_to_felts(self.hash()).unwrap()
    }

    /// Computes MsgToL2 hash.
    pub fn hash(&self) -> BigUint {
        let mut buf: Vec<u8> = Vec::new();

        // As EthAddress is only 20 bytes, we do want the 32 bytes
        // to compute the hash as done in solidity `uint256(uint160(fromAddress))`.
        let from_felt: FieldElement = self.from_address.clone().into();

        buf.extend(from_felt.to_bytes_be());
        buf.extend(self.to_address.to_bytes_be());
        buf.extend(self.nonce.to_bytes_be());
        buf.extend(self.selector.to_bytes_be());
        buf.extend(FieldElement::from(self.payload.len()).to_bytes_be());
        for p in &self.payload {
            buf.extend(p.to_bytes_be());
        }

        let mut hasher = Keccak256::new();
        hasher.update(buf);
        let hash = hasher.finalize();
        BigUint::from_bytes_be(hash.as_slice())
    }
}

impl L1HandlerTransaction {
    /// Parses and returns the `MsgToL2` from
    /// the transaction's content.
    pub fn parse_msg_to_l2(&self) -> MsgToL2 {
        // TODO: is that necessary? As the sequencer
        // itself is the one firing this kind of transaction?
        assert!(!self.calldata.is_empty());

        // Ok to unwrap as the sequencer already checks for address ranges
        // even if the `from_address` in `l1_handler` is still `felt252` type in cairo.
        let from_address = self.calldata[0].try_into().unwrap();
        let to_address = self.contract_address;
        let selector = self.entry_point_selector;
        let nonce = (self.nonce as u128).into();
        let payload = &self.calldata[1..];

        MsgToL2 {
            from_address,
            to_address,
            selector,
            nonce,
            payload: payload.to_vec(),
        }
    }
}

impl MsgToL1 {
    pub fn hash_as_felts(&self) -> Vec<FieldElement> {
        // TODO: is it safe to unwrap as keccak returns a valid u256?
        biguint_to_felts(self.hash()).unwrap()
    }

    /// Computes MsgToL1 hash.
    pub fn hash(&self) -> BigUint {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(self.from_address.to_bytes_be());
        buf.extend(self.to_address.to_bytes_be());
        buf.extend(FieldElement::from(self.payload.len()).to_bytes_be());

        for p in &self.payload {
            buf.extend(p.to_bytes_be());
        }

        let mut hasher = Keccak256::new();
        hasher.update(buf);
        let hash = hasher.finalize();
        BigUint::from_bytes_be(hash.as_slice())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use num_traits::Num;

    use super::*;
    use crate::types::EthAddress;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[allow(clippy::vec_init_then_push)]
    fn test_msg_to_l2_hash() {
        let mut payload = Vec::new();
        payload.push(FieldElement::ONE);
        payload.push(FieldElement::TWO);

        // Tx used for this test on goerli:
        // 0x46144b600db00853c57a8cf003030ffaa51a810758ef5bfe1bb42bf55b7af38.
        let msg = MsgToL2 {
            from_address: EthAddress::from_hex("0xbe3c44c09bc1a3566f3e1ca12e5aba0fa4ca72be")
                .unwrap(),
            to_address: FieldElement::from_hex_be(
                "0x039dc79e64f4bb3289240f88e0bae7d21735bef0d1a51b2bf3c4730cb16983e1",
            )
            .unwrap(),
            selector: FieldElement::from_hex_be(
                "0x02f15cff7b0eed8b9beb162696cf4e3e0e35fa7032af69cd1b7d2ac67a13f40f",
            )
            .unwrap(),
            payload,
            nonce: 782870_u128.into(),
        };

        assert!(
            msg.hash()
                == BigUint::from_str_radix(
                    "7d56f59fe6cce2fd0620a4b1cce69c488acac84670c38053ffec3763a2eec09d",
                    16
                )
                .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[allow(clippy::vec_init_then_push)]
    fn test_msg_to_l2_handler_tx() {
        // Tx used for this test on goerli:
        // 0x46144b600db00853c57a8cf003030ffaa51a810758ef5bfe1bb42bf55b7af38.
        let mut calldata = Vec::new();
        calldata
            .push(FieldElement::from_hex_be("0xbe3c44c09bc1a3566f3e1ca12e5aba0fa4ca72be").unwrap());
        calldata.push(FieldElement::ONE);
        calldata.push(FieldElement::TWO);

        let tx = L1HandlerTransaction {
            transaction_hash: FieldElement::from_hex_be(
                "0x46144b600db00853c57a8cf003030ffaa51a810758ef5bfe1bb42bf55b7af38",
            )
            .unwrap(),
            version: 0x0,
            nonce: 0xbf216,
            contract_address: FieldElement::from_hex_be(
                "0x39dc79e64f4bb3289240f88e0bae7d21735bef0d1a51b2bf3c4730cb16983e1",
            )
            .unwrap(),
            entry_point_selector: FieldElement::from_hex_be(
                "0x2f15cff7b0eed8b9beb162696cf4e3e0e35fa7032af69cd1b7d2ac67a13f40f",
            )
            .unwrap(),
            calldata,
        };

        let msg = tx.parse_msg_to_l2();

        assert!(
            msg.hash()
                == BigUint::from_str_radix(
                    "7d56f59fe6cce2fd0620a4b1cce69c488acac84670c38053ffec3763a2eec09d",
                    16
                )
                .unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[allow(clippy::vec_init_then_push)]
    fn test_msg_to_l1_hash() {
        // Tx used for this test on goerli:
        // 0x34a700d53a8aac7fb241d2b3b696d87e5bd1bb291104bb3bc6cd3e3ca0482f5
        let mut payload = Vec::new();
        payload.push(FieldElement::ONE);
        payload.push(FieldElement::TWO);

        let msg = MsgToL1 {
            from_address: FieldElement::from_hex_be(
                "0x39dc79e64f4bb3289240f88e0bae7d21735bef0d1a51b2bf3c4730cb16983e1",
            )
            .unwrap(),
            to_address: FieldElement::from_hex_be("0xbe3c44c09bc1a3566f3e1ca12e5aba0fa4ca72be")
                .unwrap(),
            payload,
        };

        assert!(
            msg.hash()
                == BigUint::from_str_radix(
                    "27dd75eb3d94688853676deda3b2cf14d2ef1074f81e1f5712d7c3946b9ab727",
                    16
                )
                .unwrap()
        );
    }
}
