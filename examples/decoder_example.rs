use starknet::{
    core::types::FieldElement,
    providers::{Provider, SequencerGatewayProvider},
};
use starknet_core::{
    decoder::{decode, Address, Decode, ParamType, Token},
    types::{TransactionType, ValueOutOfRangeError},
};

use crypto_bigint::U256;
use starknet_macros::Decode;

#[derive(Debug)]
struct Uint {
    low: U256,
    high: U256,
}

#[derive(Debug, Decode)]
struct Transfer {
    from: Address,
    to: Address,
    amount: Uint,
}

impl TryFrom<&Token> for Uint {
    type Error = ValueOutOfRangeError;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if let Token::Tuple(v) = value {
            Ok(Uint {
                low: U256::from(&v[0]),
                high: U256::from(&v[1]),
            })
        } else {
            Err(ValueOutOfRangeError)
        }
    }
}

impl TryFrom<Vec<Token>> for Transfer {
    type Error = ValueOutOfRangeError;

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        let from = Address::try_from(&value[0])?;
        let to = Address::try_from(&value[1])?;
        let amount = Uint::try_from(&value[2])?;
        Ok(Self { from, to, amount })
    }
}

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let tx_hash = "0x03a4ce1bb249ed3f8b190012dc7ca0ab2caff155d1b81727aebf2bb7ee12b04b";
    let tx_hash = FieldElement::from_hex_be(tx_hash).unwrap();
    let tx = provider.get_transaction(tx_hash).await.unwrap();
    println!("tx: {tx:?}");

    let tx_type = tx.r#type.unwrap();

    let types = [
        ParamType::FieldElement,
        ParamType::FieldElement,
        ParamType::Tuple(2),
    ];
    if let TransactionType::L1Handler(tx) = tx_type {
        let decoded = decode(&types, &tx.calldata).unwrap();
        println!("decoded: {decoded:?}");

        let transfer = Transfer::decode(&decoded);
        println!("transfer: {transfer:?}");
    }
}
